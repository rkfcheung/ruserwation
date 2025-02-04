use serde_json::json as to_json;
use std::sync::Arc;
use warp::{
    http::StatusCode,
    reject::Rejection,
    reply::{json, with_status, Json, Reply, WithStatus},
    Filter,
};

use super::{
    helper::{validate_ref_check, validate_reservation},
    models::{Reservation, ReservationRequest, ReservationResponse},
    repo::ReservationRepo,
};
use crate::{
    config::{context::with_context, models::Context},
    db::QueryError,
    utils::env_util::{var_as_int_or, var_as_str_or},
};

pub fn reserve_route(
    context: Arc<Context<impl ReservationRepo + Send + Sync>>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let create_reservation = warp::post()
        .and(warp::path!("reservations" / "reserve"))
        .and(warp::header::exact("Content-Type", "application/json"))
        .and(warp::body::json())
        .and(with_context(context.clone()))
        .and_then(handle_reserve);

    let update_reservation = warp::put()
        .and(warp::path!("reservations" / "update" / String))
        .and(warp::header::exact("Content-Type", "application/json"))
        .and(warp::body::json())
        .and(with_context(context))
        .and_then(handle_update);

    create_reservation.or(update_reservation)
}

async fn handle_update(
    book_ref: String,
    body: ReservationRequest,
    context: Arc<Context<impl ReservationRepo + Send + Sync>>,
) -> Result<impl Reply, Rejection> {
    if let Some(req_book_ref) = body.book_ref() {
        if &book_ref != req_book_ref {
            log::error!(
                "Failed to process reservation book_ref={book_ref} due to: requested ref [{}] is different",
                req_book_ref
            );

            return reservation_error("The reservation request is invalid.");
        }
    } else {
        log::error!(
            "Failed to process reservation book_ref={book_ref} due to: missing requested ref",
        );

        return reservation_error("The reservation request is invalid.");
    }

    handle_reserve(body, context).await
}

async fn handle_reserve(
    body: ReservationRequest,
    context: Arc<Context<impl ReservationRepo + Send + Sync>>,
) -> Result<WithStatus<Json>, Rejection> {
    // Extract environment variables
    let secret = var_as_str_or("RW_RSV_SECRET", "ChangeMe");
    let expired = var_as_int_or("RW_RSV_EXPIRED", 3_600) as i64;

    // Validate ref_check
    let ref_check = body.ref_check();
    if let Err(err) = validate_ref_check(ref_check, &secret, expired) {
        log::error!("Failed to process reservation ref_check={ref_check} due to: {err}");

        return reservation_error("The reservation request is either invalid or has expired.");
    }

    // Convert to Reservation
    let has_book_ref = body.has_book_ref();
    let mut reservation: Reservation = body.into();

    // Validate reservation details
    if let Err(err) = validate_reservation(&reservation) {
        log::error!("Failed to validate reservation due to: {err}");

        return reservation_error(&err);
    }

    if has_book_ref {
        // Handle for updating reservation
        let repo = context.get();
        let book_ref = &reservation.book_ref;
        let result = repo.find_by_book_ref(book_ref).await;
        match result {
            Some(saved) => {
                if saved.customer_email != reservation.customer_email {
                    log::error!(
                        "Failed to match reservation book_ref={book_ref} with customer_email: saved[{}] vs requested[{}]", 
                        saved.customer_email, reservation.customer_email
                    );

                    return handle_failure_with_status(
                        Some(book_ref.to_string()),
                        QueryError::InvalidQuery("Reservation Details Not Match".to_string()),
                        StatusCode::BAD_REQUEST,
                    );
                }

                reservation.id = saved.id;
                reservation.assigned_table = saved.assigned_table;
            }
            None => {
                log::error!("Failed to find reservation book_ref={book_ref}");

                return handle_failure_with_status(
                    Some(book_ref.to_string()),
                    QueryError::NotFound("Invalid Book Ref".to_string()),
                    StatusCode::NOT_FOUND,
                );
            }
        }
    }

    // Save reservation
    match save_reservation(&mut reservation, context).await {
        Ok(id) => handle_success(id, &reservation),
        Err(err) => handle_failure(&reservation, err),
    }
}

// Helper function for error responses
fn reservation_error(message: &str) -> Result<WithStatus<Json>, Rejection> {
    handle_failure_with_status(None, message, StatusCode::BAD_REQUEST)
}

// Helper function to save the reservation
async fn save_reservation(
    reservation: &mut Reservation,
    context: Arc<Context<impl ReservationRepo + Send + Sync>>,
) -> Result<u32, QueryError> {
    context.get().save(reservation).await
}

// Helper function for success response
fn handle_success(id: u32, reservation: &Reservation) -> Result<WithStatus<Json>, Rejection> {
    log::info!(
        "Reservation id={} with book_ref={} saved",
        id,
        reservation.book_ref
    );
    let response = json(&to_json!(ReservationResponse::ok(&reservation.book_ref)));

    Ok(with_status(response, StatusCode::OK))
}

// Helper function for failure response
fn handle_failure(
    reservation: &Reservation,
    err: QueryError,
) -> Result<WithStatus<Json>, Rejection> {
    log::error!(
        "Failed to save reservation book_ref={} due to: {}",
        reservation.book_ref,
        err
    );
    handle_failure_with_status(None, err, StatusCode::INTERNAL_SERVER_ERROR)
}

fn handle_failure_with_status<E: ToString>(
    book_ref: Option<String>,
    err: E,
    status: StatusCode,
) -> Result<WithStatus<Json>, Rejection> {
    let response = json(&to_json!(match book_ref {
        Some(book_ref) => ReservationResponse::err_with_book_ref(&book_ref, &err.to_string()),
        None => ReservationResponse::err(&err.to_string()),
    }));

    Ok(with_status(response, status))
}

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
    warp::post()
        .and(warp::path!("reservations" / "reserve"))
        .and(warp::header::exact("Content-Type", "application/json"))
        .and(warp::body::json())
        .and(with_context(context.clone()))
        .and_then(handle_reserve)
}

async fn handle_reserve(
    body: ReservationRequest,
    context: Arc<Context<impl ReservationRepo + Send + Sync>>,
) -> Result<impl Reply, Rejection> {
    // Extract environment variables
    let secret = var_as_str_or("RW_RSV_SECRET", "ChangeMe");
    let expired = var_as_int_or("RW_RSV_EXPIRED", 3_600) as i64;

    // Validate ref_check
    if let Err(err) = validate_ref_check(body.ref_check(), &secret, expired) {
        log_ref_check_error(body.ref_check(), &err);
        return reservation_error("The reservation request is either invalid or has expired.");
    }

    // Convert to Reservation
    let mut reservation: Reservation = body.into();

    // Validate reservation details
    if let Err(err) = validate_reservation(&reservation) {
        return reservation_error(&err);
    }

    // Save reservation
    match save_reservation(&mut reservation, context).await {
        Ok(id) => handle_success(id, &reservation),
        Err(err) => handle_failure(&reservation, err),
    }
}

// Helper function to handle ref_check errors
fn log_ref_check_error(ref_check: &str, err: &str) {
    log::error!(
        "Failed to process reservation ref_check={} due to: {}",
        ref_check,
        err
    );
}

// Helper function for error responses
fn reservation_error(message: &str) -> Result<WithStatus<Json>, Rejection> {
    let response = json(&to_json!(ReservationResponse::err(message)));
    Ok(with_status(response, StatusCode::BAD_REQUEST))
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
    let response = json(&to_json!(ReservationResponse::err(&err.to_string())));
    Ok(with_status(response, StatusCode::INTERNAL_SERVER_ERROR))
}

use serde_json::json as to_json;
use std::sync::Arc;
use warp::{
    http::StatusCode,
    reject::Rejection,
    reply::Reply,
    reply::{json, with_status},
    Filter,
};

use super::{
    helper::{validate_ref_check, validate_reservation},
    models::{Reservation, ReservationRequest, ReservationResponse},
    repo::ReservationRepo,
};
use crate::{
    config::{context::with_context, models::Context},
    utils::env_util::{var_as_int_or, var_as_str_or},
};

pub fn reserve(
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
    let secret = var_as_str_or("RW_RSV_SECRET", "ChangeMe");
    let expired = var_as_int_or("RW_RSV_EXPIRED", 3_600) as i64;

    // Validate ref_check
    if let Err(err) = validate_ref_check(body.ref_check(), &secret, expired) {
        log::error!(
            "Failed to process reservation ref_check={} due to: {}",
            body.ref_check(),
            err
        );

        return Err(warp::reject::custom(ReservationResponse::err(
            "Invalid Request or expired",
        )));
    }

    // Convert to Reservation
    let mut reservation: Reservation = body.into();

    // Validate reservation details
    if let Err(err) = validate_reservation(&reservation) {
        return Err(warp::reject::custom(ReservationResponse::err(&err)));
    }

    // Save reservation to the repository
    match context.get().save(&mut reservation).await {
        Ok(id) => {
            log::info!(
                "Reservation id={id} with book_ref={} saved",
                reservation.book_ref
            );

            // Construct success response
            let response = json(&to_json!(ReservationResponse::ok(&reservation.book_ref)));
            Ok(with_status(response, StatusCode::OK))
        }
        Err(err) => {
            log::error!(
                "Failed to save reservation book_ref={} due to: {}",
                reservation.book_ref,
                err
            );

            // Construct error response
            let response = json(&to_json!(ReservationResponse::err(&err.to_string())));
            Ok(with_status(response, StatusCode::INTERNAL_SERVER_ERROR))
        }
    }
}

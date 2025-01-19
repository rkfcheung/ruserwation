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
    helper::validate_reservation,
    models::{Reservation, ReservationRequest, ReservationResponse},
    repo::ReservationRepo,
};
use crate::config::{context::with_context, models::Context};

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
    let mut reservation: Reservation = body.into();
    let validated = validate_reservation(&reservation);
    match validated {
        Ok(_) => {
            let saved = context.get().save(&mut reservation).await;
            match saved {
                Ok(id) => {
                    log::info!(
                        "Reservation id={id} with book_ref={} saved",
                        reservation.book_ref
                    );
                    let response = json(&to_json!(ReservationResponse::ok(&reservation.book_ref)));
                    Ok(with_status(response, StatusCode::OK))
                }
                Err(err) => {
                    let response = json(&to_json!(ReservationResponse::err(&err.to_string())));
                    Ok(with_status(response, StatusCode::INTERNAL_SERVER_ERROR))
                }
            }
        }
        Err(err) => {
            let response = json(&to_json!(ReservationResponse::err(&err)));
            Ok(with_status(response, StatusCode::BAD_REQUEST))
        }
    }
}

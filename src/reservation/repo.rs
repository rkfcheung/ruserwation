use chrono::NaiveDateTime;
use std::future::Future;

use super::models::{Reservation, ReservationQuery, ReservationStatus};
use crate::common::Repo;

pub trait ReservationRepo: Repo<u32, Reservation> {
    fn find_all_by_query(
        &self,
        query: ReservationQuery,
    ) -> impl Future<Output = Vec<Reservation>> + Send;

    fn find_one_by_query(
        &self,
        query: ReservationQuery,
    ) -> impl Future<Output = Option<Reservation>> + Send;

    fn find_by_book_ref(&self, book_ref: &str) -> impl Future<Output = Option<Reservation>> + Send {
        self.find_one_by_query(ReservationQuery::default().book_ref(book_ref))
    }

    fn find_by_status(
        &self,
        status: ReservationStatus,
    ) -> impl Future<Output = Vec<Reservation>> + Send {
        self.find_all_by_query(ReservationQuery::default().status(status.clone()))
    }

    fn find_by_time(
        &self,
        from_time: NaiveDateTime,
        to_time: NaiveDateTime,
    ) -> impl Future<Output = Vec<Reservation>> + Send {
        self.find_all_by_query(
            ReservationQuery::default()
                .start_time(from_time)
                .end_time(to_time),
        )
    }
}

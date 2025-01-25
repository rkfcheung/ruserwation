use mock_derive::{mock_captured_arguments, mock_invoked};
use mocks::InvocationTracker;
use ruserwation::{
    common::Repo,
    db::QueryError,
    reservation::{
        models::{Reservation, ReservationQuery},
        repo::ReservationRepo,
    },
};

#[cfg(test)]
#[allow(dead_code)]
#[derive(Default)]
pub(crate) struct MockReservationRepo {
    pub(crate) invocation: InvocationTracker,
}

impl Repo<u32, Reservation> for MockReservationRepo {
    async fn find_by_id(&self, _id: u32) -> Option<Reservation> {
        unimplemented!()
    }

    #[mock_captured_arguments]
    #[mock_invoked]
    async fn save(&self, entity: &mut Reservation) -> Result<u32, QueryError> {
        if entity.customer_email.contains("save_failure") {
            Err(QueryError::SqlxError("Failed to save reservation".into()))
        } else {
            Ok(42)
        }
    }
}

impl ReservationRepo for MockReservationRepo {
    async fn find_all_by_query(&self, _query: ReservationQuery) -> Vec<Reservation> {
        unimplemented!()
    }

    async fn find_one_by_query(&self, _query: ReservationQuery) -> Option<Reservation> {
        unimplemented!()
    }
}

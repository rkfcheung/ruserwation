use std::future::Future;

use crate::db::QueryError;

pub trait Repo<Id, Entity> {
    // Find an Entity by Id
    fn find_by_id(&self, id: Id) -> impl Future<Output = Option<Entity>> + Send;

    // Save an Entity and return its Id
    fn save(&self, entity: &mut Entity) -> impl Future<Output = Result<Id, QueryError>> + Send;
}

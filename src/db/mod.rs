pub mod sqlite;

pub enum OpType {
    Insert,
    Update,
    NoOp,
}

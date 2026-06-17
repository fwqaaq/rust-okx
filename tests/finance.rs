//! Finance API integration tests grouped by read-only versus asset-changing calls.

mod common;

#[path = "finance/read_only.rs"]
mod read_only;
#[path = "finance/todo.rs"]
mod todo;

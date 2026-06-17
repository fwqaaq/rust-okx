//! Trade API integration tests grouped by read-only versus order-changing calls.

mod common;

#[path = "trade/read_only.rs"]
mod read_only;
#[path = "trade/todo.rs"]
mod todo;

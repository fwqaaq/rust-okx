//! Authenticated Trading Account integration tests, grouped by safety level.

mod common;

#[path = "account/loans.rs"]
mod loans;
#[path = "account/read_only.rs"]
mod read_only;
#[path = "account/todo.rs"]
mod todo;

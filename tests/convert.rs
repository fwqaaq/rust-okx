//! Convert API integration tests, separated into read-only and mutating cases.

mod common;

#[path = "convert/read_only.rs"]
mod read_only;
#[path = "convert/todo.rs"]
mod todo;

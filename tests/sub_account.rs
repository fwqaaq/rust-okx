//! Sub-account API integration tests, separated into read-only and mutating cases.

mod common;

#[path = "sub_account/read_only.rs"]
mod read_only;
#[path = "sub_account/todo.rs"]
mod todo;

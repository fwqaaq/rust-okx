//! Funding / Asset API integration tests grouped by safety level.

mod common;

#[path = "funding/read_only.rs"]
mod read_only;
#[path = "funding/todo.rs"]
mod todo;

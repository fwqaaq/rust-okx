//! Public reference-data integration tests.

mod common;

#[path = "public_data/core.rs"]
mod core;
#[path = "public_data/history.rs"]
mod history;
#[path = "public_data/options_and_quotas.rs"]
mod options_and_quotas;
#[path = "public_data/todo.rs"]
mod todo;

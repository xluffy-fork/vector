//! Limit the max number of requests being concurrently processed.

mod controller;
mod future;
mod service;

pub(crate) use controller::IsBackPressure;
pub(crate) use service::AutoConcurrencyLimit;

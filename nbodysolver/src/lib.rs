pub mod lorenz;
pub mod n_body_core;
pub mod n_body_wrapper;

#[cfg(feature = "plot")]
pub mod plotting;

pub use crate::n_body_wrapper::*;

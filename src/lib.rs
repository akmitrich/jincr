pub mod db;
mod error;
pub mod log;
pub mod op;
pub mod utils;

pub use error::{Error, Result};
pub use op::Op;

pub trait DbBackend {}

#![warn(clippy::all)]

pub use keyboard_usage_id::KeyboardUsageId;
use thiserror::Error;

mod keyboard_usage_id {
    include!(concat!(env!("OUT_DIR"), "/gen.rs"));
}

#[derive(Error, Debug)]
#[error("no mapping found {0:?}")]
pub struct NoMappingFound(btknmle_input::KeyCodes);

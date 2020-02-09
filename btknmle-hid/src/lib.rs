#![warn(clippy::all)]

pub use keyboard_usage_id::KeyboardUsageId;

mod keyboard_usage_id {
    include!(concat!(env!("OUT_DIR"), "/gen.rs"));
}

#[derive(Debug, failure::Fail)]
#[fail(display = "no mapping found {:?}", _0)]
pub struct NoMappingFound(btknmle_input::KeyCodes);

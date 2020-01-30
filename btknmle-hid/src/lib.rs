use std::fmt;

pub use keyboard_usage_id::KeyboardUsageId;

mod keyboard_usage_id {
    include!(concat!(env!("OUT_DIR"), "/gen.rs"));
}

#[derive(Debug)]
pub struct NoMappingFound(btknmle_input::KeyCodes);

impl fmt::Display for NoMappingFound {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "no mapping found {:?}", self.0)
    }
}

impl std::error::Error for NoMappingFound {}

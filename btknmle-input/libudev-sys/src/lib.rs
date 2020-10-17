#[lazylink::lazylink("udev", include = "libudev-sys/src/lib.rs")]
mod sys {}

pub use sys::*;

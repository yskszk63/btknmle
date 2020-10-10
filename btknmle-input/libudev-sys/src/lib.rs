#[lazylink::lazylink("udev", include="src/libudev-sys.rs")]
mod sys {
}

pub use sys::*;

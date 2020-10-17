#![allow(non_camel_case_types, non_upper_case_globals)]

extern crate libc;

#[cfg(feature="libinput_1_15")]
#[lazylink::lazylink("input", include="input.rs/input-sys/src/gen_1_15.rs")]
mod gen {
}

#[cfg(all(feature="libinput_1_14", not(feature="libinput_1_15")))]
#[lazylink::lazylink("input", include="input.rs/input-sys/src/gen_1_14.rs")]
mod gen {
}

#[cfg(all(feature="libinput_1_11", not(any(feature="libinput_1_14", feature="libinput_1_15"))))]
#[lazylink::lazylink("input", include="input.rs/input-sys/src/gen_1_11.rs")]
mod gen {
}

#[cfg(all(not(any(feature="libinput_1_11", feature="libinput_1_14", feature="libinput_1_15"))))]
#[lazylink::lazylink("input", include="input.rs/input-sys/src/gen_1_9.rs")]
mod gen {
}

pub use gen::*;

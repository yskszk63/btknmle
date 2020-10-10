#![allow(non_camel_case_types, non_upper_case_globals)]

extern crate libc;

#[lazylink::lazylink("input", include_outdir="gen.rs")]
mod gen {
}

pub use gen::*;

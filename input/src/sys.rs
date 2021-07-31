#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(clippy::all)]

#[cfg(not(bindgen))]
#[path = "bindgen/linux_input.rs"]
pub(crate) mod linux_input;

#[cfg(not(bindgen))]
#[path = "bindgen/linux_input_event_codes.rs"]
pub(crate) mod linux_input_event_codes;

#[cfg(bindgen)]
pub(crate) mod linux_input {
    include!(concat!(env!("OUT_DIR"), "/linux_input.rs"));
}

#[cfg(bindgen)]
pub(crate) mod linux_input_event_codes {
    include!(concat!(env!("OUT_DIR"), "/linux_input_event_codes.rs"));
}

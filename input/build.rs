#[cfg(bindgen)]
use std::env;
#[cfg(bindgen)]
use std::path::PathBuf;

#[cfg(not(bindgen))]
fn main() {}

#[cfg(bindgen)]
fn main() {
    println!("cargo:rerun-if-changed=src/linux_input.h");
    println!("cargo:rerun-if-changed=src/linux_input_event_codes.h");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindgen::Builder::default()
        .header("src/linux_input.h")
        .whitelist_var("_EVIOCGRAB")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(out_path.join("linux_input.rs"))
        .expect("Couldn't write bindings!");

    bindgen::Builder::default()
        .header("src/linux_input_event_codes.h")
        .whitelist_var("KEY_.*")
        .whitelist_var("BTN_.*")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(out_path.join("linux_input_event_codes.rs"))
        .expect("Couldn't write bindings!");
}

use std::{env, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let libs = system_deps::Config::new().probe().unwrap();
    let headers = libs.get("srt").unwrap().include_paths.clone();

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let mut bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        .size_t_is_usize(true)
        .whitelist_function("srt_.*")
        .whitelist_type("SRT.*")
        .whitelist_var("SRT.*")
        .bitfield_enum("SRT_EPOLL_OPT")
        .default_enum_style(bindgen::EnumVariation::NewType { is_bitfield: false })
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks));

    for header in headers {
        bindings = bindings.clang_arg("-I").clang_arg(header.to_str().unwrap());
    }

    let bindings = bindings
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
    Ok(())
}

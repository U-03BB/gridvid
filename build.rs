fn main() {
    cc::Build::new()
        .file("ffi/wrapper.c")
        .warnings(false) // minimp4.h warnings not in scope
        .compile("minimp4");

    let bindings = bindgen::Builder::default()
        .header("ffi/wrapper.h")
        .generate()
        .expect("Couldn't generate bindings");

    let out_path = std::path::PathBuf::from(std::env::var_os("OUT_DIR").unwrap());

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings");

    println!("cargo:rerun-if-changed=ffi/minimp4.h");
    println!("cargo:rerun-if-changed=ffi/wrapper.c");
    println!("cargo:rerun-if-changed=ffi/wrapper.h");
}

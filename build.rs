use std::path::PathBuf;

use cmake::Config;

fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let out_path = PathBuf::from(&out_dir);
    let bindings_path = PathBuf::from(&out_dir).join("bindings.rs");

    let harfbuzz_dst = Config::new("harfbuzz")
        .define("BUILD_SHARED_LIBS", "FALSE")
        .define("HAVE_FREETYPE", "TRUE")
        .build();

    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed=src/wrapper.h");

    println!(
        "cargo::rustc-link-search=native={}/lib",
        harfbuzz_dst.display()
    );

    println!("cargo::rustc-link-lib=static=harfbuzz");

    let binding = bindgen::builder()
        .allowlist_item("^hb_.*$")
        .allowlist_item("^HB_.*$")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .clang_arg(format!("-I{}/include/harfbuzz", harfbuzz_dst.display()))
        .header("src/wrapper.h")
        .generate()
        .unwrap();

    binding.write_to_file(bindings_path).unwrap();
}

use std::env;
use std::path::PathBuf;

fn main() {
	println!("cargo:rerun-if-changed=build.rs");

	println!(
		"cargo:rustc-link-search=all=./natives/{}",
		if cfg!(target_arch = "x86_64") {
			"x86_64"
		} else if cfg!(target_arch = "aarch64") {
			"aarch64"
		} else {
			panic!("Unsupported platform")
		}
	);

	println!("cargo:rustc-link-lib=dylib=ultimate_alpr-sdk");

    if cfg!(not(target_arch = "aarch64")) {
        println!("cargo:rustc-link-lib=dylib=tensorflow");
    }

	println!("cargo:rustc-link-lib=dylib=tensorflow_framework");

	println!("cargo:rerun-if-changed=wrapper.hpp");
	println!("cargo:rustc-link-arg=-std=c++14");

	let bindings = bindgen::Builder::default()
		.header("wrapper.hpp")
		.clang_arg("--std=c++11")
		.allowlist_item("ultimateAlprSdk::(.+)")
		.parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
		.generate()
		.expect("Unable to generate bindings");

	let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

	bindings
		.write_to_file(out_path.join("bindings.rs"))
		.expect("Couldn't write bindings!");
}

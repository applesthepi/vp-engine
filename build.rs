fn main() {
	cxx_build::bridges(&[
		"src/bindings/program.rs",
	]).flag_if_supported("-std=c++20")
		.compile("vpe");
	println!("cargo:rerun-if-changed=src/bindings/program.rs");
}
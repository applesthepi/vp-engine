fn main() {
	cxx_build::bridges(&[
		"src/bindings/program.rs",
	]).flag_if_supported("-std=c++20")
		.flag_if_supported("-D_ITERATOR_DEBUG_LEVEL=0")
		.flag_if_supported("-DRuntimeLibrary=MD_DynamicDebug")
		
		.compile("vpe");
	println!("cargo:rerun-if-changed=src/bindings/program.rs");
}
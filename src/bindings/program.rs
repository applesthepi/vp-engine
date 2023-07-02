#[cxx::bridge(namespace = "vpe")]
mod cxx_program {
	extern "Rust" {
		fn test(text: String) -> String;
	}
}

pub fn test(text: String) -> String {
	text + "_YOO"
}
pub struct Program {
	pub instance: vpb::Instance,
	pub window: vpb::Window,
}

impl Program {
	pub fn new(
		name: &str,
	) -> Self {
		let instance = vpb::Instance::new(
			name,
			"vpe",
		);
		let window = vpb::Window::new(
			name,
			&instance,
		);
		Self {
			instance,
			window,
		}
	}
}
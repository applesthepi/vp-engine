use crate::Object;

pub struct ObjectRect {
	name: String,
	position: [f32; 2],
	size: [f32; 2],
}

impl ObjectRect {
	pub fn new(
		name: &str,
		position: [f32; 2],
		size: [f32; 2],
	) -> Self {
		let name = name.to_string();
		Self {
			name,
			position,
			size,
		}
	}
}

impl Object for ObjectRect {
	fn name(&self) -> &String {
		&self.name
	}
	fn render(
		&self,
		command_buffer: vk::CommandBuffer,
	) {

	}
}
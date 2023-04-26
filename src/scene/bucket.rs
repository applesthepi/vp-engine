use std::sync::Arc;

use ash::vk;

use crate::Object;

pub struct Bucket {
	pub name: String,
	pub pipeline: Box<dyn vpb::Pipeline>,
	objects: Vec<Arc<dyn Object>>,
}

impl Bucket {
	pub fn new(
		name: &str,
		pipeline: Box<dyn vpb::Pipeline>,
	) -> Self {
		let name = name.to_string();
		let objects: Vec<Arc<dyn Object>> = Vec::with_capacity(1024);
		Self {
			name,
			pipeline,
			objects,
		}
	}

	pub fn get_object(
		&self,
		name: &str,
	) -> Arc<dyn Object> {
		self.objects.iter().find(
			|x|
			x.name() == name
		).expect(format!("no object with name \"{}\" inside bucket \"{}\"", name, self.name).as_str()).clone()
	}

	pub fn add_object(
		&mut self,
		object: Arc<dyn Object>,
	) {
		self.objects.push(object);
	}

	pub fn render(
		&mut self,
		device: &vpb::Device,
		command_buffer: vk::CommandBuffer,
	) { unsafe {
		for object in self.objects.iter() {
			let vertex_buffer = object.vertex_buffer();
			let index_buffer = object.index_buffer();
			vertex_buffer.bind(device, command_buffer);
			index_buffer.bind(device, command_buffer);
			device.device.cmd_draw_indexed(
				command_buffer,
				index_buffer.index_count() as u32,
				1,
				0,
				0,
				1,
			);
		}
	}}
}
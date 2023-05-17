use std::sync::Arc;

use ash::vk;

use crate::Object;

pub struct Bucket<'a> {
	pub name: String,
	pub pipeline: Box<dyn vpb::Pipeline>,
	objects: Vec<Arc<dyn Object>>,
	device: &'a vpb::Device,
	instance: &'a vpb::Instance,
	descriptor_pool: &'a vk::DescriptorPool,
	frame_count: usize,
	binding: u32,
}

impl<'a> Bucket<'a> {
	pub fn new(
		name: &str,
		pipeline: Box<dyn vpb::Pipeline>,
		device: &'a vpb::Device,
		instance: &'a vpb::Instance,
		descriptor_pool: &'a vk::DescriptorPool,
		frame_count: usize,
		binding: u32,
	) -> Self {
		let name = name.to_string();
		let objects: Vec<Arc<dyn Object>> = Vec::with_capacity(1024);
		Self {
			name,
			pipeline,
			objects,
			device,
			instance,
			descriptor_pool,
			frame_count,
			binding,
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
		unsafe {
			let wa_object = Arc::get_mut_unchecked(
				&mut object,
			);
			wa_object.setup_block_state(
				self.device,
				self.instance,
				self.descriptor_pool,
				self.frame_count,
				self.binding,
			);
		}
		self.objects.push(object);
	}

	pub fn render(
		&mut self,
		device: &vpb::Device,
		command_buffer: vk::CommandBuffer,
		frame: usize,
	) { unsafe {
		device.device.cmd_bind_pipeline(
			command_buffer,
			vk::PipelineBindPoint::GRAPHICS,
			self.pipeline.get_pipeline(),
		);
		device.device.cmd_set_viewport(
			command_buffer,
			0,
			&self.pipeline.get_viewport(),
		);
		device.device.cmd_set_scissor(
			command_buffer,
			0,
			&self.pipeline.get_scissor(),
		);
		self.pipeline.bind_blocks(
			device,
			&command_buffer,
			frame,
		);
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
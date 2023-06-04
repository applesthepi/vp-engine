use std::sync::Arc;

use ash::vk;

use crate::{Object, ProgramData, EnginePipeline};

pub struct Bucket {
	pub name: String,
	pub pipeline: Arc<dyn vpb::Pipeline>,
	pub pipeline_engine: Arc<dyn EnginePipeline>,
	pub program_data: ProgramData,
	objects: Vec<Arc<dyn Object>>,
	binding: u32,
}

impl Bucket {
	pub fn new(
		name: &str,
		pipeline: Arc<dyn vpb::Pipeline>,
		pipeline_engine: Arc<dyn EnginePipeline>,
		program_data: ProgramData,
		binding: u32,
	) -> Self {
		let name = name.to_string();
		let objects: Vec<Arc<dyn Object>> = Vec::with_capacity(1024);
		Self {
			name,
			pipeline,
			pipeline_engine,
			program_data,
			objects,
			binding,
		}
	}

	pub fn get_object(
		&self,
		name: &str,
	) -> Arc<dyn Object> {
		self.objects.iter().find(
			|x|
			x.state().name == name
		).expect(format!("no object with name \"{}\" inside bucket \"{}\"", name, self.name).as_str()).clone()
	}

	pub fn add_object(
		&mut self,
		mut object: Arc<dyn Object>,
	) {
		let block_states = self.pipeline_engine.create_block_states(
			&self.program_data,
			&self.pipeline.get_descriptor_pool(),
		);
		let wa_object = vpb::gmuc!(object);
		let mut wa_object_state = wa_object.state();
		let wa_object_state = vpb::gmuc!(wa_object_state);
		wa_object_state.block_states = Some(block_states);
		self.objects.push(object);
	}

	pub fn update_blocks(
		&mut self,
		device: &vpb::Device,
		command_buffer: &vk::CommandBuffer,
		frame: usize,
	) {
		vpb::gmuc!(self.pipeline).update_blocks(
			device,
			command_buffer,
			frame,
		);
		// for object in self.objects.iter() {
		// 	object.update_block_states(
		// 		device,
		// 		command_buffer,
		// 		frame,
		// 	);
		// }
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
		for object in self.objects.iter() {
			let object_state = object.state();
			let block_states = &object_state.block_states.as_ref().expect(
				"attempting to bind no block states during rendering"
			);
			// TODO: USE
			// let block_state_layouts: Vec<vk::DescriptorSet> = block_states.iter().map(
			// 	|x| {
			// 		x.descriptor_buffers[frame].set
			// 	}
			// ).collect();
			let block_state_layouts: Vec<vk::DescriptorSet> = vec![block_states[0].descriptor_buffers[frame].set];
			device.device.cmd_bind_descriptor_sets(
				command_buffer,
				vk::PipelineBindPoint::GRAPHICS,
				self.pipeline.get_pipeline_layout(),
				0,
				&block_state_layouts,
				&[],
			);
			object_state.bind_buffers(
				&self.program_data,
				&command_buffer,
			);
			device.device.cmd_draw_indexed(
				command_buffer,
				object_state.index_count(),
				1,
				0,
				0,
				1,
			);
		}
	}}

	pub fn destroy_descriptor_set_layouts(
		&mut self,
	) { unsafe {
		vpb::gmuc!(self.pipeline).destroy_set_layout(&self.program_data.device);
		for object in self.objects.iter() {
			let object_state = object.state();
			for block_state in object_state.block_states.as_ref().expect(
				"attempting to destroy block states in object without any block states"
			).iter().skip(1) {
				self.program_data.device.device.destroy_descriptor_set_layout(
					block_state.layout,
					None,
				);
			}
		}
	}}
}
use std::sync::Arc;

use ash::vk;

use crate::{Object, ProgramData, EnginePipeline, pf, render_state, InputState, RenderState};

pub struct Bucket {
	pub name: String,
	// pub pipeline: Arc<dyn vpb::Pipeline>,
	pub engine_pipeline: Arc<dyn EnginePipeline>,
	pub program_data: ProgramData,
	objects: Vec<Arc<dyn Object>>,
}

impl Bucket {
	pub fn new(
		name: &str,
		// pipeline: Arc<dyn vpb::Pipeline>,
		pipeline_engine: Arc<dyn EnginePipeline>,
		program_data: ProgramData,
	) -> Self {
		let name = name.to_string();
		let objects: Vec<Arc<dyn Object>> = Vec::with_capacity(1024);
		Self {
			name,
			// pipeline,
			engine_pipeline: pipeline_engine,
			program_data,
			objects,
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
		let pipeline_info = self.engine_pipeline.get_pipeline_info();
		let mut block_states = pipeline_info.block_states.clone();
		block_states.extend(pf::create_object_block_states(
			&self.program_data,
			&self.engine_pipeline,
		));
		let wa_object = vpb::gmuc!(object);
		let mut wa_object_state = wa_object.state();
		let wa_object_state = vpb::gmuc!(wa_object_state);
		wa_object_state.block_states = Some(block_states);
		self.objects.push(object);
	}

	pub fn update_blocks(
		&mut self,
		input_state: &InputState,
		render_state: &RenderState,
	) {
		vpb::gmuc!(self.engine_pipeline).update_block_states(
			&self.program_data,
			input_state,
			render_state,
		);
		for object in self.objects.iter_mut() {
			vpb::gmuc_ref!(object).update_block_states(
				&self.program_data.device,
				render_state.frame,
				self.program_data.frame_count,
			);
		}
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
			self.engine_pipeline.get_pipeline_info().pipeline,
		);
		device.device.cmd_set_viewport(
			command_buffer,
			0,
			&self.engine_pipeline.get_pipeline_info().viewport,
		);
		device.device.cmd_set_scissor(
			command_buffer,
			0,
			&self.engine_pipeline.get_pipeline_info().scissor,
		);
		for object in self.objects.iter() {
			let object_state = object.state();
			let block_states = &object_state.block_states.as_ref().expect(
				"attempting to bind no block states during rendering"
			);
			// TODO: USE
			let block_state_layouts: Vec<vk::DescriptorSet> = block_states.iter().map(
				|x| {
					x.frame_sets[frame].set
				}
			).collect();
			device.device.cmd_bind_descriptor_sets(
				command_buffer,
				vk::PipelineBindPoint::GRAPHICS,
				self.engine_pipeline.get_pipeline_info().pipeline_layout,
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

	pub fn destroy_block_state_memory(
		&mut self,
	) { unsafe {
		let mut pipeline_info = self.engine_pipeline.get_pipeline_info();
		let pipeline_info = vpb::gmuc!(pipeline_info);
		for block_state in pipeline_info.block_states.iter_mut() {
			let block_state = vpb::gmuc!(*block_state);
			block_state.destroy_memory(&self.program_data.device);
		}
		for object in self.objects.iter() {
			let mut state = object.state();
			let wa_state = vpb::gmuc!(state);
			let block_states = wa_state.block_states.as_mut().expect(
				"attempting to recreate block states when there are none",
			);
			for block_state in block_states.iter_mut().skip(1) {
				let block_state = vpb::gmuc_ref!(block_state);
				block_state.destroy_memory(&self.program_data.device);
			}
		}
	}}
	
	pub fn recreate_block_state_memory(
		&mut self,
	) { unsafe {
		let mut pipeline_info = self.engine_pipeline.get_pipeline_info();
		let pipeline_info = vpb::gmuc!(pipeline_info);
		for block_state in pipeline_info.block_states.iter_mut() {
			let block_state = vpb::gmuc_ref!(block_state);
			block_state.recreate_memory(
				&self.program_data.device,
				&self.program_data.instance,
				&self.program_data.descriptor_pool.descriptor_pool,
				self.program_data.frame_count,
			);
		}
		for object in self.objects.iter() {
			let mut state = object.state();
			let wa_state = vpb::gmuc!(state);
			let block_states = wa_state.block_states.as_mut().expect(
				"attempting to recreate block states when there are none",
			);
			for block_state in block_states.iter_mut().skip(1) {
				let block_state = vpb::gmuc_ref!(block_state);
				block_state.recreate_memory(
					&self.program_data.device,
					&self.program_data.instance,
					&self.program_data.descriptor_pool.descriptor_pool,
					self.program_data.frame_count,
				);
			}
		}
	}}

	pub fn destroy_pipeline(
		&mut self,
	) { unsafe {
		let mut pipeline_info = self.engine_pipeline.get_pipeline_info();
		let pipeline_info = vpb::gmuc!(pipeline_info);
		pipeline_info.destroy_pipeline(
			&self.program_data,
		);
	}}

	pub fn recreate_pipeline(
		&mut self,
	) { unsafe {
		let engine_pipeline = vpb::gmuc!(self.engine_pipeline);
		engine_pipeline.recreate_pipeline(&self.program_data);
	}}
}
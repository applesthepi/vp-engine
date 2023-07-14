use std::sync::Arc;

use ash::vk;

use crate::{ProgramData, EnginePipeline, pf, InputState, RenderState, rendering::{RenderingState, sub}, r#static::{ObjectStatic, state::StaticState}, dynamic::{ObjectDynamic, state::DynamicState}, update::UpdateState, ObjectStateBuffers};

pub struct Bucket {
	pub name: String,
	pub engine_pipeline: Arc<dyn EnginePipeline>,
	pub program_data: ProgramData,
	objects_rs: Vec<Arc<dyn RenderingState>>,
	objects_us: Vec<Arc<dyn UpdateState>>,
}

impl Bucket {
	pub fn new(
		name: &str,
		pipeline_engine: Arc<dyn EnginePipeline>,
		program_data: ProgramData,
	) -> Self {
		let name = name.to_string();
		let objects_rs: Vec<Arc<dyn RenderingState>> = Vec::with_capacity(1024);
		let objects_us: Vec<Arc<dyn UpdateState>> = Vec::with_capacity(1024);
		Self {
			name,
			engine_pipeline: pipeline_engine,
			program_data,
			objects_rs,
			objects_us,
		}
	}

	pub fn get_static_object(
		&self,
		name: &str,
	) -> Arc<dyn RenderingState> {
		self.objects_rs.iter().find(
			|x|
			x.sub_state().name == name
		).expect(format!("no object with name \"{}\" inside bucket \"{}\"", name, self.name).as_str()).clone()
	}

	pub fn add_static_object(
		&mut self,
		mut static_state: Arc<StaticState>,
		update_state: Arc<dyn UpdateState>,
	) {
		let pipeline_info = self.engine_pipeline.get_pipeline_info();
		let mut block_states = pipeline_info.block_states.clone();
		block_states.extend(pf::create_object_block_states(
			&self.program_data,
			&self.engine_pipeline,
		));
		let wa_object_state = vpb::gmuc!(static_state);
		vpb::gmuc!(wa_object_state.sub_state).block_states = Some(block_states);
		drop(wa_object_state);
		self.objects_rs.push(static_state);
		self.objects_us.push(update_state);
	}

	pub fn add_dynamic_object(
		&mut self,
		mut dynamic_state: Arc<DynamicState>,
		update_state: Arc<dyn UpdateState>,
	) {
		let pipeline_info = self.engine_pipeline.get_pipeline_info();
		let mut block_states = pipeline_info.block_states.clone();
		block_states.extend(pf::create_object_block_states(
			&self.program_data,
			&self.engine_pipeline,
		));
		let wa_object_state = vpb::gmuc!(dynamic_state);
		vpb::gmuc!(wa_object_state.sub_state).block_states = Some(block_states);
		drop(wa_object_state);
		self.objects_rs.push(dynamic_state);
		self.objects_us.push(update_state);
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
		for object in self.objects_us.iter_mut() {
			let wa_object = vpb::gmuc_ref!(object);
			wa_object.update_block_states(
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
		for object in self.objects_rs.iter() {
			let mut sub_state = object.sub_state();
			if !sub_state.enabled {
				continue;
			}
			let sub_state = vpb::gmuc!(sub_state);
			let block_states = &sub_state.block_states.as_ref().expect(
				"attempting to bind no block states during rendering"
			);
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
			object.bind_buffers(
				&self.program_data,
				&command_buffer,
			);
			match &object.sub_state().buffers {
				ObjectStateBuffers::GOIndexed(
					_,
					index_buffer,
				) => {
					device.device.cmd_draw_indexed(
						command_buffer,
						index_buffer.index_count as u32,
						1,
						0,
						0,
						0,
					);
				},
				ObjectStateBuffers::GOIndirect(
					indirect_buffer,
				) => {
					device.device.cmd_draw_indexed_indirect(
						command_buffer,
						indirect_buffer.indirect_gpu,
						0,
						indirect_buffer.indirect_count as u32,
						std::mem::size_of::<vk::DrawIndexedIndirectCommand>() as u32,
					);
				},
				ObjectStateBuffers::GOInstanced(
					instance_buffer,
				) => {
					device.device.cmd_draw_indexed(
						command_buffer,
						instance_buffer.index_count as u32,
						instance_buffer.instance_count as u32,
						0,
						0,
						0,
					);
				},
			}
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
		for object in self.objects_rs.iter() {
			let mut state = object.sub_state();
			let state = vpb::gmuc!(state);
			let block_states = state.block_states.as_mut().expect(
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
		for object in self.objects_rs.iter() {
			let mut state = object.sub_state();
			let state = vpb::gmuc!(state);
			let block_states = state.block_states.as_mut().expect(
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
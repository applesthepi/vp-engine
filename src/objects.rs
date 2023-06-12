use std::{sync::Arc, cell::{RefCell, OnceCell}};

use ash::vk;

use crate::ProgramData;

pub mod ui;

/// Contains vertex and index buffers. Stores different configurations of those.
pub enum ObjectStateBuffers {
	GO(Arc<vpb::VertexBufferGO>, Arc<vpb::IndexBufferGO>),
}

/// Every object has an object state to be referenced from above.
pub struct ObjectState {
	pub name: String,
	pub block_states: Option<Vec<Arc<vpb::BlockState>>>,
	pub buffers: ObjectStateBuffers,
}

impl ObjectState {
	pub fn bind_buffers(
		&self,
		program_data: &ProgramData,
		command_buffer: &vk::CommandBuffer,
	) { unsafe {
		match &self.buffers {
			ObjectStateBuffers::GO(
				vertex_buffer,
				index_buffer,
			) => {
				program_data.device.device.cmd_bind_vertex_buffers(
					*command_buffer,
					0,
					&[vertex_buffer.buffer_gpu],
					&[0],
				);
				program_data.device.device.cmd_bind_index_buffer(
					*command_buffer,
					index_buffer.buffer_gpu,
					0,
					vk::IndexType::UINT32,
				);
			}
		}
	}}

	pub fn index_count(
		&self,
	) -> u32 {
		match &self.buffers {
			ObjectStateBuffers::GO(
				vertex_buffer,
				index_buffer,
			) => {
				index_buffer.index_count as u32
			}
		}
	}
}

pub trait Object {
	fn state(&self) -> Arc<ObjectState>;
	// fn name(&self) -> &String;
	// fn vertex_buffer(&self) -> Arc<dyn vpb::VertexBuffer>;
	// fn index_buffer(&self) -> Arc<dyn vpb::IndexBuffer>;
	// fn setup_block_states(
	// 	&mut self,
	// 	program_data: &ProgramData,
	// 	descriptor_pool: &vk::DescriptorPool,
	// 	pipeline_block_state: Arc<vpb::BlockState>,
	// );
	fn update_block_states(
		&self,
		device: &vpb::Device,
		frame: usize,
	);
	// fn bind_block_states(
	// 	&self,
	// 	device: &vpb::Device,
	// 	command_buffer: &vk::CommandBuffer,
	// 	pipeline_layout: &vk::PipelineLayout,
	// 	frame: usize,
	// );
}
use std::{sync::Arc, cell::{RefCell, OnceCell}};

use ash::vk;

use crate::ProgramData;

pub mod ui;

/// Contains vertex and index buffers. Stores different configurations of those.
pub enum ObjectStateBuffers {
	GO(Arc<vpb::VertexBufferGO>, Arc<vpb::IndexBufferGO>),
}

pub enum DirtyState {
	Clean,
	
	// SPECIFIC

	/// Vertex buffers.
	VB,
	/// Index buffers.
	IB,
	/// Block states.
	BS,

	// GENERAL
	
	Position,
	Size,
	Mesh,
}

/// Every object has an object state to be referenced from above.
pub struct ObjectState {
	pub name: String,
	pub block_states: Option<Vec<Arc<vpb::BlockState>>>,
	pub buffers: ObjectStateBuffers,
	pub dirty_state: DirtyState,
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
	
	fn update_block_states(
		&self,
		device: &vpb::Device,
		frame: usize,
	);

	fn dirty(
		&mut self,
		dirty_state: DirtyState,
	) {
		vpb::gmuc!(self.state()).dirty_state = dirty_state;
	}
}
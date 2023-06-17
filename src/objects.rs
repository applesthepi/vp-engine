use std::{sync::Arc, cell::{RefCell, OnceCell}};

use ash::vk;
use bitflags::bitflags;

use crate::ProgramData;

pub mod ui;

/// Contains vertex and index buffers. Stores different configurations of those.
pub enum ObjectStateBuffers {
	GO(Arc<vpb::VertexBufferGO>, Arc<vpb::IndexBufferGO>),
}

macro_rules! bit_compare {
	($a:expr, $b:expr) => {
		($a & $b) == $b
	};
}

bitflags! {
	#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
	pub struct DirtyState: u32 {
		/// Vertex buffer.
		const VB = 0b00000001;
		/// Index buffers.
		const IB = 0b00000010;
		/// Block states.
		const BS = 0b00000100;
		const Position = Self::BS.bits();
		const Mesh = Self::VB.bits() | Self::IB.bits();
		const All = Self::VB.bits() | Self::IB.bits() | Self::BS.bits();
	}
}

/// Every object has an object state to be referenced from above.
pub struct ObjectState {
	pub name: String,
	pub block_states: Option<Vec<Arc<vpb::BlockState>>>,
	pub buffers: ObjectStateBuffers,
	pub dirty_state: DirtyState,
	pub bs_left: u8,
}

impl ObjectState {
	pub fn new(
		program_data: &ProgramData,
		name: String,
		buffers: ObjectStateBuffers,
	) -> Self {
		Self {
			name,
			block_states: None,
			buffers,
			dirty_state: DirtyState::All,
			bs_left: 0,
		}
	}

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

	fn dirty(
		&mut self,
		dirty_state: DirtyState,
	) {
		let mut state = self.state();
		let state = vpb::gmuc!(state);
		state.dirty_state |= dirty_state;
	}

	fn update_vb(
		&self,
		device: &vpb::Device,
	);

	fn update_ib(
		&self,
		device: &vpb::Device,
	);

	fn update_bs(
		&self,
		device: &vpb::Device,
		frame: usize,
	);

	fn update_block_states(
		&mut self,
		device: &vpb::Device,
		frame: usize,
		frame_count: usize,
	) {
		let mut state = self.state();
		let dirty_state = state.as_ref().dirty_state;
		let bs_left = &mut vpb::gmuc!(state).bs_left;
		// Position: BS
		// Mesh: VB & IB
		let mut vb = false;
		let mut ib = false;
		let mut bs = false;
		if bit_compare!(dirty_state, DirtyState::VB) {
			vb = true;
			self.update_vb(device);
		}
		if bit_compare!(dirty_state, DirtyState::IB) {
			ib = true;
			self.update_ib(device);
		}
		let bs_state = bit_compare!(dirty_state, DirtyState::BS);
		if bs_state || *bs_left > 0 {
			bs = true;
			self.update_bs(device, frame);
			if bs_state {
				*bs_left = frame_count as u8 - 1;
			} else {
				*bs_left -= 1;
			}
		}
		if bit_compare!(dirty_state, DirtyState::Position) {
			if !bs {
				self.update_bs(device, frame);
			}
		}
		if bit_compare!(dirty_state, DirtyState::Mesh) {
			if !vb {
				self.update_vb(device);
			}
			if !ib {
				self.update_ib(device);
			}
		}
		vpb::gmuc!(self.state()).dirty_state = DirtyState::empty();
	}
}
use std::sync::Arc;

use ash::vk;
use vpb::{VertexBuffer, IndexBuffer, InstanceBuffer};

use crate::ProgramData;

/// Contains vertex and index buffers. Stores different configurations of those.
pub enum ObjectStateBuffers {
	// TODO: combine
	GOIndexed(Arc<vpb::VB_GO_Indexed>, Arc<vpb::IB_GO_Indexed>),
	GOIndirect(Arc<vpb::GO_Indirect>),
	GOInstanced(Arc<vpb::GO_Instanced>),

	// CGO(Arc<vpb::VertexBufferCGO<V>>, Arc<vpb::IndexBufferCGO>),
}

macro_rules! bit_compare {
	($a:expr, $b:expr) => {
		($a & $b) == $b
	};
}

bitflags::bitflags! {
	#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
	pub struct DynamicDirtyState: u32 {
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

bitflags::bitflags! {
	#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
	pub struct StaticDirtyState: u32 {
		/// Vertex & Index buffers.
		const VIB = 0b00000001;
		/// Block states.
		const BS = 0b00000010;
	}
}

pub fn bind_buffers(
	program_data: &ProgramData,
	command_buffer: &vk::CommandBuffer,
	buffers: &ObjectStateBuffers,
) { unsafe {
	match buffers {
		ObjectStateBuffers::GOIndexed(
			vertex_buffer,
			index_buffer,
		) => {
			vertex_buffer.bind(
				&program_data.device,
				*command_buffer,
			);
			index_buffer.bind(
				&program_data.device,
				*command_buffer,
			);
		},
		ObjectStateBuffers::GOIndirect(
			indirect_buffer,
		) => {
			VertexBuffer::bind(
				indirect_buffer.as_ref(),
				&program_data.device,
				*command_buffer,
			);
			IndexBuffer::bind(
				indirect_buffer.as_ref(),
				&program_data.device,
				*command_buffer,
			);
		},
		ObjectStateBuffers::GOInstanced(
			instance_buffer,
		) => {
			VertexBuffer::bind(
				instance_buffer.as_ref(),
				&program_data.device,
				*command_buffer,
			);
			IndexBuffer::bind(
				instance_buffer.as_ref(),
				&program_data.device,
				*command_buffer,
			);
			InstanceBuffer::bind(
				instance_buffer.as_ref(),
				&program_data.device,
				*command_buffer,
			);
		},
	}
}}

pub fn index_count(
	buffers: &ObjectStateBuffers,
) -> u32 {
	match buffers {
		ObjectStateBuffers::GOIndexed(
			_,
			index_buffer,
		) => {
			index_buffer.index_count as u32
		},
		ObjectStateBuffers::GOIndirect(
			indirect_buffer,
		) => {
			indirect_buffer.index_count as u32
		},
		ObjectStateBuffers::GOInstanced(
			instance_buffer,
		) => {
			instance_buffer.index_count as u32
		},
	}
}
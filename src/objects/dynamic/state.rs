use std::sync::Arc;

use ash::vk;

use crate::{ObjectStateBuffers, DynamicDirtyState, ProgramData, rendering::{RenderingState, sub::SubState}, objects::buffers};

/// Every dynamic object has an object state to be referenced from parents.
pub struct DynamicState {
	// TODO: lifetime so its not stored random heap
	pub sub_state: Arc<SubState>,
	pub dirty_state: DynamicDirtyState,
	pub bs_left: u8,
}

impl DynamicState {
	pub fn new(
		program_data: &ProgramData,
		name: String,
		buffers: ObjectStateBuffers,
	) -> Self {
		Self {
			sub_state: Arc::new(SubState {
				name,
				block_states: None,
				buffers,
				enabled: true,
			}),
			dirty_state: DynamicDirtyState::All,
			bs_left: 0,
		}
	}
}

// TODO: redundent? optimize? vtable!!!?
impl RenderingState for DynamicState {
	fn sub_state(
		&self,
	) -> Arc<SubState> {
		self.sub_state.clone()
	}

	fn bind_buffers(
		&self,
		program_data: &ProgramData,
		command_buffer: &vk::CommandBuffer,
	) { unsafe {
		buffers::bind_buffers(
			program_data,
			command_buffer,
			&self.sub_state.buffers,
		);
	}}

	fn index_count(
		&self,
	) -> u32 {
		buffers::index_count(
			&self.sub_state.buffers,
		)
	}
}
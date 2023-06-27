use std::sync::Arc;

use ash::vk;

use crate::{ObjectStateBuffers, StaticDirtyState, ProgramData, objects::buffers, rendering::{sub::SubState, RenderingState}};

/// Every static object has an object state to be referenced from parents.
pub struct StaticState {
	pub sub_state: Arc<SubState>,
	pub dirty_state: StaticDirtyState,
	pub bs_left: u8,
}

impl StaticState {
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
			}),
			dirty_state: StaticDirtyState::all(),
			bs_left: 0,
		}
	}
}

impl RenderingState for StaticState {
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
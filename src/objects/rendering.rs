use std::sync::Arc;

use ash::vk;
use vpb::ProgramData;

use self::sub::SubState;

pub mod sub;

pub trait RenderingState {
	fn sub_state(
		&self,
	) -> Arc<SubState>;

	fn bind_buffers(
		&self,
		program_data: &ProgramData,
		command_buffer: &vk::CommandBuffer,
	);

	fn index_count(
		&self,
	) -> u32;
}
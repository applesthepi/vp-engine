use ash::vk;
use vpb::ProgramData;

pub trait UpdateState {
	/// Give the option for the object to update their
	/// block states during the render loop.
	fn update_block_states(
		&mut self,
		program_data: &ProgramData,
		frame: usize,
		frame_count: usize,
		command_buffer: &vk::CommandBuffer,
		pipeline_layout: &vk::PipelineLayout,
	);
}
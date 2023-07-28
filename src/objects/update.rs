use ash::vk;

pub trait UpdateState {
	/// Give the option for the object to update their
	/// block states during the render loop.
	fn update_block_states(
		&mut self,
		instance: &vpb::Instance,
		device: &vpb::Device,
		frame: usize,
		frame_count: usize,
		command_buffer: &vk::CommandBuffer,
		pipeline_layout: &vk::PipelineLayout,
	);
}
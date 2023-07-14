pub trait UpdateState {
	/// Give the option for the object to update their
	/// block states during the render loop.
	fn update_block_states(
		&mut self,
		device: &vpb::Device,
		frame: usize,
		frame_count: usize,
	);
}
pub trait UpdateState {
	fn update_block_states(
		&mut self,
		device: &vpb::Device,
		frame: usize,
		frame_count: usize,
	);
}
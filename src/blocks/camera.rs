use std::mem::size_of;

use ash::vk;


pub struct BlockCamera {
	pub view: [f32; 16],
	pub projection: [f32; 16],
}

impl BlockCamera {
	pub fn create_block_state(
		device: &vpb::Device,
		instance: &vpb::Instance,
		descriptor_pool: &vk::DescriptorPool,
		frame_count: usize,
		binding: u32,
		set: u32,
	) -> vpb::BlockState {
		let block_state = vpb::BlockState::new(
			device,
			instance,
			descriptor_pool,
			frame_count,
			binding,
			size_of::<BlockCamera>(),
		);
		block_state
	}
}
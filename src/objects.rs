use std::sync::Arc;

use ash::vk;

pub mod ui;

pub trait Object {
	fn name(&self) -> &String;
	fn vertex_buffer(&self) -> Arc<dyn vpb::VertexBuffer>;
	fn index_buffer(&self) -> Arc<dyn vpb::IndexBuffer>;
	fn setup_block_state(
		&mut self,
		device: &vpb::Device,
		instance: &vpb::Instance,
		descriptor_pool: &vk::DescriptorPool,
		frame_count: usize,
		binding: u32,
	);
}
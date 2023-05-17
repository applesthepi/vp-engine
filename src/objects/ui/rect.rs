use std::{sync::Arc, intrinsics::size_of};

use ash::vk;

use crate::{Object, VertexUI};

pub struct ObjectRect {
	name: String,
	pub position: [f32; 2],
	pub size: [f32; 2],
	pub block_state: Option<vpb::BlockState>,
	vertex_buffer: Arc<vpb::VertexBufferGO>,
	index_buffer: Arc<vpb::IndexBufferGO>,
}

impl ObjectRect {
	pub fn new(
		device: &vpb::Device,
		name: &str,
		position: [f32; 2],
		size: [f32; 2],
	) -> Self {
		let name = name.to_string();
		let color: [f32; 4] = [1.0, 0.5, 0.5, 1.0];
		let vertices = [
			VertexUI {
				position: [position[0], position[1]],
				color,
			},
			VertexUI {
				position: [position[0] + size[0], position[1]],
				color,
			},
			VertexUI {
				position: [position[0] + size[0], position[1] + size[1]],
				color,
			},
			VertexUI {
				position: [position[0], position[1] + size[1]],
				color,
			},
		];
		let indices = [
			0, 1, 3,
			3, 1, 2,
		];
		let vertex_buffer = Arc::new(vpb::VertexBufferGO::new(
			device,
			&vertices,
		));
		let index_buffer = Arc::new(vpb::IndexBufferGO::new(
			device,
			&indices,
		));
		Self {
			name,
			position,
			size,
			block_state: None,
			vertex_buffer,
			index_buffer,
		}
	}
}

impl Object for ObjectRect {
	fn name(&self) -> &String {
		&self.name
	}
	fn vertex_buffer(
		&self,
	) -> Arc<dyn vpb::VertexBuffer> {
		self.vertex_buffer.clone()
	}
	fn index_buffer(
		&self,
	) -> Arc<dyn vpb::IndexBuffer> {
		self.index_buffer.clone()
	}
	fn setup_block_state(
		&mut self,
		device: &vpb::Device,
		instance: &vpb::Instance,
		descriptor_pool: &vk::DescriptorPool,
		frame_count: usize,
		binding: u32,
	) {
		self.block_state = Some(vpb::BlockState::new(
			device,
			instance,
			descriptor_pool,
			frame_count,
			binding,
			size_of::<ObjectRect>(),
		));
	}
}
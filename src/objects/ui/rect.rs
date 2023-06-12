use std::{sync::Arc, intrinsics::size_of, cell::{RefCell, OnceCell}};

use ash::vk;
use nalgebra::{Matrix4, Isometry2, Vector2, vector, Isometry3};

use crate::{Object, VertexUI, BlockModel, ProgramData, BlockCamera, simple, ObjectState, ObjectStateBuffers};

pub struct ObjectRect {
	state: Arc<ObjectState>,
	pub position: [f32; 2],
	pub size: [f32; 2],
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
		let state = Arc::new(ObjectState {
			name,
			block_states: None,
			buffers: ObjectStateBuffers::GO(vertex_buffer, index_buffer),
		});
		Self {
			state,
			position,
			size,
		}
	}
}

impl Object for ObjectRect {
	fn state(&self) -> Arc<ObjectState> {
		self.state.clone()
	}
	
	fn update_block_states(
		&self,
		device: &vpb::Device,
		frame: usize,
	) {
		let matrix = Isometry3::<f32>::new(
			vector![0.0, 0.0, 0.0],
			vector![0.0, 0.0, 0.0],
		).to_matrix();
		let model = BlockModel {
			model: matrix.as_slice().try_into().unwrap(),
		};
		if let Some(block_state) = self.state.block_states.as_ref() {
			block_state[1].update(
				device,
				&model,
				Some(frame),
			);
			println!("updating rect");
		}
	}
}
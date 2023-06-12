use std::sync::Arc;

use crate::{Object, VertexUI, BlockModel, ObjectState, ObjectStateBuffers};

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
		let model = nalgebra_glm::translate(
			&nalgebra_glm::identity(),
			&nalgebra_glm::vec3(50.0, 0.0, 0.0),
		);
		let model_block = BlockModel {
			model,
		};
		if let Some(block_state) = self.state.block_states.as_ref() {
			block_state[1].update(
				device,
				&model_block,
				Some(frame),
			);
		}
	}
}
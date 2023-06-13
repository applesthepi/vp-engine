use std::sync::Arc;

use crate::{Object, VertexUI, BlockModel, ObjectState, ObjectStateBuffers, DirtyState};

pub struct ObjectRect {
	pub state: Option<Arc<ObjectState>>,
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
		let mut object = Self {
			state: None,
			position,
			size,
		};
		let indices = ObjectRect::generate_indices();
		let vertices = object.generate_vertices();
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
			dirty_state: DirtyState::Clean,
		});
		object.state = Some(state);
		object
	}

	fn generate_vertices(
		&self,
	) -> [VertexUI; 4] {
		let color: [f32; 4] = [1.0, 0.5, 0.5, 1.0];
		[
			VertexUI {
				position: [self.position[0], self.position[1]],
				color,
			},
			VertexUI {
				position: [self.position[0] + self.size[0], self.position[1]],
				color,
			},
			VertexUI {
				position: [self.position[0] + self.size[0], self.position[1] + self.size[1]],
				color,
			},
			VertexUI {
				position: [self.position[0], self.position[1] + self.size[1]],
				color,
			},
		]
	}

	fn generate_indices(
	) -> [u32; 6] {
		[
			0, 1, 3,
			3, 1, 2,
		]
	}
}

impl Object for ObjectRect {
	fn state(&self) -> Arc<ObjectState> { unsafe {
		self.state.as_ref().unwrap_unchecked().clone()
	}}
	
	fn update_block_states(
		&self,
		device: &vpb::Device,
		frame: usize,
	) { unsafe {
		match &self.state.as_ref().unwrap_unchecked().dirty_state {
			DirtyState::Clean => {},
			DirtyState::VB |
			DirtyState::Size |
			DirtyState::Mesh => {
				let vertices = self.generate_vertices();
				match &self.state.as_ref().unwrap_unchecked().buffers {
					ObjectStateBuffers::GO(
						vb,
						_,
					) => {
						vb.update(
							device,
							&vertices,
						);
					},
				}
			},
			DirtyState::IB => {
				panic!("not impl");
			},
			DirtyState::BS |
			DirtyState::Position => {
				let model = nalgebra_glm::translate(
					&nalgebra_glm::identity(),
					&nalgebra_glm::vec3(
						self.position[0],
						self.position[1],
						0.0,
					),
				);
				let model_block = BlockModel {
					model,
				};
				if let Some(block_state) = self.state.as_ref().unwrap_unchecked().block_states.as_ref() {
					block_state[1].update(
						device,
						&model_block,
						Some(frame),
					);
				}
			},
		}
	}}
}
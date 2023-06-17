use std::sync::Arc;

use crate::{Object, VertexUI, BlockModel, ObjectState, ObjectStateBuffers, ProgramData};

pub struct ObjectRect {
	pub state: Option<Arc<ObjectState>>,
	pub position: [f32; 2],
	pub size: [f32; 2],
	pub color: [f32; 4],
}

impl ObjectRect {
	pub fn new(
		program_data: &ProgramData,
		name: &str,
		position: [f32; 2],
		size: [f32; 2],
		color: [f32; 4],
	) -> Self {
		let name = name.to_string();
		let mut object = Self {
			state: None,
			position,
			size,
			color,
		};
		let indices = ObjectRect::generate_indices();
		let vertices = object.generate_vertices();
		let vertex_buffer = Arc::new(vpb::VertexBufferGO::new(
			&program_data.device,
			&vertices,
		));
		let index_buffer = Arc::new(vpb::IndexBufferGO::new(
			&program_data.device,
			&indices,
		));
		let state = Arc::new(ObjectState::new(
			program_data,
			name,
			ObjectStateBuffers::GO(vertex_buffer, index_buffer),
		));
		object.state = Some(state);
		object
	}

	fn generate_indices(
	) -> [u32; 6] {
		[
			0, 1, 3,
			3, 1, 2,
		]
	}

	fn generate_vertices(
		&self,
	) -> [VertexUI; 4] {
		let color: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
		[
			VertexUI {
				position: [0.0, 0.0],
				color,
			},
			VertexUI {
				position: [0.0 + self.size[0], 0.0],
				color,
			},
			VertexUI {
				position: [0.0 + self.size[0], self.size[1]],
				color,
			},
			VertexUI {
				position: [0.0, self.size[1]],
				color,
			},
		]
	}
}

impl Object for ObjectRect {
	fn state(&self) -> Arc<ObjectState> { unsafe {
		self.state.as_ref().unwrap_unchecked().clone()
	}}

	fn update_vb(
		&self,
		device: &vpb::Device,
	) { unsafe {
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
	}}

	fn update_ib(
		&self,
		device: &vpb::Device,
	) { unsafe {
		let indices = ObjectRect::generate_indices();
		match &self.state.as_ref().unwrap_unchecked().buffers {
			ObjectStateBuffers::GO(
				_,
				ib,
			) => {
				ib.update(
					device,
					&indices,
				);
			},
		}
	}}

	fn update_bs(
		&self,
		device: &vpb::Device,
		frame: usize,
	) { unsafe {
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
			color: nalgebra_glm::vec4(
				self.color[0],
				self.color[1],
				self.color[2],
				self.color[3],
			),
		};
		self.state.as_ref().unwrap_unchecked().block_states.as_ref().unwrap_unchecked()[1].update(
			device,
			&model_block,
			Some(frame),
		);
	}}
}
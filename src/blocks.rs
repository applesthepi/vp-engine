mod camera3d;
use std::sync::Arc;

pub use camera3d::*;
mod camera2d;
pub use camera2d::*;
mod model_example;
pub use model_example::*;
use nalgebra::Matrix4;
use vpb::ProgramData;

use crate::{InputState, RenderState};

pub trait Camera {
	fn build_perspective(
		&mut self,
		program_data: &ProgramData,
	);

	fn build_view(
		&mut self,
		program_data: &ProgramData,
		input_state: &InputState,
		render_state: &RenderState,
	);

	fn update(
		&self,
		device: &vpb::Device,
		frame: Option<usize>,
		block_state: &Arc<vpb::BlockState>,
	);

	fn get_view(
		&self,
	) -> (Matrix4<f32>, f32);
}
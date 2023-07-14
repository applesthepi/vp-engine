use std::sync::Arc;

use nalgebra::vector;
use winit::event_loop::EventLoop;

use crate::{Program, pipelines::ui_example::PipelineUIExample, CameraState2d};

use super::cxx_program::ProgramContext;

pub struct ProgramHandler {
	pub program_context: Box<ProgramContext>,
	pub program: Program,
	pub event_loop: EventLoop<()>,
}

impl ProgramHandler {
	pub fn new(
		program_context: Box<ProgramContext>,
	) -> Self {
		let event_loop = EventLoop::new();
		let camera_state = Arc::new(CameraState2d::new(
			vector![0.0, 0.0],
		));
		let program = Program::new(
			&program_context.name,
			&event_loop,
			("tiles",
			|program_data| {
				Arc::new(PipelineUIExample::new(
					program_data,
					camera_state.clone(),
				))
			}),
		);
		Self {
			program_context,
			program,
			event_loop,
		}
	}

	pub fn run(
		self,
	) {
		let scene = self.program.scene.clone();
		Program::run(
			Arc::new(self.program.program_data),
			scene,
			self.event_loop,
			move |scene| {  },
			move |scene| {  },
			move |scene| {  },
		);
	}
}
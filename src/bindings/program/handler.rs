use std::sync::Arc;

use winit::event_loop::EventLoop;

use crate::Program;

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
		let program = Program::new(
			&program_context.name,
			&event_loop,
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
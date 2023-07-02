use self::{cxx_program::ProgramContext, handler::ProgramHandler};

static mut PROGRAM_HANDLER: Option<ProgramHandler> = None;

pub mod handler;

#[cxx::bridge(namespace = "vpe")]
mod cxx_program {
	pub struct ProgramContext {
		pub name: String,
	}
	extern "Rust" {
		fn initialize(
			program_context: Box<ProgramContext>,
		);
		fn run();
	}
}

pub fn initialize(
	program_context: Box<ProgramContext>,
) {
	let program_handler = ProgramHandler::new(
		program_context,
	);
	unsafe { PROGRAM_HANDLER = Some(program_handler); }
}

pub fn run(

) { unsafe {
	let program_handler = PROGRAM_HANDLER.take().expect("must be initialized before run");
	program_handler.run();
}}
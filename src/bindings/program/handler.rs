use std::sync::{Arc, atomic::AtomicBool};

use nalgebra::vector;
use winit::event_loop::EventLoop;

use super::{cxx_program::ProgramContext, custom::setup_program};

pub struct ProgramHandler {
	pub program_context: Box<ProgramContext>,
	pub program: vpe::Program,
}

impl ProgramHandler {
	pub fn new(
		program_context: Box<ProgramContext>,
	) -> Self {
		let program = setup_program(&program_context);
		Self {
			program_context,
			program,
		}
	}

	pub fn tick_events(
		&mut self,
	) -> vpe::TickResult {
		self.program.tick_events()
	}
}
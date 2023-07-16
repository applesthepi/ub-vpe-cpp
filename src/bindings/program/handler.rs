use std::sync::Arc;

use nalgebra::vector;
use winit::event_loop::EventLoop;

use super::{cxx_program::ProgramContext, custom::setup_program};

pub struct ProgramHandler {
	pub program_context: Box<ProgramContext>,
	pub program: vpe::Program,
	pub event_loop: EventLoop<()>,
}

impl ProgramHandler {
	pub fn new(
		program_context: Box<ProgramContext>,
	) -> Self {
		let event_loop = EventLoop::new();
		let program = setup_program(&event_loop, &program_context);
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
		vpe::Program::run(
			Arc::new(self.program.program_data),
			scene,
			self.event_loop,
			move |scene| {  },
			move |scene| {  },
			move |scene| {  },
		);
	}
}
use std::sync::Arc;

use nalgebra::vector;
use winit::event_loop::EventLoop;

use super::{cxx_program::ProgramContext, custom::add_buckets};

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
		let camera_state = Arc::new(vpe::CameraState2d::new(
			vector![0.0, 0.0],
		));
		let mut program = vpe::Program::new(
			&program_context.name,
			&event_loop,
			("tiles",
			|program_data| {
				Arc::new(vpe::pipelines::ui_example::PipelineUIExample::new(
					program_data,
					camera_state.clone(),
				))
			}),
		);
		add_buckets(
			vpb::gmuc!(program.scene),
			camera_state.clone(),
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
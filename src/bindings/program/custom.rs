use std::sync::Arc;

use nalgebra::vector;
use ub_vpe_custom::pipelines::ui::PipelineUI;
use winit::event_loop::EventLoop;

use super::cxx_program::ProgramContext;

#[allow(unused_mut)]
pub fn setup_program(
	program_context: &Box<ProgramContext>,
) -> vpe::Program {
	let camera_state = Arc::new(vpe::CameraState2d::new(
		vector![0.0, 0.0],
	));
	let mut program = vpe::Program::new(
		&program_context.name,
		(
			"ub_ui",
			|program_data| {
				Arc::new(PipelineUI::new(
					program_data,
					camera_state.clone(),
				))
			},
		),
	);
	// vpb::gmuc!(program.scene).add_bucket(
	// 	"",
	// 	|program_data| {
	// 	},
	// );
	program
}
use super::program::{PROGRAM_HANDLER, PROGRAM_INIT_ERROR};

#[cxx::bridge(namespace = "vpe::scene")]
pub mod cxx_scene {
	extern "Rust" {
		fn render();
	}
}

pub fn render() { unsafe {
	let program_handler = PROGRAM_HANDLER.as_mut().expect(PROGRAM_INIT_ERROR);
	program_handler.render();
}}
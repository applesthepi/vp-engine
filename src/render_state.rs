use std::time::Instant;

#[derive(Default)]
pub struct RenderState {
	pub frame: usize,
	pub delta_time: f64,
}

pub struct RenderStateLocal {
	pub delta_timer: Instant,
}
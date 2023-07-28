use std::time::Instant;

use nalgebra::Vector2;

pub const DOUBLE_CLICK_MS: u32 = 200;

pub struct InputState {
	pub mouse: MouseState,
	pub down_keys: Vec<bool>,
	pub shift: bool,
	pub control: bool,
	pub alt: bool,
}

pub struct MouseState {
	pub position: Vector2<i32>,
	pub position_delta: Vector2<i32>,
	pub scroll_delta: i32,
	pub left: bool,
	pub middle: bool,
	pub right: bool,
	pub last_left: Instant,
}

impl Default for MouseState {
	fn default() -> Self {
		Self {
			position: Vector2::default(),
			position_delta: Vector2::default(),
			scroll_delta: 0,
			left: false,
			middle: false,
			right: false,
			last_left: Instant::now(),
		}
	}
}

impl InputState {
	pub fn new(
	) -> Self {
		let mut state = Self {
			mouse: MouseState::default(),
			down_keys: Vec::with_capacity(255),
			shift: false,
			control: false,
			alt: false,
		};
		state.down_keys.resize(255, false);
		state
	}
}
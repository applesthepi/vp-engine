use nalgebra::Vector2;

pub struct InputState {
	pub mouse: MouseState,
	pub down_keys: Vec<bool>,
}

#[derive(Default)]
pub struct MouseState {
	pub position: Vector2<i32>,
	pub position_delta: Vector2<i32>,
	pub scroll_delta: i32,
	pub left: bool,
	pub middle: bool,
	pub right: bool,
}

impl InputState {
	pub fn new(
	) -> Self {
		let mut state = Self {
			mouse: MouseState::default(),
			down_keys: Vec::with_capacity(255),
		};
		state.down_keys.resize(255, false);
		state
	}
}
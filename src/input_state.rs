#[derive(Default)]
pub struct InputState {
	mouse: MouseState,
}

#[derive(Default)]
pub struct MouseState {
	position: [i32; 2],
	position_delta: [i32; 2],
	scroll_delta: i32,
	left: bool,
	middle: bool,
	right: bool,
}
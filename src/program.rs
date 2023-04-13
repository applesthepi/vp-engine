use winit::{event_loop::ControlFlow, event::{Event, WindowEvent, KeyboardInput, ElementState, VirtualKeyCode}};

use crate::Scene;

pub struct Program<FO, FC, FR>
where
FO: Fn(&mut Box<Scene>) + 'static,
FC: Fn(&mut Box<Scene>) + 'static,
FR: Fn(&mut Box<Scene>) + 'static {
	pub instance: vpb::Instance,
	pub window: vpb::Window,
	pub surface: vpb::Surface,
	pub scene: Box<Scene>,
	fn_open: FO,
	fn_close: FC,
	fn_render: FR,
}

impl<FO, FC, FR> Program<FO, FC, FR>
where
FO: Fn(&mut Box<Scene>) + 'static,
FC: Fn(&mut Box<Scene>) + 'static,
FR: Fn(&mut Box<Scene>) + 'static {
	pub fn new(
		name: &str,
		fn_open: FO,
		fn_close: FC,
		fn_render: FR,
	) -> Self {
		let window = vpb::Window::new(
			name,
		);
		let instance = vpb::Instance::new(
			name,
			"vpe",
			&window,
		);
		let surface = vpb::Surface::new(
			&instance,
			&window,
		);
		let device: vpb::Device::new();
		let scene = Scene::new(

		);
		Self {
			instance,
			window,
			surface,
			scene: Box::new(scene),
			fn_open,
			fn_close,
			fn_render,
		}
	}

	pub fn run(
		self,
	) {
		let mut c_scene = self.scene;
		self.window.event_loop.run(
			move |event: Event<()>, _, control_flow: &mut ControlFlow| {
				*control_flow = ControlFlow::Poll;
				Program::<FO, FC, FR>::event_match(
					&event,
					control_flow,
					&mut c_scene,
					&self.fn_render,
					&self.fn_close,
				);
			}
		);
	}

	fn event_match(
		event: &Event<()>,
		control_flow: &mut ControlFlow,
		scene: &mut Box<Scene>,
		fn_render: &FR,
		fn_close: &FC,
	) {
		match event {
			Event::WindowEvent {
				event:
					WindowEvent::CloseRequested |
					WindowEvent::KeyboardInput {
						input:
							KeyboardInput {
								state: ElementState::Pressed,
								virtual_keycode: Some(VirtualKeyCode::Escape),
								..
							},
						..
					},
				..
			} => {
				*control_flow = ControlFlow::Exit;
				fn_close(scene);
			},
			Event::MainEventsCleared => {
				fn_render(scene);
				scene.render();
			},
			_ => {},
		}
	}
}
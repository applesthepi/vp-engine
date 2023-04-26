use std::sync::Arc;

use winit::{event_loop::ControlFlow, event::{Event, WindowEvent, KeyboardInput, ElementState, VirtualKeyCode}};

use crate::Scene;

pub struct Program<'a> {
	pub instance: vpb::Instance,
	pub surface: vpb::Surface,
	pub window: vpb::Window,
	pub scene: Box<Scene>,
	pub shader_loader: Arc<vpb::ShaderLoader<'a>>,
}

impl<'a> Program<'a> {
	pub fn new(
		name: &str,
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
		let mut device = vpb::Device::new(
			&instance,
			&surface,
		);
		let swapchain = vpb::Swapchain::new(
			&instance,
			&window,
			&surface,
			&device,
		);
		let mut command_pool = vpb::CommandPool::new(
			&device,
		);
		let command_buffer = vpb::CommandBuffer::new(
			&mut device,
			&mut command_pool,
		);
		let renderpass = vpb::RenderPass::new(
			&device,
			&swapchain,
		);
		let shader_loader = vpb::ShaderLoader::new();
		let scene = Scene::new(
			device,
			command_buffer,
			renderpass,
			&surface,
			swapchain,
			&window,
			&shader_loader,
			window.extent,
		);
		Self {
			instance,
			surface,
			window,
			scene: Box::new(scene),
			shader_loader,
		}
	}

	pub fn run<FO, FC, FR>(
		self,
		fn_open: FO,
		fn_close: FC,
		fn_render: FR,
	) where
	FO: Fn(&mut Box<Scene>) + 'static,
	FC: Fn(&mut Box<Scene>) + 'static,
	FR: Fn(&mut Box<Scene>) + 'static {
		let mut c_scene = self.scene;
		fn_open(&mut c_scene);
		self.window.event_loop.run(
			move |event: Event<()>, _, control_flow: &mut ControlFlow| {
				*control_flow = ControlFlow::Poll;
				Program::event_match(
					&event,
					control_flow,
					&mut c_scene,
					&fn_render,
					&fn_close,
				);
			}
		);
	}

	fn event_match<FC, FR>(
		event: &Event<()>,
		control_flow: &mut ControlFlow,
		scene: &mut Box<Scene>,
		fn_render: &FR,
		fn_close: &FC,
	) where
	FC: Fn(&mut Box<Scene>) + 'static,
	FR: Fn(&mut Box<Scene>) + 'static {
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
				scene.idle();
			},
			Event::MainEventsCleared => {
				fn_render(scene);
				scene.render();
			},
			_ => {},
		}
	}
}
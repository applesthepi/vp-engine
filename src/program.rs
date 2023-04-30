use std::{sync::Arc, marker::PhantomData, rc::Rc, borrow::Borrow};

use ash::vk::Instance;
use winit::{event_loop::{ControlFlow, EventLoop}, event::{Event, WindowEvent, KeyboardInput, ElementState, VirtualKeyCode}};

use crate::Scene;

pub struct Program<'a> {
	pub scene: Arc<Scene<'a>>,
	pub data: Arc<ProgramData>,
}

pub struct ProgramData {
	pub instance: vpb::Instance,
	pub surface: vpb::Surface,
	pub command_pool: vpb::CommandPool,
	pub window: vpb::Window,
}

impl<'b: 'a, 'a> Program<'a> {
	pub fn new(
		name: &str,
		event_loop: &EventLoop<()>,
	) -> Self {
		let mut window = vpb::Window::new(
			name,
			event_loop,
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
			&mut window,
			&surface,
			&device,
		);
		let mut command_pool = vpb::CommandPool::new(
			&device,
		);
		let command_buffer = vpb::CommandBuffer::new(
			&mut device,
			&mut command_pool,
			&swapchain,
		);
		let renderpass = vpb::RenderPass::new(
			&device,
			&swapchain,
		);
		let shader_loader = vpb::ShaderLoader::new();
		let scene = Scene::new(
			device,
			command_buffer,
			&mut command_pool,
			renderpass,
			&surface,
			swapchain,
			&window,
			shader_loader,
			window.extent,
		);
		let data = ProgramData {
			instance,
			surface,
			command_pool,
			window,
		};
		Self {
			scene: Arc::new(scene),
			data: Arc::new(data),
		}
	}

	pub fn run<FO, FC, FR>(
		mut program: Arc<ProgramData>,
		mut scene: Arc<Scene<'static>>,
		event_loop: EventLoop<()>,
		fn_open: FO,
		fn_close: FC,
		fn_render: FR,
	) where
	FO: Fn(&mut Scene) + 'static,
	FC: Fn(&mut Scene) + 'static,
	FR: Fn(&mut Scene) + 'static { unsafe {
		// let mut scene = self.scene.clone();
		{
			let scene = Arc::get_mut_unchecked(
				&mut scene,
			);
			fn_open(scene);
		}
		// let c_program = program.clone();
		event_loop.run(
			move |event: Event<()>, _, control_flow: &mut ControlFlow| {
				let program = Arc::get_mut_unchecked(
					&mut program,
				);
				let scene = Arc::get_mut_unchecked(
					&mut scene,
				);
				*control_flow = ControlFlow::Poll;
				Program::event_match(
					program,
					scene,
					&event,
					control_flow,
					// &mut self,
					&fn_render,
					&fn_close,
				);
			}
		);
	}}

	fn event_match<FC, FR>(
		// program: Rc<Program>,
		program: &mut ProgramData,
		scene: &mut Scene<'a>,
		// mut self,
		event: &Event<()>,
		control_flow: &mut ControlFlow,
		// scene: &mut Arc<Scene>,
		// program
		fn_render: &FR,
		fn_close: &FC,
	) where
	FC: Fn(&mut Scene) + 'static,
	FR: Fn(&mut Scene) + 'static { unsafe {
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
			Event::WindowEvent {
				event: WindowEvent::Resized(size),
				..
			} => {
				scene.resize(
					&program.instance,
					&mut program.window,
					&program.surface,
					&mut program.command_pool,
					[size.width, size.height],
				);
			}
			Event::MainEventsCleared => {
				fn_render(scene);
				scene.render();
			},
			_ => {},
		}
	}}
}
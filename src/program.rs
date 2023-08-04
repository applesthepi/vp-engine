use std::{sync::Arc, marker::PhantomData, rc::Rc, borrow::Borrow, cell::RefCell, fs::File, io::Read, time::{Duration, Instant}};

use ash::vk::{Instance, self};
use glfw::{Key, Action, MouseButton};
use nalgebra::vector;
use shaderc::{ShaderKind, CompileOptions};
use vpb::ProgramData;

use crate::{Scene, pipelines::ui_example::PipelineUIExample, EnginePipeline};

mod macros;
pub use macros::*;

#[derive(Copy, Clone)]
pub enum TickResult {
	CONTINUE,
	RENDER,
	EXIT,
}

pub struct Program {
	pub scene: Arc<Scene>,
	pub program_data: vpb::ProgramData,
	// pub images: Vec<vk::Image>,
}

impl Program {
	pub fn new<FC>(
		name: &str,
		initial_pipeline: (&str, FC),
	) -> Self where FC: Fn(&ProgramData) -> Arc<dyn EnginePipeline> {
		let mut window = vpb::Window::new(
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
			&mut window,
			&surface,
			&device,
		);
		let render_pass = vpb::RenderPass::new(
			&device,
			&swapchain,
		);
		let mut command_pool = vpb::CommandPool::new(
			&device,
		);
		let command_buffer_draw = vpb::CommandBuffer::new(
			&mut device,
			&mut command_pool,
			&swapchain,
		);
		let command_buffer_setup = vpb::CommandBuffer::new(
			&mut device,
			&mut command_pool,
			&swapchain,
		);
		let descriptor_pool = vpb::DescriptorPool::new(
			&device,
			3, // TODO: remove
		);
		let shader_loader = vpb::ShaderLoader::new();
		let mut program_data = ProgramData {
			allocator: Arc::new(None),
			window: Arc::new(window),
			instance: Arc::new(instance),
			surface: Arc::new(surface),
			device: Arc::new(device),
			swapchain: Arc::new(swapchain),
			render_pass: Arc::new(render_pass),
			descriptor_pool: Arc::new(descriptor_pool),
			command_pool: Arc::new(command_pool),
			command_buffer_draw: Arc::new(command_buffer_draw),
			command_buffer_setup: Arc::new(command_buffer_setup),
			shader_loader: Arc::new(shader_loader),
			frame_count: 0,
		};
		let (scene, frame_count) = Scene::new(
			program_data.clone(),
			initial_pipeline,
		);
		program_data.frame_count = frame_count;
		Self {
			scene: Arc::new(scene),
			program_data,
		}
	}

	pub fn tick_events(
		&mut self,
	) -> TickResult {
		if self.program_data.window.window.should_close() {
			return TickResult::EXIT;
		}
		vpb::gmuc!(self.scene).input_state.mouse.scroll_delta = 0;
		let mut program_data = self.program_data.clone();
		let mut scene = self.scene.clone();
		vpb::gmuc!(self.program_data.window).glfw.poll_events();
		for (_, event) in glfw::flush_messages(&self.program_data.window.events) {
			match Program::tick_event(
				&mut program_data,
				&mut scene,
				event,
			) {
				TickResult::CONTINUE => {},
				TickResult::RENDER => { return TickResult::RENDER; },
				TickResult::EXIT => { return TickResult::EXIT; },
			};
		}
		TickResult::RENDER
	}

	fn tick_event(
		program_data: &mut ProgramData,
		scene: &mut Arc<Scene>,
		event: glfw::WindowEvent,
	) -> TickResult {
		let glfw_window = &mut vpb::gmuc!(program_data.window).window;
		match event {
			glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
				glfw_window.set_should_close(true);
				return TickResult::EXIT;
			},
			glfw::WindowEvent::Size(x, y) => {
				let window = vpb::gmuc!(program_data.window);
				let scene = vpb::gmuc_ref!(scene);
				window.extent = vk::Extent2D {
					width: x as u32,
					height: y as u32,
				};
				scene.resize(
					program_data,
				);
			},
			glfw::WindowEvent::Refresh => {
				let scene = vpb::gmuc_ref!(scene);
				scene.resize(program_data);
				return TickResult::CONTINUE;
			},
			glfw::WindowEvent::Key(key, x, action, modifiers) => {
				if (key as i32) >= (Key::Space as i32) && (key as i32) < (Key::GraveAccent as i32) {
					let scene = vpb::gmuc_ref!(scene);
					match action {
						Action::Press => { scene.input_state.down_keys[key as usize] = true; },
						Action::Release => { scene.input_state.down_keys[key as usize] = false; },
						_ => {},
					};
				} else if key == Key::LeftShift {
					let scene = vpb::gmuc_ref!(scene);
					match action {
						Action::Press => { scene.input_state.shift = true; },
						Action::Release => { scene.input_state.shift = false; },
						_ => {},
					};
				} else if key == Key::LeftControl {
					let scene = vpb::gmuc_ref!(scene);
					match action {
						Action::Press => { scene.input_state.control = true; },
						Action::Release => { scene.input_state.control = false; },
						_ => {},
					};
				} else if key == Key::LeftAlt {
					let scene = vpb::gmuc_ref!(scene);
					match action {
						Action::Press => { scene.input_state.alt = true; },
						Action::Release => { scene.input_state.alt = false; },
						_ => {},
					};
				}
			},
			glfw::WindowEvent::Scroll(x, y) => {
				let scene = vpb::gmuc_ref!(scene);
				scene.input_state.mouse.scroll_delta = y as i32;
			},
			glfw::WindowEvent::MouseButton(button, action, modifiers) => {
				let scene = vpb::gmuc_ref!(scene);
				match button {
					MouseButton::Button1 => {
						match action {
							Action::Press => { scene.input_state.mouse.left = true; },
							Action::Release => {
								scene.input_state.mouse.left = false;
								scene.input_state.mouse.last_left = Instant::now();
							},
							_ => {},
						};
					},
					MouseButton::Button2 => {
						match action {
							Action::Press => { scene.input_state.mouse.right = true; },
							Action::Release => { scene.input_state.mouse.right = false; },
							_ => {},
						};
					},
					MouseButton::Button3 => {
						match action {
							Action::Press => { scene.input_state.mouse.middle = true; },
							Action::Release => { scene.input_state.mouse.middle = false; },
							_ => {},
						};
					},
					_ => {},
				};
			},
			glfw::WindowEvent::CursorPos(x, y) => {
				let scene = vpb::gmuc_ref!(scene);
				scene.input_state.mouse.position.x = x as i32;
				scene.input_state.mouse.position.y = y as i32;
			},
			_ => {},
		};
		TickResult::CONTINUE
	}
}
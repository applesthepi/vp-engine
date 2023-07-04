use std::{sync::Arc, marker::PhantomData, rc::Rc, borrow::Borrow, cell::RefCell, fs::File, io::Read};

use ash::vk::{Instance, self};
use nalgebra::vector;
use shaderc::{ShaderKind, CompileOptions};
use winit::{event_loop::{ControlFlow, EventLoop}, event::{Event, WindowEvent, KeyboardInput, ElementState, VirtualKeyCode, MouseButton}};

use crate::{Scene, pipelines::ui::PipelineUI, EnginePipeline};

mod macros;
pub use macros::*;

pub struct Program {
	pub scene: Arc<Scene>,
	pub program_data: ProgramData,
}

#[derive(Clone)]
pub struct ProgramData {
	pub window: Arc<vpb::Window>,
	pub instance: Arc<vpb::Instance>,
	pub surface: Arc<vpb::Surface>,
	pub device: Arc<vpb::Device>,
	pub swapchain: Arc<vpb::Swapchain>,
	pub render_pass: Arc<vpb::RenderPass>,
	pub descriptor_pool: Arc<vpb::DescriptorPool>,
	pub command_pool: Arc<vpb::CommandPool>,
	pub command_buffer_setup: Arc<vpb::CommandBuffer>,
	pub command_buffer_draw: Arc<vpb::CommandBuffer>,
	pub shader_loader: Arc<vpb::ShaderLoader>,
	pub frame_count: usize,
}

impl Program {
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
			3, // Use 3 frames for max descriptor sets.
		);
		let shader_loader = vpb::ShaderLoader::new();
		let mut program_data = ProgramData {
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
		);
		program_data.frame_count = frame_count;
		Self {
			scene: Arc::new(scene),
			program_data,
		}
	}

	pub fn run<FO, FC, FR>(
		mut program_data: Arc<ProgramData>,
		mut scene: Arc<Scene>,
		event_loop: EventLoop<()>,
		mut fn_open: FO,
		mut fn_close: FC,
		mut fn_render: FR,
	) where
	FO: FnMut(&mut Scene) + 'static,
	FC: FnMut(&mut Scene) + 'static,
	FR: FnMut(&mut Scene) + 'static { unsafe {
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
					&mut program_data,
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
					&mut fn_render,
					&mut fn_close,
				);
			}
		);
	}}

	fn event_match<FC, FR>(
		// program: Rc<Program>,
		program_data: &mut ProgramData,
		scene: &mut Scene,
		// mut self,
		event: &Event<()>,
		control_flow: &mut ControlFlow,
		// scene: &mut Arc<Scene>,
		// program
		fn_render: &mut FR,
		fn_close: &mut FC,
	) where
	FC: FnMut(&mut Scene) + 'static,
	FR: FnMut(&mut Scene) + 'static { unsafe {
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
				let wa_window = vpb::gmuc!(program_data.window);
				wa_window.extent = vk::Extent2D {
					width: size.width,
					height: size.height,
				};
				scene.resize(
					program_data,
				);
			}
			Event::MainEventsCleared => {
				fn_render(scene);
				scene.render();
			},
			Event::WindowEvent { event: WindowEvent::KeyboardInput { input, .. }, .. } => {
				let value = match input.state {
					ElementState::Pressed => true,
					ElementState::Released => false,
				};
				if let Some(x) = input.virtual_keycode {
					scene.input_state.down_keys[x as usize] = value;
				}
			},
			Event::WindowEvent { event: WindowEvent::MouseInput { device_id, state, button, modifiers }, .. } =>  {
				match button {
					MouseButton::Left => {
						match state {
							ElementState::Pressed => {
								scene.input_state.mouse.left = true;
								// wa_program.user_io.init_rotation_vector = wa_program.user_io.camera_rotation;
								// wa_program.user_io.init_mouse = wa_program.user_io.mouse_position;
								// wa_program.user_io.mouse_down = true;
							},
							ElementState::Released => {
								scene.input_state.mouse.left = false;
								// wa_program.user_io.mouse_down = false;
							}
						}
					},
					MouseButton::Middle => {
						match state {
							ElementState::Pressed => {
								scene.input_state.mouse.middle = true;
							},
							ElementState::Released => {
								scene.input_state.mouse.middle = false;
							}
						}
					},
					MouseButton::Right => {
						match state {
							ElementState::Pressed => {
								scene.input_state.mouse.right = true;
							},
							ElementState::Released => {
								scene.input_state.mouse.right = false;
							}
						}
					},
					_ => {}
				}
			},
			Event::WindowEvent { event: WindowEvent::CursorMoved { device_id, position, modifiers }, .. } => {
				scene.input_state.mouse.position = vector![position.x as i32, position.y as i32];
			},
			_ => {},
		}
	}}
}

impl ProgramData {
	pub fn load_shader(
		&self,
		shader_kind: ShaderKind,
		name: &str,
	) -> vk::ShaderModule { unsafe {
		let options = CompileOptions::new().unwrap();
		let glsl_path = ("res/shaders/".to_string() + name) + match shader_kind {
			ShaderKind::Vertex => ".vert",
			ShaderKind::Fragment => ".frag",
			ShaderKind::Compute => ".comp",
			_ => { panic!("not impl"); }
		};
		let glsl_path = glsl_path.as_str();
		let spv_path = ("res/shaders/".to_string() + name) + ".spv";
		let spv_path = spv_path.as_str();
		let mut file = File::open(glsl_path).expect(
			format!("shader \"{}\" does not exist", glsl_path).as_str()
		);
		let mut text: String = String::with_capacity(1024);
		file.read_to_string(&mut text).unwrap();
		let binary_artifact = self.shader_loader.compiler.compile_into_spirv(
			text.as_str(),
			shader_kind,
			glsl_path, "main",
			Some(&options),
		).expect(format!("failed to compile \"{}\"", glsl_path).as_str());
		debug_assert_eq!(Some(&0x07230203), binary_artifact.as_binary().first());
		// let text_artifact = shader_loader.compiler.compile_into_spirv_assembly(
		// 	text.as_str(),
		// 	shader_kind,
		// 	glsl_path, "main",
		// 	Some(&shader_loader.options),
		// ).expect(format!("failed to compile \"{}\"", glsl_path).as_str());
		// debug_assert!(text_artifact.as_text().starts_with("; SPIR-V\n"));
		// let mut spv_file = File::open(spv_path).unwrap();
		// let spv_text = read_spv(&mut spv_file).unwrap();
	
		let spv_text = binary_artifact.as_binary();
	
		let shader_info = vk::ShaderModuleCreateInfo::builder()
			.code(spv_text)
			.build();
		self.device.device.create_shader_module(
			&shader_info,
			None,
		).unwrap()
	}}
}
use std::{sync::Arc, marker::PhantomData, rc::Rc, borrow::Borrow, cell::RefCell, fs::File};

use ash::vk::{Instance, self};
use shaderc::{ShaderKind, CompileOptions};
use winit::{event_loop::{ControlFlow, EventLoop}, event::{Event, WindowEvent, KeyboardInput, ElementState, VirtualKeyCode}};

use crate::Scene;

mod macros;
pub use macros::*;

pub struct Program<'a> {
	pub scene: Arc<Scene<'a>>,
	pub data: ProgramData,
}

#[derive(Clone)]
pub struct ProgramData {
	pub window: Arc<vpb::Window>,
	pub instance: Arc<vpb::Instance>,
	pub surface: Arc<vpb::Surface>,
	pub device: Arc<vpb::Device>,
	pub swapchain: Arc<vpb::Swapchain>,
	pub command_pool: Arc<vpb::CommandPool>,
	pub command_buffer_setup: Arc<vpb::CommandBuffer>,
	pub command_buffer_draw: Arc<vpb::CommandBuffer>,
	pub shader_loader: Arc<vpb::ShaderLoader>,
}

impl<'a> Program<'a> {
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
		let command_buffer_draw = vpb::CommandBuffer::new(
			&mut device,
			&mut command_pool,
			&swapchain,
		);
		let shader_loader = vpb::ShaderLoader::new();
		let program_data = ProgramData {
			window,
			instance,
			surface,
			device,
			swapchain,
			command_pool,
			command_buffer_draw,
			shader_loader,
		};
		let scene = Scene::new(
			device,
			command_buffer,
			&mut command_pool,
			renderpass,
			&surface,
			swapchain,
			&window,
			&instance,
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
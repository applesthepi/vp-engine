use std::{mem::size_of, sync::Arc, f32::consts::{PI, FRAC_PI_2}};

use ash::vk;
use nalgebra::{Matrix4, vector, Vector3, Vector2, Perspective3};
use winit::event::VirtualKeyCode;

use crate::{ProgramData, InputState, RenderState};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BlockCamera {
	pub view: Matrix4<f32>,
	pub projection: Matrix4<f32>,
}

impl Default for BlockCamera {
	fn default() -> Self {
		Self {
			view: Matrix4::identity(),
			projection: Matrix4::identity(),
		}
	}
}

impl vpb::Block for BlockCamera {
	fn create_block_state(
		device: &vpb::Device,
		instance: &vpb::Instance,
		descriptor_pool: &vk::DescriptorPool,
		descriptor_set_layout: &vk::DescriptorSetLayout,
		frame_count: usize,
		binding: u32,
		set: u32,
	) -> Arc<vpb::BlockState> {
		Arc::new(vpb::BlockState::new(
			device,
			instance,
			descriptor_pool,
			descriptor_set_layout,
			frame_count,
			binding,
			set,
			size_of::<BlockCamera>(),
			1,
		))
	}

	fn create_descriptor_set_layout(
		device: &Arc<vpb::Device>,
		binding: u32,
	) -> vk::DescriptorSetLayout { unsafe {
		let descriptor_set_layout_binding = vk::DescriptorSetLayoutBinding::builder()
			.binding(binding)
			.descriptor_type(vk::DescriptorType::UNIFORM_BUFFER)
			.stage_flags(vk::ShaderStageFlags::VERTEX)
			.descriptor_count(1)
			.build();
		let descriptor_set_layout_info = vk::DescriptorSetLayoutCreateInfo::builder()
			.bindings(&[
				descriptor_set_layout_binding,
			]).build();
		device.device.create_descriptor_set_layout(
			&descriptor_set_layout_info,
			None,
		).unwrap()
	}}
}

#[derive(Default, Debug)]
pub struct CameraState {
	pub block: BlockCamera,
	pub camera_preposition: Vector3<f32>,
	pub camera_postposition: Vector3<f32>,
	pub camera_rotation: Vector2<f32>,
	pub init_rotation_vector: Vector2<f32>,
	pub init_mouse: Vector2<i32>,
	pub was_down: bool,
}

impl CameraState {
	pub fn new(
		position: [f32; 3],
	) -> Self {
		Self {
			camera_preposition: vector![
				position[0],
				position[1],
				position[2]
			],
			camera_postposition: vector![
				position[0],
				position[1],
				position[2]
			],
			camera_rotation: vector![
				0.0,
				PI
			],
			..Default::default()
		}
	}

	pub fn build_perspective(
		&mut self,
		program_data: &ProgramData,
	) {
		self.block.projection = *Perspective3::new(
			program_data.window.extent.width as f32 /
			program_data.window.extent.height as f32,
			90.0,
			0.1, 10_000.0,
		).as_matrix();
		self.block.projection[(1, 1)] *= -1.0;
	}

	pub fn build_view(
		&mut self,
		program_data: &ProgramData,
		input_state: &InputState,
		render_state: &RenderState,
	) {
		if input_state.mouse.middle && !self.was_down {
			self.was_down = true;
			self.init_rotation_vector = self.camera_rotation;
			self.init_mouse = input_state.mouse.position;
		} else if !input_state.mouse.middle && self.was_down {
			self.was_down = false;
		}
		let mut move_speed: f32 = 150.0;
		let move_snappiness: f32 = 17.0;
		let r_x_cam = Matrix4::new_rotation(
			vector![0.0, self.camera_rotation.x, 0.0]
		);
		let r_y_cam = Matrix4::new_rotation(
			vector![self.camera_rotation.y, 0.0, 0.0]
		);
		let r_x_rot = Matrix4::new_rotation(
			vector![0.0, -self.camera_rotation.x, 0.0]
		);
		let r_y_rot = Matrix4::new_rotation(
			vector![PI - self.camera_rotation.y, 0.0, 0.0]
		);
		let rotation_cam = r_y_cam * r_x_cam;
		let rotation_rot = r_x_rot * r_y_rot;
		if input_state.down_keys[VirtualKeyCode::LShift as usize] {
			move_speed *= 2.0;
		}
		if input_state.down_keys[VirtualKeyCode::W as usize] {
			let direction = rotation_rot.transform_vector(
				&Vector3::z()
			).scale(render_state.delta_time * move_speed);
			self.camera_preposition += direction;
		}
		if input_state.down_keys[VirtualKeyCode::S as usize] {
			let direction = rotation_rot.transform_vector(
				&-Vector3::z()
			).scale(render_state.delta_time * move_speed);
			self.camera_preposition += direction;
		}
		if input_state.down_keys[VirtualKeyCode::A as usize] {
			let direction = rotation_rot.transform_vector(
				&-Vector3::x()
			).scale(render_state.delta_time * move_speed);
			self.camera_preposition += direction;
		}
		if input_state.down_keys[VirtualKeyCode::D as usize] {
			let direction = rotation_rot.transform_vector(
				&Vector3::x()
			).scale(render_state.delta_time * move_speed);
			self.camera_preposition += direction;
		}
		if input_state.down_keys[VirtualKeyCode::Q as usize] {
			let direction = Vector3::y().scale(
				render_state.delta_time * move_speed
			);
			self.camera_preposition += direction;
		}
		if input_state.down_keys[VirtualKeyCode::E as usize] {
			let direction = -Vector3::y().scale(
				render_state.delta_time * move_speed
			);
			self.camera_preposition += direction;
		}
		if input_state.mouse.middle {
			let delta_mouse = [
				self.init_mouse[0] - input_state.mouse.position[0],
				self.init_mouse[1] - input_state.mouse.position[1],
			];
			// println!("{:.3}, {:.3}", delta_mouse[0], delta_mouse[1]);
			let vr_delta: Vector2<f32> = vector![delta_mouse[0] as f32 * 0.005, -delta_mouse[1] as f32 * 0.005];
			self.camera_rotation = self.init_rotation_vector + vr_delta;
			self.camera_rotation.y = self.camera_rotation.y.max(FRAC_PI_2).min(PI + FRAC_PI_2);
		}
		self.camera_postposition = self.camera_postposition.lerp(&self.camera_preposition, (render_state.delta_time * move_snappiness).min(1.0));
		let translation = Matrix4::new_translation(&vector![
			-self.camera_postposition.x,
			-self.camera_postposition.y,
			-self.camera_postposition.z
		]);
		let view_matrix = rotation_cam * translation;
		self.block.view = view_matrix;
	}
}
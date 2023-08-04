use std::{mem::size_of, sync::Arc, f32::consts::{PI, FRAC_PI_2}};

use ash::vk;
use glfw::ffi;
use nalgebra::{Matrix4, vector, Vector3, Vector2, Perspective3};
use vpb::{DDType, DDTypeUniform, DescriptorDescription, BindingId, ProgramData};

use crate::{InputState, RenderState, Camera};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BlockCamera3d {
	pub view: Matrix4<f32>,
	pub projection: Matrix4<f32>,
}

impl Default for BlockCamera3d {
	fn default() -> Self {
		Self {
			view: Matrix4::identity(),
			projection: Matrix4::identity(),
		}
	}
}

impl vpb::Block for BlockCamera3d {
	fn create_block_state(
		program_data: &ProgramData,
		descriptor_set_layout: &vk::DescriptorSetLayout,
		frame_count: usize,
		binding: BindingId,
		set: vpb::SetId,
	) -> Arc<vpb::BlockState> {
		Arc::new(vpb::BlockState::new(
			program_data,
			descriptor_set_layout,
			frame_count,
			set,
			DescriptorDescription::new(&[
				DDType::Uniform(DDTypeUniform {
					binding,
					size: size_of::<BlockCamera3d>(),
				}),
			]),
		))
	}

	fn create_descriptor_set_layout(
		device: &Arc<vpb::Device>,
		binding: BindingId,
	) -> vk::DescriptorSetLayout { unsafe {
		let descriptor_set_layout_binding = vk::DescriptorSetLayoutBinding::builder()
			.binding(binding.0)
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
pub struct CameraState3d {
	pub block: BlockCamera3d,
	pub camera_preposition: Vector3<f32>,
	pub camera_postposition: Vector3<f32>,
	pub camera_rotation: Vector2<f32>,
	pub init_rotation_vector: Vector2<f32>,
	pub init_mouse: Vector2<i32>,
	pub was_down: bool,
}

impl CameraState3d {
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
}

impl Camera for CameraState3d {
	fn build_perspective(
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

	fn build_view(
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
		if input_state.down_keys[ffi::KEY_LEFT_SHIFT as usize] {
			move_speed *= 2.0;
		}
		if input_state.down_keys[ffi::KEY_W as usize] {
			let direction = rotation_rot.transform_vector(
				&Vector3::z()
			).scale(render_state.delta_time * move_speed);
			self.camera_preposition += direction;
		}
		if input_state.down_keys[ffi::KEY_S as usize] {
			let direction = rotation_rot.transform_vector(
				&-Vector3::z()
			).scale(render_state.delta_time * move_speed);
			self.camera_preposition += direction;
		}
		if input_state.down_keys[ffi::KEY_A as usize] {
			let direction = rotation_rot.transform_vector(
				&-Vector3::x()
			).scale(render_state.delta_time * move_speed);
			self.camera_preposition += direction;
		}
		if input_state.down_keys[ffi::KEY_D as usize] {
			let direction = rotation_rot.transform_vector(
				&Vector3::x()
			).scale(render_state.delta_time * move_speed);
			self.camera_preposition += direction;
		}
		if input_state.down_keys[ffi::KEY_Q as usize] {
			let direction = Vector3::y().scale(
				render_state.delta_time * move_speed
			);
			self.camera_preposition += direction;
		}
		if input_state.down_keys[ffi::KEY_E as usize] {
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

	fn update(
		&self,
		device: &vpb::Device,
		frame: Option<usize>,
		block_state: &Arc<vpb::BlockState>,
	) {
		block_state.update(
			device,
			&self.block,
			frame,
		)
	}

	fn get_view(
		&self,
	) -> (Matrix4<f32>, f32) {
		(self.block.view, 1.0)
	}
}
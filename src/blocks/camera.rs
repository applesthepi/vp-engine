use std::{mem::size_of, sync::Arc, f32::consts::{PI, FRAC_PI_2}};

use ash::vk;
use nalgebra::{Matrix4, vector, Vector3, Vector2};
use winit::event::VirtualKeyCode;

use crate::{ProgramData, InputState, RenderState};

#[repr(C)]
#[derive(Copy, Clone)]
pub struct BlockCamera {
	pub view: nalgebra_glm::Mat4,
	pub projection: nalgebra_glm::Mat4,
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

#[derive(Default)]
pub struct CameraState {
	pub camera_preposition: Vector3<f32>,
	pub camera_postposition: Vector3<f32>,
	pub camera_rotation: Vector2<f32>,
	pub init_rotation_vector: Vector2<f32>,
	pub init_mouse: [i32; 2],
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
}

pub fn camera_view_matrix(
	program_data: &ProgramData,
	input_state: &InputState,
	render_state: &RenderState,
	camera_state: &mut CameraState,
) -> [f32; 16] {
	if input_state.mouse.middle && !camera_state.was_down {
		camera_state.was_down = true;
		camera_state.init_rotation_vector = camera_state.camera_rotation;
		camera_state.init_mouse = input_state.mouse.position;
	} else if !input_state.mouse.middle && camera_state.was_down {
		camera_state.was_down = false;
	}
	let mut move_speed: f32 = 150.0;
	let move_snappiness: f32 = 17.0;
	let r_x_cam = Matrix4::new_rotation(
		vector![0.0, camera_state.camera_rotation.x, 0.0]
	);
	let r_y_cam = Matrix4::new_rotation(
		vector![camera_state.camera_rotation.y, 0.0, 0.0]
	);
	let r_x_rot = Matrix4::new_rotation(
		vector![0.0, -camera_state.camera_rotation.x, 0.0]
	);
	let r_y_rot = Matrix4::new_rotation(
		vector![PI - camera_state.camera_rotation.y, 0.0, 0.0]
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
		camera_state.camera_preposition += direction;
	}
	if input_state.down_keys[VirtualKeyCode::S as usize] {
		let direction = rotation_rot.transform_vector(
			&-Vector3::z()
		).scale(render_state.delta_time * move_speed);
		camera_state.camera_preposition += direction;
	}
	if input_state.down_keys[VirtualKeyCode::A as usize] {
		let direction = rotation_rot.transform_vector(
			&-Vector3::x()
		).scale(render_state.delta_time * move_speed);
		camera_state.camera_preposition += direction;
	}
	if input_state.down_keys[VirtualKeyCode::D as usize] {
		let direction = rotation_rot.transform_vector(
			&Vector3::x()
		).scale(render_state.delta_time * move_speed);
		camera_state.camera_preposition += direction;
	}
	if input_state.down_keys[VirtualKeyCode::Q as usize] {
		let direction = Vector3::y().scale(
			render_state.delta_time * move_speed
		);
		camera_state.camera_preposition += direction;
	}
	if input_state.down_keys[VirtualKeyCode::E as usize] {
		let direction = -Vector3::y().scale(
			render_state.delta_time * move_speed
		);
		camera_state.camera_preposition += direction;
	}
	if input_state.mouse.middle {
		let delta_mouse = [
			camera_state.init_mouse[0] - input_state.mouse.position[0],
			camera_state.init_mouse[1] - input_state.mouse.position[1],
		];
		println!("{:.3}, {:.3}", delta_mouse[0], delta_mouse[1]);
		let vr_delta: Vector2<f32> = vector![delta_mouse[0] as f32 * 0.005, -delta_mouse[1] as f32 * 0.005];
		camera_state.camera_rotation = camera_state.init_rotation_vector + vr_delta;
		camera_state.camera_rotation.y = camera_state.camera_rotation.y.max(FRAC_PI_2).min(PI + FRAC_PI_2);
	}
	camera_state.camera_postposition = camera_state.camera_postposition.lerp(&camera_state.camera_preposition, (render_state.delta_time * move_snappiness).min(1.0));
	let translation = Matrix4::new_translation(&vector![
		-camera_state.camera_postposition.x,
		-camera_state.camera_postposition.y,
		-camera_state.camera_postposition.z
	]);
	(rotation_cam * translation).as_slice().try_into().unwrap()
}
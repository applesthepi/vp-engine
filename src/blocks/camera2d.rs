use std::{mem::size_of, sync::Arc, f32::consts::{PI, FRAC_PI_2}, collections::{HashSet, BTreeMap}};

use ash::vk;
use bytemuck::{Zeroable, Pod};
use nalgebra::{Matrix4, vector, Vector3, Vector2, Perspective3, Orthographic3, point, Point3};
use vpb::{BindingId, DDType, DescriptorDescription, DDTypeUniform, ProgramData};
use crate::{InputState, RenderState, Camera};

#[repr(C)]
#[derive(Debug, Copy, Clone, Pod, Zeroable)]
pub struct BlockCamera2d {
	pub view: [f32; 16],
	pub projection: [f32; 16],
}

impl Default for BlockCamera2d {
	fn default() -> Self {
		Self {
			view: Matrix4::identity().as_slice().try_into().unwrap(),
			projection: Matrix4::identity().as_slice().try_into().unwrap(),
		}
	}
}

impl vpb::Block for BlockCamera2d {
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
			DescriptorDescription::new(vec![
				DDType::Uniform(DDTypeUniform {
					binding,
					size: size_of::<BlockCamera2d>(),
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
pub struct CameraState2d {
	pub block: BlockCamera2d,
	pub camera_init_preposition: Vector2<f32>,
	pub camera_preposition: Vector2<f32>,
	pub camera_postposition: Vector2<f32>,
	pub init_mouse: Vector2<i32>,
	pub was_down: bool,
	pub predistance_index: i16,
	pub predistance: f64,
	pub postdistance: f64,
	pub dragging_distance: f64,
	// internal settings
	pub step_factor_exp: f32,
	pub step_factor_mul: f32,
	pub max_zoom: f32,
}

impl CameraState2d {
	pub fn new(
		position: Vector2<f32>,
		// TODO: pass these as parameters from toml settings
	) -> Self {
		// max_zoom: (0.0, inf)
		// tile px width of max zoom
		let max_zoom: f32 = 200.0;
		// predistance_index: [0, inf)
		// how many scrolls away from 0 (per tile max zoom).
		let predistance_index: i16 = 2;
		// step_factor_exp: (1.0, inf) EXCLUSIVE
		// 2.0 - squares every scroll
		// >1.0 - little exponential change
		let step_factor_exp: f32 = 1.5;

		// step_factor_mul: (0.0, inf) EXCLUSIVE
		// 2.0 - doubles every scroll
		// 1.0 - no multiplicitive change
		// (0.0, 1.0) EXCLUSIVE - linear reduction in change
		let step_factor_mul: f32 = 2.0;

		let predistance: f64 = CameraState2d::calc_predistance(
			predistance_index,
			step_factor_exp,
			step_factor_mul,
		);
		Self {
			camera_preposition: position,
			camera_postposition: position,
			predistance_index,
			predistance,
			postdistance: predistance,
			step_factor_exp,
			step_factor_mul,
			max_zoom,
			..Default::default()
		}
	}

	fn calc_predistance(
		index: i16,
		step_factor_exp: f32,
		step_factor_mul: f32,
	) -> f64 {
		(step_factor_exp as f64).powf(index as f64) * (step_factor_mul - step_factor_mul + 1.0) as f64
	}
}

impl Camera for CameraState2d {
	fn build_perspective(
		&mut self,
		program_data: &ProgramData,
	) {
		let data = *Orthographic3::new(
			0.0,
			program_data.window.extent.width as f32,
			0.0,
			program_data.window.extent.height as f32,
			-1_000.0,
			1_000.0,
		).as_matrix();
		self.block.projection = data.as_slice().try_into().unwrap();
	}

	fn build_view(
		&mut self,
		program_data: &ProgramData,
		input_state: &InputState,
		render_state: &RenderState,
	) {
		// STATE CHANGE

		if input_state.mouse.middle && !self.was_down {
			self.was_down = true;
			self.init_mouse = input_state.mouse.position.into();
			self.camera_init_preposition = self.camera_preposition;
			self.dragging_distance = self.predistance;
		} else if !input_state.mouse.middle && self.was_down {
			self.was_down = false;
		}

		// PRE STATE

		let translate_scale_static = self.max_zoom / (self.dragging_distance as f32);
		let translate_scale_dynamic = self.max_zoom / (self.postdistance as f32);
		if input_state.mouse.middle {
			let delta_position = input_state.mouse.position - self.init_mouse;
			self.camera_preposition = self.camera_init_preposition + vector![
				(delta_position.x as f32) / translate_scale_static,
				(delta_position.y as f32) / translate_scale_static
			];
		}
		self.predistance_index -= input_state.mouse.scroll_delta as i16;
		self.predistance_index = self.predistance_index.max(0);
		self.predistance = CameraState2d::calc_predistance(
			self.predistance_index,
			self.step_factor_exp,
			self.step_factor_mul,
		);
		
		// POST STATE

		// let t = 0.5 * render_state.delta_time;
		// self.camera_postposition = self.camera_preposition.lerp(&self.camera_postposition, t);
		self.camera_postposition = self.camera_preposition;

		let t = (20.0 * render_state.delta_time as f64).min(1.0);
		let omt = 1.0 - t;
		self.postdistance = (self.predistance * t) + (self.postdistance * omt);

		// VIEW

		let h_width = program_data.window.extent.width as f32 / translate_scale_dynamic * 0.5;
		let h_height = program_data.window.extent.height as f32 / translate_scale_dynamic * 0.5;
		let view: Matrix4<f32> = Matrix4::new_translation(&vector![
			h_width,
			h_height,
			0.0
		]);
		let view: Matrix4<f32> = view.append_translation(&vector![
			self.camera_postposition.x,
			self.camera_postposition.y,
			0.0
		]);
		let view: Matrix4<f32> = view.append_scaling(translate_scale_dynamic);
		self.block.view = view.as_slice().try_into().unwrap();
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
		let translate_scale_dynamic = self.max_zoom / (self.postdistance as f32);
		(Matrix4::<f32>::from_column_slice(&self.block.view), translate_scale_dynamic)
	}
}
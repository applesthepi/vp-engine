use std::{sync::Arc, marker::PhantomData, borrow::Borrow, cell::RefCell};

use ash::vk;
use bytemuck::bytes_of;
use nalgebra::{Orthographic3, Matrix4};

use crate::{ViewportDepthRange, create_graphics_pipeline, PipelineInfo, BlockCamera, ProgramData, BlockModel, EnginePipeline};

pub struct PipelineSimple<V: vpb::Vertex> {
	vertex: PhantomData<V>,
	pub pipeline: vk::Pipeline,
	pub pipeline_layout: vk::PipelineLayout,
	pub viewport: [vk::Viewport; 1],
	pub scissor: [vk::Rect2D; 1],
	pub descriptor_set_layouts: Vec<vk::DescriptorSetLayout>,
	pub block_state: Arc<vpb::BlockState>,
}

impl<V: vpb::Vertex> EnginePipeline for PipelineSimple<V> {
	fn create_object_block_states(
		&self,
		program_data: &ProgramData,
	) -> Vec<Arc<vpb::BlockState>> {
		vec![
			self.block_state.clone()
			// TODO: CREATE NEW MODEL BLOCK STATE
		]
	}

	fn recreate_block_states(
		&mut self,
		program_data: &ProgramData,
	) {
		let block_state = vpb::gmuc!(self.block_state);
		block_state.recreate_memory(
			&program_data.device,
			&program_data.instance,
			&program_data.descriptor_pool.descriptor_pool,
			program_data.frame_count,
		);
	}

	fn recreate_pipeline(
		&mut self,
		program_data: &ProgramData,
	) { unsafe {
		let (
			pipeline,
			pipeline_layout,
			viewport,
			scissor
		) = create_graphics_pipeline::<V>(
			program_data,
			"ui_lighting",
			PipelineInfo {
				depth: true,
				viewport_depth_range: ViewportDepthRange::UI,
				polygon_mode: vk::PolygonMode::FILL,
				descriptor_set_layouts: &self.descriptor_set_layouts,
			},
		);
		self.pipeline = pipeline;
		self.pipeline_layout = pipeline_layout;
		self.viewport = viewport;
		self.scissor = scissor;
	}}
}

impl<V: vpb::Vertex> PipelineSimple<V> {
	pub fn new(
		program_data: &ProgramData,
	) -> Self { unsafe {
		let mut descriptor_set_layouts = vec![
			BlockCamera::create_descriptor_set_layout(&program_data.device),
		];
		let (
			pipeline,
			pipeline_layout,
			viewport,
			scissor
		) = create_graphics_pipeline::<V>(
			program_data,
			"ui_lighting",
			PipelineInfo {
				depth: true,
				viewport_depth_range: ViewportDepthRange::UI,
				polygon_mode: vk::PolygonMode::FILL,
				descriptor_set_layouts: &descriptor_set_layouts,
			},
		);
		let block_state = Arc::new(BlockCamera::create_block_state(
			program_data,
			&descriptor_set_layouts[0],
			0, 0,
		));
		Self {
			vertex: PhantomData,
			pipeline,
			pipeline_layout,
			viewport,
			scissor,
			block_state,
			descriptor_set_layouts,
		}
	}}
}

impl<V: vpb::Vertex> vpb::Pipeline for PipelineSimple<V> {
	fn get_viewport(&self) -> [vk::Viewport; 1] {
		self.viewport
	}
	
	fn get_scissor(&self) -> [vk::Rect2D; 1] {
		self.scissor
	}
	
	fn destroy_block_state_memory(
		&mut self,
		device: &Arc<vpb::Device>,
	) {
		let block_state = vpb::gmuc!(self.block_state);
		block_state.destroy_memory(device);
	}
	
	fn destroy_pipeline(
		&mut self,
		device: &Arc<vpb::Device>,
	) { unsafe {
		device.device.destroy_pipeline(
			self.pipeline,
			None,
		);
		device.device.destroy_pipeline_layout(
			self.pipeline_layout,
			None,
		);
	}}
	
	fn get_pipeline(&self) -> vk::Pipeline {
		self.pipeline
	}
	
	fn get_pipeline_layout(&self) -> vk::PipelineLayout {
		self.pipeline_layout
	}

	fn get_block(&self) -> Arc<vpb::BlockState> {
		self.block_state.clone()
	}
	
	fn bind_block(
		&mut self,
		device: &vpb::Device,
		command_buffer: &vk::CommandBuffer,
		frame: usize,
	) { unsafe {
		device.device.cmd_bind_descriptor_sets(
			*command_buffer,
			vk::PipelineBindPoint::GRAPHICS,
			self.pipeline_layout,
			0,
			&[self.block_state.frame_sets[frame].set],
			&[],
		);
	}}
	
	fn destroy_set_layout(
		&mut self,
		device: &vpb::Device,
	) { unsafe {
		device.device.destroy_descriptor_set_layout(
			self.block_state.layout,
			None,
		);
		// TODO: bucket destroys object models.
	}}
	
	fn update_blocks(
		&mut self,
		device: &vpb::Device,
		extent: &vk::Extent2D,
		frame: usize,
	) {
		let view = nalgebra_glm::look_at(
			&nalgebra_glm::vec3(0.0, 0.0, 1.0),
			&nalgebra_glm::vec3(0.0, 0.0, 0.0),
			&nalgebra_glm::vec3(0.0, 1.0, 0.0),
		);
		let projection = nalgebra_glm::ortho(
			0.0,
			extent.width as f32,
			0.0,
			extent.height as f32,
			-100.0,
			100.0,
		);
		// let mut projection = nalgebra_glm::perspective(
		// 	16.0 / 9.0,
		// 	90.0,
		// 	0.1,
		// 	100.0,
		// );
		// projection[(1, 1)] *= -1.0;
		let camera_block = BlockCamera {
			view,
			projection,
		};

		// let orthographic = nalgebra_glm::ortho(
		// 	0.0,
		// 	100.0,
		// 	100.0,
		// 	0.0,
		// 	-100.0,
		// 	100.0,
		// );
		// let camera_block = BlockCamera {
		// 	view: Dim::value(&nalgebra_glm::identity()).,
		// 	projection: orthographic.as_slice(),
		// }
		self.block_state.update(
			device,
			&camera_block,
			Some(frame),
		)
	}

	fn object_binding_set(
		&self,
	) -> Vec<(u32, u32)> {
		vec![
			(1, 0),
			(1, 1),
		]
	}
}
use std::{sync::Arc, marker::PhantomData, borrow::Borrow, cell::RefCell};

use ash::vk;
use bytemuck::bytes_of;
use nalgebra::{Matrix4, Perspective3, Orthographic3};

use crate::{ViewportDepthRange, create_graphics_pipeline, PipelineInfo, BlockCamera, ProgramData, BlockModel, EnginePipeline};

pub struct PipelineSimple<V: vpb::Vertex> {
	vertex: PhantomData<V>,
	pub descriptor_pool: vk::DescriptorPool,
	pub pipeline: vk::Pipeline,
	pub pipeline_layout: vk::PipelineLayout,
	pub viewport: [vk::Viewport; 1],
	pub scissor: [vk::Rect2D; 1],
	pub descriptor_set_layouts: Vec<vk::DescriptorSetLayout>,
	pub block_state: Arc<vpb::BlockState>,
}

impl<V: vpb::Vertex> EnginePipeline for PipelineSimple<V> {
	fn create_block_states(
			&self,
			program_data: &ProgramData,
			descriptor_pool: &vk::DescriptorPool,
		) -> Vec<Arc<vpb::BlockState>> {
		vec![
			self.block_state.clone(),
			// Arc::new(BlockModel::create_block_state(
			// 	program_data,
			// 	descriptor_pool,
			// 	&self.descriptor_set_layouts[1],
			// 	1,
			// 	1,
			// )),
		]
	}
}

impl<V: vpb::Vertex> PipelineSimple<V> {
	pub fn new(
		program_data: &ProgramData,
		render_pass: &vpb::RenderPass,
	) -> Self { unsafe {
		let descriptor_pool_max = 64 * program_data.frame_count as u32;
		let descriptor_pool_size = vk::DescriptorPoolSize::builder()
			.descriptor_count(descriptor_pool_max)
			.build();
		let descriptor_pool_info = vk::DescriptorPoolCreateInfo::builder()
			.pool_sizes(&[descriptor_pool_size])
			.max_sets(descriptor_pool_max)
			.build();
		let descriptor_pool = program_data.device.device.create_descriptor_pool(
			&descriptor_pool_info,
			None,
		).unwrap();
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
			render_pass,
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
			&descriptor_pool,
			&descriptor_set_layouts[0],
			0, 0,
		));
		Self {
			vertex: PhantomData,
			descriptor_pool,
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
	fn get_pipeline(&self) -> vk::Pipeline {
		self.pipeline
	}
	fn get_pipeline_layout(&self) -> vk::PipelineLayout {
		self.pipeline_layout
	}
	fn get_descriptor_pool(&self) -> vk::DescriptorPool {
		self.descriptor_pool
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
			&[self.block_state.descriptor_buffers[frame].set],
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
		command_buffer: &vk::CommandBuffer,
		frame: usize,
	) { unsafe {
		let orthographic: Orthographic3<f32> = Orthographic3::new(
			0.0,
			100.0,
			100.0,
			0.0,
			-100.0,
			100.0,
		);
		let camera_block = BlockCamera {
			view: Matrix4::identity().as_slice().try_into().unwrap(),
			projection: orthographic.as_matrix().as_slice().try_into().unwrap(),
		};
		self.block_state.update(
			device,
			command_buffer,
			&camera_block,
			Some(frame),
		)
	}}

	fn object_binding_set(
		&self,
	) -> Vec<(u32, u32)> {
		vec![
			(1, 0),
			(1, 1),
		]
	}
}
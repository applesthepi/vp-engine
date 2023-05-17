use std::{sync::Arc, marker::PhantomData, borrow::Borrow};

use ash::vk;

use crate::{ViewportDepthRange, create_graphics_pipeline, PipelineInfo, BlockCamera};

pub struct PipelineSimple<V: vpb::Vertex> {
	vertex: PhantomData<V>,
	pub descriptor_pool: vk::DescriptorPool,
	pub pipeline: vk::Pipeline,
	pub pipeline_layout: vk::PipelineLayout,
	pub viewport: [vk::Viewport; 1],
	pub scissor: [vk::Rect2D; 1],
	pub block_states: Vec<vpb::BlockState>,
}

impl<V: vpb::Vertex> PipelineSimple<V> {
	pub fn new(
		device: &vpb::Device,
		instance: &vpb::Instance,
		window: &vpb::Window,
		renderpass: &vpb::RenderPass,
		shader_loader: &Arc<vpb::ShaderLoader>,
		frame_count: usize,
	) -> Self { unsafe {
		let descriptor_pool_max = 1 * frame_count as u32;
		let descriptor_pool_size = vk::DescriptorPoolSize::builder()
			.descriptor_count(descriptor_pool_max)
			.build();
		let descriptor_pool_info = vk::DescriptorPoolCreateInfo::builder()
			.pool_sizes(&[descriptor_pool_size])
			.max_sets(descriptor_pool_max)
			.build();
		let descriptor_pool = device.device.create_descriptor_pool(
			&descriptor_pool_info,
			None,
		).unwrap();
		let block_states: Vec<vpb::BlockState> = vec![
			BlockCamera::create_block_state(
				device,
				instance,
				&descriptor_pool,
				frame_count,
				2, 0,
			),
		];
		let (pipeline, pipeline_layout, viewport, scissor) = create_graphics_pipeline::<V>(
			device,
			window,
			renderpass,
			shader_loader,
			"ui_lighting",
			PipelineInfo {
				depth: true,
				viewport_depth_range: ViewportDepthRange::UI,
				polygon_mode: vk::PolygonMode::FILL,
				block_states: &block_states,
			},
		);
		Self {
			vertex: PhantomData,
			descriptor_pool,
			pipeline,
			pipeline_layout,
			viewport,
			scissor,
			block_states,
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
	fn bind_blocks(
		&self,
		device: &vpb::Device,
		command_buffer: &vk::CommandBuffer,
		frame: usize,
	) { unsafe {
		let descriptor_sets: Vec<vk::DescriptorSet> = self.block_states.iter().map(
			|x| {
				x.descriptor_buffers[frame].set
			}
		).collect();
		device.device.cmd_bind_descriptor_sets(
			*command_buffer,
			vk::PipelineBindPoint::GRAPHICS,
			self.pipeline_layout,
			0,
			&descriptor_sets,
			&[],
		);
	}}
	fn destroy_set_layouts(
		&self,
		device: &vpb::Device,
	) { unsafe {
		for block_state in self.block_states.iter() {
			device.device.destroy_descriptor_set_layout(
				block_state.layout,
				None,
			);
		}
		// TODO: destroy object's model 
	}}
}
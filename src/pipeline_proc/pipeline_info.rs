use std::sync::Arc;

use ash::vk;

use crate::{ViewportDepthRange, ProgramData, create_graphics_pipeline, ObjectBlockStructure};

pub struct PipelineInfo {
	pub pipeline: vk::Pipeline,
	pub pipeline_layout: vk::PipelineLayout,
	pub viewport: [vk::Viewport; 1],
	pub scissor: [vk::Rect2D; 1],
	pub depth: bool,
	pub viewport_depth_range: ViewportDepthRange,
	pub polygon_mode: vk::PolygonMode,
	pub block_states: Vec<Arc<vpb::BlockState>>,
	pub name: String,
}

impl PipelineInfo {
	pub fn new<V: vpb::Vertex>(
		program_data: &ProgramData,
		name: &str,
		depth: bool,
		viewport_depth_range: ViewportDepthRange,
		polygon_mode: vk::PolygonMode,
		pipeline_block_structure: &Arc<ObjectBlockStructure>,
		object_block_structure: &Arc<ObjectBlockStructure>,
	) -> Self {
		let block_states = pipeline_block_structure.spawners.iter().map(
			|x|
			x.spawn(
				&program_data.device,
				&program_data.instance,
				&program_data.descriptor_pool.descriptor_pool,
				program_data.frame_count,
			)
		).collect();
		let mut pipeline_info = Self {
			pipeline: vk::Pipeline::null(),
			pipeline_layout: vk::PipelineLayout::null(),
			viewport: [vk::Viewport::default()],
			scissor: [vk::Rect2D::default()],
			depth,
			viewport_depth_range,
			polygon_mode,
			block_states,
			name: name.to_string(),
		};
		let (
			pipeline,
			pipeline_layout,
			viewport,
			scissor
		) = create_graphics_pipeline::<V>(
			program_data,
			name,
			&pipeline_info,
			pipeline_block_structure,
			object_block_structure,
		);
		pipeline_info.pipeline = pipeline;
		pipeline_info.pipeline_layout = pipeline_layout;
		pipeline_info.viewport = viewport;
		pipeline_info.scissor = scissor;
		pipeline_info
	}

	pub fn recreate_pipeline<V: vpb::Vertex>(
		&mut self,
		program_data: &ProgramData,
		pipeline_block_structure: &Arc<ObjectBlockStructure>,
		object_block_structure: &Arc<ObjectBlockStructure>,
	) {
		let (
			pipeline,
			pipeline_layout,
			viewport,
			scissor
		) = create_graphics_pipeline::<V>(
			program_data,
			&self.name,
			&self,
			pipeline_block_structure,
			object_block_structure,
		);
		self.pipeline = pipeline;
		self.pipeline_layout = pipeline_layout;
		self.viewport = viewport;
		self.scissor = scissor;
	}

	pub fn destroy_pipeline(
		&mut self,
		program_data: &ProgramData,
	) { unsafe {
		program_data.device.device.destroy_pipeline(
			self.pipeline,
			None,
		);
		program_data.device.device.destroy_pipeline_layout(
			self.pipeline_layout,
			None,
		);
	}}
}
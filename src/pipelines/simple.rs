use std::{sync::Arc, marker::PhantomData};

use ash::vk;

use crate::{ViewportDepthRange, create_graphics_pipeline, PipelineInfo};

pub struct PipelineSimple<V: vpb::Vertex> {
	vertex: PhantomData<V>,
	pub pipeline: vk::Pipeline,
	pub viewport: [vk::Viewport; 1],
	pub scissor: [vk::Rect2D; 1],
}

impl<V: vpb::Vertex> PipelineSimple<V> {
	pub fn new(
		device: &vpb::Device,
		window: &vpb::Window,
		renderpass: &vpb::RenderPass,
		shader_loader: &Arc<vpb::ShaderLoader>,
	) -> Self {
		let (pipeline, viewport, scissor) = create_graphics_pipeline::<V>(
			device,
			window,
			renderpass,
			shader_loader,
			"ui_lighting",
			PipelineInfo {
				depth: true,
				viewport_depth_range: ViewportDepthRange::UI,
				polygon_mode: vk::PolygonMode::FILL,
			},
		);
		Self {
			vertex: PhantomData,
			pipeline,
			viewport,
			scissor,
		}
	}
}

impl<V: Vertex> vpb::Pipeline for PipelineSimple<V> {
	fn get_viewport(&self) -> [vk::Viewport; 1] {
		self.viewport
	}
	fn get_scissor(&self) -> [vk::Rect2D; 1] {
		self.scissor
	}
}
use std::{sync::Arc, marker::PhantomData};

use ash::vk;

use crate::{Vertex, ViewportDepthRange, create_graphics_pipeline, PipelineInfo};

pub struct PipelineSimple<V: Vertex> {
	vertex: PhantomData<V>,
	pub pipeline: vk::Pipeline,
}

impl<V: Vertex> PipelineSimple<V> {
	pub fn new(
		device: &vpb::Device,
		window: &vpb::Window,
		renderpass: &vpb::RenderPass,
		shader_loader: &Arc<vpb::ShaderLoader>,
	) -> Self {
		let pipeline = create_graphics_pipeline::<V>(
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
		}
	}
}
use std::sync::Arc;

use ash::vk;
use shaderc::ShaderKind;
use crate::ProgramData;

mod engine_pipeline;
pub use engine_pipeline::*;
mod pipeline_info;
pub use pipeline_info::*;
pub mod pf;
pub use pf::*;

#[derive(Copy, Clone)]
pub enum ViewportDepthRange {
	UI,
	WORLD,
}

pub fn create_viewport(
	window: &vpb::Window,
	depth_range: ViewportDepthRange,
) -> vk::Viewport {
	let depths = match depth_range {
		ViewportDepthRange::UI => [-100.0, 100.0],
		ViewportDepthRange::WORLD => [0.1, 1_000.0],
	};
	vk::Viewport::builder()
		.x(0.0)
		.y(0.0)
		.width(window.extent.width as f32)
		.height(window.extent.height as f32)
		.min_depth(0.0)
		.max_depth(1.0)
		.build()
}

pub fn create_graphics_pipeline<V: vpb::Vertex>(
	program_data: &ProgramData,
	shader_name: &str,
	pipeline_info: &PipelineInfo,
	pipeline_block_structure: &Arc<ObjectBlockStructure>,
	object_block_structure: &Arc<ObjectBlockStructure>,
) -> (vk::Pipeline, vk::PipelineLayout, [vk::Viewport; 1], [vk::Rect2D; 1]) {
	let sm_vert = program_data.load_shader(
		ShaderKind::Vertex,
		shader_name,
	);
	let sm_frag = program_data.load_shader(
		ShaderKind::Fragment,
		shader_name,
	);
	let stages = vpb::create_stage_infos(
		&[
			(
				sm_vert,
				vk::ShaderStageFlags::VERTEX,
			),
			(
				sm_frag,
				vk::ShaderStageFlags::FRAGMENT,
			),
		]
	);
	create_pipeline::<V>(
		program_data,
		&stages,
		pipeline_info,
		pipeline_block_structure,
		object_block_structure,
	)
}

fn create_pipeline<V: vpb::Vertex>(
	program_data: &ProgramData,
	stages: &[vk::PipelineShaderStageCreateInfo],
	pipeline_info: &PipelineInfo,
	pipeline_block_structure: &Arc<ObjectBlockStructure>,
	object_block_structure: &Arc<ObjectBlockStructure>,
) -> (vk::Pipeline, vk::PipelineLayout, [vk::Viewport; 1], [vk::Rect2D; 1]) { unsafe {
	let binding_descriptions = V::binding_descriptions();
	let attribute_descriptions = V::attribute_descriptions();
	let input_state_info = vk::PipelineVertexInputStateCreateInfo::builder()
		.vertex_attribute_descriptions(&attribute_descriptions)
		.vertex_binding_descriptions(&binding_descriptions)
		.build();
	let assembly_state_info = vk::PipelineInputAssemblyStateCreateInfo::builder()
		.topology(vk::PrimitiveTopology::TRIANGLE_LIST)
		.build();
	let viewports = [create_viewport(&program_data.window, pipeline_info.viewport_depth_range)];
	let scissors = [program_data.window.extent.into()];
	let viewport_state_info = vk::PipelineViewportStateCreateInfo::builder()
		.scissors(&scissors)
		.viewports(&viewports)
		.build();
	let rasterization_info = vk::PipelineRasterizationStateCreateInfo::builder()
		.front_face(vk::FrontFace::COUNTER_CLOCKWISE)
		.line_width(1.0)
		.polygon_mode(pipeline_info.polygon_mode)
		.build();
	let multisample_state_info = vk::PipelineMultisampleStateCreateInfo::builder()
		.rasterization_samples(vk::SampleCountFlags::TYPE_1)
		.build();
	let stencil_state_info = vk::StencilOpState::builder()
		.fail_op(vk::StencilOp::KEEP)
		.pass_op(vk::StencilOp::KEEP)
		.depth_fail_op(vk::StencilOp::KEEP)
		.compare_op(vk::CompareOp::ALWAYS)
		.build();
	let color_blend_attachment_states = [
		vk::PipelineColorBlendAttachmentState::builder()
			.blend_enable(false)
			.src_color_blend_factor(vk::BlendFactor::SRC_COLOR)
			.dst_color_blend_factor(vk::BlendFactor::ONE_MINUS_DST_COLOR)
			.color_blend_op(vk::BlendOp::ADD)
			.src_alpha_blend_factor(vk::BlendFactor::ZERO)
			.dst_alpha_blend_factor(vk::BlendFactor::ZERO)
			.alpha_blend_op(vk::BlendOp::ADD)
			.color_write_mask(vk::ColorComponentFlags::RGBA)
			.build(),
	];
	let color_blend_state = vk::PipelineColorBlendStateCreateInfo::builder()
		.logic_op(vk::LogicOp::CLEAR)
		.attachments(&color_blend_attachment_states)
		.build();
	let dynamic_state = [vk::DynamicState::VIEWPORT, vk::DynamicState::SCISSOR];
	let dynamic_state_info = vk::PipelineDynamicStateCreateInfo::builder()
		.dynamic_states(&dynamic_state)
		.build();
	let mut descriptor_set_layouts: Vec<vk::DescriptorSetLayout> = pipeline_block_structure.spawners.iter().map(
		|x|
		x.layout()
	).collect();
	descriptor_set_layouts.extend(object_block_structure.spawners.iter().map(
		|x|
		x.layout()
	));
	let pipeline_layout_info = vk::PipelineLayoutCreateInfo::builder()
		.set_layouts(&descriptor_set_layouts);
	let pipeline_layout = program_data.device.device.create_pipeline_layout(
		&pipeline_layout_info,
		None,
	).unwrap();
	let graphics_pipeline_info = vk::GraphicsPipelineCreateInfo::builder()
		.stages(&stages)
		.vertex_input_state(&input_state_info)
		.input_assembly_state(&assembly_state_info)
		.viewport_state(&viewport_state_info)
		.rasterization_state(&rasterization_info)
		.multisample_state(&multisample_state_info)
		.color_blend_state(&color_blend_state)
		.dynamic_state(&dynamic_state_info)
		.layout(pipeline_layout)
		.render_pass(program_data.render_pass.render_pass);
	if pipeline_info.depth {
		let depth_state_info = vk::PipelineDepthStencilStateCreateInfo::builder()
			.depth_test_enable(true)
			.depth_write_enable(true)
			.depth_compare_op(vk::CompareOp::LESS_OR_EQUAL)
			.front(stencil_state_info)
			.back(stencil_state_info)
			.max_depth_bounds(1.0)
			.build();
		let graphics_pipeline_info = graphics_pipeline_info.depth_stencil_state(
			&depth_state_info,
		).build();
		(program_data.device.device.create_graphics_pipelines(
			vk::PipelineCache::null(),
			&[graphics_pipeline_info],
			None,
		).unwrap()[0], pipeline_layout, viewports, scissors)
	} else {
		let graphics_pipeline_info = graphics_pipeline_info.build();
		(program_data.device.device.create_graphics_pipelines(
			vk::PipelineCache::null(),
			&[graphics_pipeline_info],
			None,
		).unwrap()[0], pipeline_layout, viewports, scissors)
	}
}}
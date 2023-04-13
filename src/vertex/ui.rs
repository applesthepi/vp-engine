use std::mem;

use ash::vk;
use memoffset::offset_of;

use crate::Vertex;

pub struct VertexUI {
	pub position: [f32; 2],
	pub color: [f32; 4],
}

impl Vertex for VertexUI {
	fn stride() -> u32 {
		mem::size_of::<VertexUI>() as u32
	}

	fn attribute_descriptions(
	) -> Vec<vk::VertexInputAttributeDescription> {
		vec![
			vk::VertexInputAttributeDescription::builder()
				.location(0)
				.binding(0)
				.format(vk::Format::R32G32_SFLOAT)
				.offset(offset_of!(VertexUI, position) as u32)
				.build(),
			vk::VertexInputAttributeDescription::builder()
				.location(1)
				.binding(0)
				.format(vk::Format::R32G32B32A32_SFLOAT)
				.offset(offset_of!(VertexUI, position) as u32)
				.build(),
		]
	}
}
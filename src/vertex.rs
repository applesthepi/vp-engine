mod ui;
use ash::vk;
pub use ui::*;

pub trait Vertex {
	fn stride() -> u32;
	fn attribute_descriptions() -> Vec<vk::VertexInputAttributeDescription>;
}
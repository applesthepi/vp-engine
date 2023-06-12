use std::{mem::size_of, sync::Arc};

use ash::vk;

use crate::ProgramData;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct BlockCamera {
	pub view: nalgebra_glm::Mat4,
	pub projection: nalgebra_glm::Mat4,
}

impl BlockCamera {
	pub fn create_block_state(
		program_data: &ProgramData,
		descriptor_set_layout: &vk::DescriptorSetLayout,
		binding: u32,
		set: u32,
	) -> vpb::BlockState {
		let block_state = vpb::BlockState::new(
			&program_data.device,
			&program_data.instance,
			&program_data.descriptor_pool.descriptor_pool,
			descriptor_set_layout,
			program_data.frame_count,
			binding,
			set,
			size_of::<BlockCamera>(),
			1,
		);
		block_state
	}
	pub fn create_descriptor_set_layout(
		device: &Arc<vpb::Device>,
	) -> vk::DescriptorSetLayout { unsafe {
		let descriptor_set_layout_binding = vk::DescriptorSetLayoutBinding::builder()
			.binding(0)
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
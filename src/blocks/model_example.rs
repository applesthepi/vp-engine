use std::{mem::size_of, sync::Arc};

use ash::vk;
use nalgebra::{Vector4, Matrix4};

#[repr(C)]
#[derive(Copy, Clone)]
pub struct BlockModelExample {
	pub model: Matrix4<f32>,
	pub color: Vector4<f32>,
}

impl vpb::Block for BlockModelExample {
	fn create_block_state(
		device: &vpb::Device,
		instance: &vpb::Instance,
		descriptor_pool: &vk::DescriptorPool,
		descriptor_set_layout: &vk::DescriptorSetLayout,
		frame_count: usize,
		binding: u32,
		set: u32,
	) -> Arc<vpb::BlockState> {
		Arc::new(vpb::BlockState::new(
			device,
			instance,
			descriptor_pool,
			descriptor_set_layout,
			frame_count,
			binding,
			set,
			size_of::<BlockModelExample>(),
			1,
		))
	}

	fn create_descriptor_set_layout(
		device: &Arc<vpb::Device>,
		binding: u32,
	) -> vk::DescriptorSetLayout { unsafe {
		let descriptor_set_layout_binding = vk::DescriptorSetLayoutBinding::builder()
			.binding(binding)
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
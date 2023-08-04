use std::{mem::size_of, sync::Arc};

use ash::vk;
use nalgebra::{Vector4, Matrix4};
use vpb::{DescriptorDescription, DDType, DDTypeUniform, BindingId, ProgramData};

#[repr(C)]
#[derive(Copy, Clone)]
pub struct BlockModelExample {
	pub model: Matrix4<f32>,
	pub color: Vector4<f32>,
}

impl vpb::Block for BlockModelExample {
	fn create_block_state(
		program_data: &ProgramData,
		descriptor_set_layout: &vk::DescriptorSetLayout,
		frame_count: usize,
		binding: vpb::BindingId,
		set: vpb::SetId,
	) -> Arc<vpb::BlockState> {
		Arc::new(vpb::BlockState::new(
			program_data,
			descriptor_set_layout,
			frame_count,
			set,
			DescriptorDescription::new(&[
				DDType::Uniform(DDTypeUniform {
					binding,
					size: size_of::<BlockModelExample>(),
				}),
			]),
		))
	}

	fn create_descriptor_set_layout(
		device: &Arc<vpb::Device>,
		binding: BindingId,
	) -> vk::DescriptorSetLayout { unsafe {
		let descriptor_set_layout_binding = vk::DescriptorSetLayoutBinding::builder()
			.binding(binding.0)
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
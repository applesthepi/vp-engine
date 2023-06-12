mod bucket;

use std::{sync::Arc, marker::PhantomData};

use ash::vk;
pub use bucket::*;
use vpb::{create_depth_image, create_presentation_images};

use crate::{VertexUI, ProgramData, pd_vdevice, pd_device, simple::PipelineSimple};

pub struct Scene {
	program_data: ProgramData,
	buckets: Vec<Box<Bucket>>,
	semaphore_present: vk::Semaphore,
	semaphore_render: vk::Semaphore,
	framebuffers: Vec<vk::Framebuffer>,
	framebuffer_imageviews: Vec<vk::ImageView>,
	depth_image_view: vk::ImageView,
}

impl Scene {
	pub fn new(
		mut program_data: ProgramData,
	) -> (Self, usize) { unsafe {
		let semaphore_create_info = vk::SemaphoreCreateInfo::builder().build();
		let semaphore_present = program_data.device.device.create_semaphore(
			&semaphore_create_info,
			None,
		).unwrap();
		let semaphore_render = program_data.device.device.create_semaphore(
			&semaphore_create_info,
			None,
		).unwrap();
		let (
			framebuffers,
			present_image_views,
			depth_image_view,
			depth_image,
		) = Scene::create_framebuffers(
			&mut program_data,
		);
		program_data.frame_count = framebuffers.len();
		let pipelines = Scene::create_pipelines(
			&program_data,
		);
		let mut buckets: Vec<Box<Bucket>> = Vec::with_capacity(8);
		let pipeline_ui = Arc::new(pipelines.0);
		let pipeline_ui_engine = pipeline_ui.clone();
		buckets.push(Box::new(Bucket::new(
			"ui",
			pipeline_ui,
			pipeline_ui_engine,
			program_data.clone(),
			2,
		)));
		let frame_count = program_data.frame_count;
		let scene = Self {
			program_data,
			buckets,
			semaphore_present,
			semaphore_render,
			framebuffers,
			framebuffer_imageviews: present_image_views,
			depth_image_view,
		};
		scene.setup_submit(
			&depth_image
		);
		(scene, frame_count)
	}}

	pub fn create_framebuffers(
		program_data: &mut ProgramData,
	) -> (Vec<vk::Framebuffer>, Vec<vk::ImageView>, vk::ImageView, vk::Image) { unsafe {
		let (present_images, present_image_views) = create_presentation_images(
			&program_data.device,
			&program_data.swapchain,
		);
		let (depth_image, depth_image_view) = create_depth_image(
			&program_data.device,
			&program_data.window,
		);
		let framebuffers = present_image_views.iter().map(
			|image_view| {
				let framebuffer_attachments = [*image_view, depth_image_view];
				let frame_buffer_info = vk::FramebufferCreateInfo::builder()
					.render_pass(program_data.render_pass.render_pass)
					.attachments(&framebuffer_attachments)
					.width(program_data.window.extent.width)
					.height(program_data.window.extent.height)
					.layers(1)
					.build();
				
				program_data.device.device.create_framebuffer(
					&frame_buffer_info,
					None,
				).unwrap()
			}
		).collect();
		(framebuffers, present_image_views, depth_image_view, depth_image)
	}}

	#[allow(unused_parens)]
	pub fn create_pipelines(
		program_data: &ProgramData,
	) -> (
		PipelineSimple<VertexUI>,
	) {
		(
			PipelineSimple::<VertexUI>::new(
				program_data,
			),
		)
	}

	pub fn get_bucket(
		&mut self,
		name: &str,
	) -> &mut Bucket {
		self.buckets.iter_mut().find(
			|x|
			x.name == name
		).expect(format!("no bucket with name \"{}\"", name).as_str())
	}

	fn sync_fences(
		&self,
		command_buffer: &vpb::CommandBuffer,
	) { unsafe {
		self.program_data.device.device.wait_for_fences(
			&[command_buffer.fence_submit],
			true,
			std::u64::MAX,
		).unwrap();
		self.program_data.device.device.reset_fences(
			&[command_buffer.fence_submit],
		).unwrap();
	}}

	pub fn acquire_next_image(
		&self,
	) -> usize { unsafe {
		self.program_data.swapchain.swapchain_loader.acquire_next_image(
			self.program_data.swapchain.swapchain,
			std::u64::MAX,
			self.semaphore_present,
			vk::Fence::null(),
		).unwrap().0 as usize
	}}

	pub fn setup_submit(
		&self,
		depth_image: &vk::Image,
	) { unsafe {
		self.sync_fences(
			&self.program_data.command_buffer_setup,
		);
		self.program_data.command_buffer_setup.open(
			&self.program_data.device
		);
		let layout_transition_barriers = vk::ImageMemoryBarrier::builder()
			.image(*depth_image)
			.dst_access_mask(
				vk::AccessFlags::DEPTH_STENCIL_ATTACHMENT_READ |
				vk::AccessFlags::DEPTH_STENCIL_ATTACHMENT_WRITE,
			).new_layout(
				vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
			).old_layout(
				vk::ImageLayout::UNDEFINED,
			).subresource_range(
				vk::ImageSubresourceRange::builder()
					.aspect_mask(vk::ImageAspectFlags::DEPTH)
					.layer_count(1)
					.level_count(1)
					.build()
			).build();
		self.program_data.device.device.cmd_pipeline_barrier(
			self.program_data.command_buffer_setup.command_buffer,
			vk::PipelineStageFlags::BOTTOM_OF_PIPE,
			vk::PipelineStageFlags::LATE_FRAGMENT_TESTS,
			vk::DependencyFlags::empty(),
			&[],
			&[],
			&[layout_transition_barriers],
		);
		self.program_data.command_buffer_setup.close(
			&self.program_data.device,
		);
		self.submit(
			&self.program_data.command_buffer_setup,
			false,
		);
		self.idle();
	}}

	pub fn render(
		&mut self,
	) {
		let present_index = self.acquire_next_image();
		self.sync_fences(
			&self.program_data.command_buffer_draw,
		);
		self.program_data.command_buffer_draw.open(
			&self.program_data.device
		);
		for bucket in self.buckets.iter_mut() {
			bucket.update_blocks(
				&self.program_data.device,
				present_index,
			);
		}
		self.program_data.render_pass.open(
			&self.program_data.device,
			&self.program_data.window.extent,
			&self.framebuffers[present_index],
			&self.program_data.command_buffer_draw.command_buffer,
		);
		for bucket in self.buckets.iter_mut() {
			bucket.render(
				&self.program_data.device,
				self.program_data.command_buffer_draw.command_buffer,
				present_index,
			);
		}
		self.program_data.render_pass.close(
			&self.program_data.device,
			&self.program_data.command_buffer_draw,
		);
		self.program_data.command_buffer_draw.close(
			&self.program_data.device,
		);
		self.submit(
			&self.program_data.command_buffer_draw,
			true,
		);
		self.present(present_index);
	}

	fn submit(
		&self,
		command_buffer: &vpb::CommandBuffer,
		use_semaphores: bool,
	) { unsafe {
		if use_semaphores {
			let command_buffers = vec![command_buffer.command_buffer];
			let submit_info = vk::SubmitInfo::builder()
				.wait_semaphores(&[self.semaphore_present])
				.wait_dst_stage_mask(&[vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT])
				.command_buffers(&command_buffers)
				.signal_semaphores(&[self.semaphore_render])
				.build();
			self.program_data.device.device.queue_submit(
				command_buffer.present_queue,
				&[submit_info],
				command_buffer.fence_submit,
			).unwrap();
		} else {
			let command_buffers = vec![command_buffer.command_buffer];
			let mut submit_info = vk::SubmitInfo::builder()
				.wait_semaphores(&[])
				.wait_dst_stage_mask(&[vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT])
				.command_buffers(&command_buffers)
				.signal_semaphores(&[])
				.build();
			submit_info.wait_semaphore_count = 0;
			submit_info.signal_semaphore_count = 0;
			self.program_data.device.device.queue_submit(
				command_buffer.present_queue,
				&[submit_info],
				command_buffer.fence_submit,
			).unwrap();
		}
	}}

	fn present(
		&self,
		present_index: usize,
	) { unsafe {
		let present_info = vk::PresentInfoKHR::builder()
			.wait_semaphores(&[self.semaphore_render])
			.swapchains(&[self.program_data.swapchain.swapchain])
			.image_indices(&[present_index as u32])
			.build();
		self.program_data.swapchain.swapchain_loader.queue_present(
			self.program_data.command_buffer_draw.present_queue,
			&present_info,
		).unwrap();
	}}

	pub fn idle(
		&self,
	) { unsafe {
		self.program_data.device.device.device_wait_idle().unwrap();
	}}

	pub fn resize(
		&mut self,
		program_data: &mut ProgramData,
	) { unsafe {
		self.destroy_swapchain();
		// SWAPCHAIN
		Scene::create_swapchain(program_data);
		// RENDER PASS
		let render_pass = vpb::gmuc!(
			self.program_data.render_pass
		);
		*render_pass = vpb::RenderPass::new(
			&self.program_data.device,
			&program_data.swapchain,
		);
		// FRAMEBUFFERS
		let (
			framebuffers,
			present_image_views,
			depth_image_view,
			depth_image,
		) = Scene::create_framebuffers(
			program_data,
		);
		self.framebuffers = framebuffers;
		self.framebuffer_imageviews = present_image_views;
		self.depth_image_view = depth_image_view;
		// PIPELINES
		for bucket in self.buckets.iter_mut() {
			bucket.recreate_pipeline();
		}
		// DESCRIPTOR POOL
		let descriptor_pool = vpb::gmuc!(
			self.program_data.descriptor_pool
		);
		*descriptor_pool = vpb::DescriptorPool::new(
			&self.program_data.device,
			self.program_data.frame_count,
		);
		// DESCRIPTOR MEMORY
		for bucket in self.buckets.iter_mut() {
			bucket.recreate_block_state_memory();
		}
		// COMMAND BUFFERS
		let command_buffer_draw = vpb::CommandBuffer::new(
			&program_data.device,
			&program_data.command_pool,
			&program_data.swapchain,
		);
		let command_buffer_setup = vpb::CommandBuffer::new(
			&program_data.device,
			&program_data.command_pool,
			&program_data.swapchain,
		);
		self.setup_submit(
			&depth_image,
		);
	}}

	fn destroy_swapchain(
		&mut self,
	) { unsafe {
		self.idle();
		// COMMAND BUFFERS
		self.program_data.device.device.free_command_buffers(
			self.program_data.command_pool.command_pool,
			&[
				self.program_data.command_buffer_setup.command_buffer,
				self.program_data.command_buffer_draw.command_buffer,
			],
		);
		// DESCRIPTOR POOL
		self.program_data.device.device.destroy_descriptor_pool(
			self.program_data.descriptor_pool.descriptor_pool,
			None,
		);
		// DESCRIPTOR MEMORY
		for bucket in self.buckets.iter_mut() {
			bucket.destroy_block_state_memory();
		}
		// FRAMEBUFFERS
		for framebuffer in self.framebuffers.iter() {
			self.program_data.device.device.destroy_framebuffer(
				*framebuffer,
				None,
			);
		}
		// PIPELINES
		for bucket in self.buckets.iter_mut() {
			bucket.destroy_pipeline();
		}
		// RENDER PASS
		self.program_data.device.device.destroy_render_pass(
			self.program_data.render_pass.render_pass,
			None,
		);
		// SWAPCHAIN IMAGE VIEWS
		for image_view in self.framebuffer_imageviews.iter() {
			self.program_data.device.device.destroy_image_view(
				*image_view,
				None,
			);
		}
		self.program_data.device.device.destroy_image_view(
			self.depth_image_view,
			None,
		);
		// SWAPCHAIN
		self.program_data.swapchain.swapchain_loader.destroy_swapchain(
			self.program_data.swapchain.swapchain,
			None,
		);
	}}

	fn create_swapchain(
		program_data: &mut ProgramData,
	) {
		let swapchain = vpb::Swapchain::new(
			&program_data.instance,
			vpb::gmuc!(program_data.window),
			&program_data.surface,
			&program_data.device,
		);
		let wa_swapchain = vpb::gmuc!(program_data.swapchain);
		*wa_swapchain = swapchain;
	}
}
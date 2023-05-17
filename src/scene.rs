mod bucket;

use std::{sync::Arc, marker::PhantomData};

use ash::vk;
pub use bucket::*;
use vpb::{create_depth_image, create_presentation_images};

use crate::{VertexUI, PipelineSimple, ProgramData};

pub struct Scene<'a> {
	program_data: ProgramData,
	command_buffer_setup: vpb::CommandBuffer,
	render_pass: vpb::RenderPass,
	buckets: Vec<Box<Bucket<'a>>>,
	semaphore_present: vk::Semaphore,
	semaphore_render: vk::Semaphore,
	framebuffers: Vec<vk::Framebuffer>,
	framebuffer_imageviews: Vec<vk::ImageView>,
	extent: vk::Extent2D,
	depth_image_view: vk::ImageView,
}

impl<'a> Scene<'a> {
	pub fn new(
		program_data: ProgramData,
	) -> Self { unsafe {
		let command_buffer_setup = vpb::CommandBuffer::new(
			&mut device,
			command_pool,
			&swapchain,
		);
		let semaphore_create_info = vk::SemaphoreCreateInfo::builder().build();
		let semaphore_present = device.device.create_semaphore(
			&semaphore_create_info,
			None,
		).unwrap();
		let semaphore_render = device.device.create_semaphore(
			&semaphore_create_info,
			None,
		).unwrap();
		let (
			framebuffers,
			present_image_views,
			depth_image_view,
			depth_image,
		) = Scene::create_framebuffers(
			&device,
			&swapchain,
			window,
			&renderpass,
		);
		let pipelines = Scene::create_pipelines(
			&device,
			instance,
			window,
			&renderpass,
			&shader_loader,
			framebuffers.len(),
		);
		let mut buckets: Vec<Box<Bucket>> = Vec::with_capacity(8);
		buckets.push(Box::new(Bucket::new(
			"ui",
			Box::new(pipelines.0),
			&device,
			instance,
			&pipelines.0.descriptor_pool,
			framebuffers.len(),
			2,
		)));
		let scene = Self {
			device,
			shader_loader,
			command_buffer,
			command_buffer_setup,
			render_pass: renderpass,
			swapchain,
			buckets,
			semaphore_present,
			semaphore_render,
			framebuffers,
			framebuffer_imageviews: present_image_views,
			extent: window.extent,
			depth_image_view,
		};
		scene.setup_submit(
			&depth_image
		);
		scene
	}}

	pub fn create_framebuffers(
		device: &vpb::Device,
		swapchain: &vpb::Swapchain,
		window: &vpb::Window,
		renderpass: &vpb::RenderPass,
	) -> (Vec<vk::Framebuffer>, Vec<vk::ImageView>, vk::ImageView, vk::Image) { unsafe {
		let (present_images, present_image_views) = create_presentation_images(
			&device,
			&swapchain,
		);
		let (depth_image, depth_image_view) = create_depth_image(
			&device,
			window,
		);
		let framebuffers = present_image_views.iter().map(
			|image_view| {
				let framebuffer_attachments = [*image_view, depth_image_view];
				let frame_buffer_info = vk::FramebufferCreateInfo::builder()
					.render_pass(renderpass.renderpass)
					.attachments(&framebuffer_attachments)
					.width(window.extent.width)
					.height(window.extent.height)
					.layers(1)
					.build();
				device.device.create_framebuffer(
					&frame_buffer_info,
					None,
				).unwrap()
			}
		).collect();
		(framebuffers, present_image_views, depth_image_view, depth_image)
	}}

	#[allow(unused_parens)]
	pub fn create_pipelines(
		device: &vpb::Device,
		instance: &vpb::Instance,
		window: &vpb::Window,
		render_pass: &vpb::RenderPass,
		shader_loader: &Arc<vpb::ShaderLoader>,
		frame_count: usize,
	) -> (
		PipelineSimple<VertexUI>,
	) {
		(
			PipelineSimple::<VertexUI>::new(
				device,
				instance,
				window,
				render_pass,
				shader_loader,
				frame_count,
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
		self.device.device.wait_for_fences(
			&[command_buffer.fence_submit],
			true,
			std::u64::MAX,
		).unwrap();
		self.device.device.reset_fences(
			&[command_buffer.fence_submit],
		).unwrap();
	}}

	pub fn acquire_next_image(
		&self,
	) -> usize { unsafe {
		self.swapchain.swapchain_loader.acquire_next_image(
			self.swapchain.swapchain,
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
			&self.command_buffer_setup,
		);
		self.command_buffer_setup.open(
			&self.device
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
		self.device.device.cmd_pipeline_barrier(
			self.command_buffer_setup.command_buffer,
			vk::PipelineStageFlags::BOTTOM_OF_PIPE,
			vk::PipelineStageFlags::LATE_FRAGMENT_TESTS,
			vk::DependencyFlags::empty(),
			&[],
			&[],
			&[layout_transition_barriers],
		);
		self.command_buffer_setup.close(
			&self.device,
		);
		self.submit(
			&self.command_buffer_setup,
			false,
		);
		self.idle();
	}}

	pub fn render(
		&mut self,
	) { unsafe {
		let present_index = self.acquire_next_image();
		self.sync_fences(
			&self.command_buffer,
		);
		self.command_buffer.open(
			&self.device
		);
		self.render_pass.open(
			&self.device,
			&self.extent,
			&self.framebuffers[present_index],
			&self.command_buffer.command_buffer,
		);
		for bucket in self.buckets.iter_mut() {
			bucket.render(
				&self.device,
				self.command_buffer.command_buffer,
				present_index,
			);
		}
		self.render_pass.close(
			&self.device,
			&self.command_buffer,
		);
		self.command_buffer.close(
			&self.device,
		);
		self.submit(
			&self.command_buffer,
			true,
		);
		self.present(present_index);
	}}

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
			self.device.device.queue_submit(
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
			self.device.device.queue_submit(
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
			.swapchains(&[self.swapchain.swapchain])
			.image_indices(&[present_index as u32])
			.build();
		self.swapchain.swapchain_loader.queue_present(
			self.command_buffer.present_queue,
			&present_info,
		).unwrap();
	}}

	pub fn idle(
		&self,
	) { unsafe {
		self.device.device.device_wait_idle().unwrap();
	}}

	pub fn resize(
		&mut self,
		instance: &vpb::Instance,
		window: &mut vpb::Window,
		surface: &vpb::Surface,
		command_pool: &mut vpb::CommandPool,
		size: [u32; 2],
	) { unsafe {
		self.idle();
		for framebuffer in self.framebuffers.iter() {
			self.device.device.destroy_framebuffer(
				*framebuffer,
				None,
			);
		}
		for image_view in self.framebuffer_imageviews.iter() {
			self.device.device.destroy_image_view(
				*image_view,
				None,
			);
		}
		self.device.device.destroy_image_view(
			self.depth_image_view,
			None,
		);
		self.device.device.free_command_buffers(
			command_pool.command_pool,
			&[
				self.command_buffer_setup.command_buffer,
				self.command_buffer.command_buffer
			],
		);
		for bucket in self.buckets.iter() {
			self.device.device.destroy_pipeline(
				bucket.pipeline.get_pipeline(),
				None,
			);
			bucket.pipeline.destroy_set_layouts(&self.device);
		}
		self.device.device.destroy_render_pass(
			self.render_pass.renderpass,
			None,
		);
		self.swapchain.swapchain_loader.destroy_swapchain(
			self.swapchain.swapchain,
			None,
		);
		window.extent = vk::Extent2D {
			width: size[0],
			height: size[1],
		};
		self.swapchain = vpb::Swapchain::new(
			instance,
			window,
			surface,
			&self.device,
		);
		self.render_pass = vpb::RenderPass::new(
			&self.device,
			&self.swapchain,
		);
		let pipelines = Scene::create_pipelines(
			&self.device,
			instance,
			window,
			&self.render_pass,
			&self.shader_loader,
			self.framebuffers.len(),
		);
		self.get_bucket("ui").pipeline = Box::new(pipelines.0);
		let (
			framebuffers,
			present_image_views,
			depth_image_view,
			depth_image,
		) = Scene::create_framebuffers(
			&self.device,
			&self.swapchain,
			window,
			&self.render_pass,
		);
		self.framebuffers = framebuffers;
		self.framebuffer_imageviews = present_image_views;
		self.depth_image_view = depth_image_view;
		self.command_buffer_setup = vpb::CommandBuffer::new(
			&mut self.device,
			command_pool,
			&self.swapchain,
		);
		self.command_buffer = vpb::CommandBuffer::new(
			&mut self.device,
			command_pool,
			&self.swapchain,
		);
		self.setup_submit(
			&depth_image,
		);
	}}
}
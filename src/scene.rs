mod bucket;

use ash::vk;
pub use bucket::*;

use crate::{VertexUI, PipelineSimple};

pub struct Scene {
	device: vpb::Device,
	command_buffer: vpb::CommandBuffer,
	render_pass: vpb::RenderPass,
	swapchain: vpb::Swapchain,
	window: vpb::Window,
	buckets: Vec<Box<Bucket>>,
	semaphore_wait: vk::Semaphore,
	semaphore_signal: vk::Semaphore,
	framebuffers: Vec<vk::Framebuffer>,
}

impl Scene {
	pub fn new(
		device: vpb::Device,
		command_buffer: vpb::CommandBuffer,
		render_pass: vpb::RenderPass,
		surface: &vpb::Surface,
		swapchain: vpb::Swapchain,
		window: vpb::Window,
		renderpass: &vpb::RenderPass,
		shader_loader: &Arc<vpb::ShaderLoader>,
	) -> Self { unsafe {
		let pipeline_simple = PipelineSimple::<VertexUI>::new(
			&device,
			&window,
			renderpass,
			shader_loader,
		);
		let mut buckets: Vec<Box<Bucket>> = Vec::with_capacity(8);
		buckets.push(Box::new(Bucket::new(
			"ui",
			Box::new(pipeline_simple),
		)));
		let semaphore_create_info = vk::SemaphoreCreateInfo::default();
		let semaphore_wait = device.device.create_semaphore(
			&semaphore_create_info,
			None,
		).unwrap();
		let semaphore_signal = device.device.create_semaphore(
			&semaphore_create_info,
			None,
		).unwrap();
		let present_images = swapchain.swapchain_loader.get_swapchain_images(
			swapchain.swapchain,
		).unwrap();
		let present_image_views = present_images.iter().map(
			|image| {
				let image_view_info = vk::ImageViewCreateInfo::builder()
					.view_type(vk::ImageViewType::TYPE_2D)
					.format(device.surface_format)
					.components(
						vk::ComponentMapping {
							r: vk::ComponentSwizzle::R,
							g: vk::ComponentSwizzle::G,
							b: vk::ComponentSwizzle::B,
							a: vk::ComponentSwizzle::A,
						}
					)
					.subresource_range(
						vk::ImageSubresourceRange {
							aspect_mask: vk::ImageAspectFlags::COLOR,
							base_mip_level: 0,
							level_count: 1,
							base_array_layer: 0,
							layer_count: 1,
						}
					)
					.image(image)
					.build();
				device.device.create_image_view(
					&image_view_info,
					None,
				).unwrap();
			}
		).collect();
		let depth_image_info = vk::ImageCreateInfo::builder()
			.image_type(vk::ImageType::TYPE_2D)
			.format(vk::Format::D16_UNORM)
			.extent(window.extent)
			.mip_levels(1)
			.array_layers(1)
			.samples(vk::SampleCountFlags::TYPE_1)
			.tiling(vk::ImageTiling::OPTIMAL)
			.usage(vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT)
			.sharing_mode(vk::SharingMode::EXCLUSIVE)
			.build();
		let depth_image = device.device.create_image(
			&depth_image_info,
			None,
		).unwrap();
		let depth_image_view_info = vk::ImageViewCreateInfo::builder()
			.subresource_range(
				vk::ImageSubresourceRange::builder()
					.aspect_mask(vk::ImageAspectFlags::DEPTH)
					.level_count(1)
					.layer_count(1)
					.build()
			)
			.image(depth_image)
			.format(depth_image_info.format)
			.view_type(vk::ImageViewType::TYPE_2D)
			.build();
		let depth_image_view = device.device.create_image_view(
			&depth_image_view_info,
			None,
		).unwrap();
		let framebuffers = present_image_views.iter().map(
			|image_view| {
				let framebuffer_attachments = [image_view, depth_image_view];
				let frame_buffer_info = vk::FramebufferCreateInfo::builder()
					.render_pass(render_pass.renderpass)
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
		Self {
			device,
			command_buffer,
			render_pass,
			swapchain,
			window,
			buckets,
			semaphore_wait,
			semaphore_signal,
			framebuffers,
		}
	}}

	pub fn get_bucket(
		&mut self,
		name: &str,
	) -> &mut Bucket {
		self.buckets.iter_mut().find(
			|x|
			x.name == name
		).expect(format!("no bucket with name \"{}\"", name).as_str())
	}

	pub fn render(
		&mut self,
	) { unsafe {
		self.device.device.wait_for_fences(
			&[self.command_buffer.fence_submit],
			true,
			std::u64::MAX,
		).unwrap();
		self.device.device.reset_fences(
			&[self.command_buffer.fence_submit],
		).unwrap();
		self.device.device.reset_command_buffer(
			self.command_buffer.command_buffer,
			vk::CommandBufferResetFlags::RELEASE_RESOURCES,
		).unwrap();
		let command_buffer_begin_info = vk::CommandBufferBeginInfo::builder()
			.flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT)
			.build();
		let present_index = self.swapchain.swapchain_loader.acquire_next_image(
			self.swapchain.swapchain,
			std::u64::MAX,
			self.semaphore_signal,
			vk::Fence::null(),
		).unwrap().0 as usize;
		self.device.device.begin_command_buffer(
			self.command_buffer.command_buffer,
			&command_buffer_begin_info,
		).unwrap();
		let render_pass_begin_info = vk::RenderPassBeginInfo::builder()
			.render_pass(self.render_pass.renderpass)
			.framebuffer(self.framebuffers[present_index])
			.render_area(self.window.extent.into())
			.clear_values(
				&[
					vk::ClearValue {
						color: vk::ClearColorValue {
							float32: [0.0, 0.0, 0.0, 0.0],
						}
					},
					vk::ClearValue {
						depth_stencil: vk::ClearDepthStencilValue {
							depth: 1.0,
							stencil: 0,
						}
					},
				]
			)
			.build();
		self.device.device.cmd_begin_render_pass(
			self.command_buffer.command_buffer,
			&render_pass_begin_info,
			vk::SubpassContents::INLINE,
		).unwrap();
		for bucket in self.buckets.iter_mut() {
			self.device.device.cmd_bind_pipeline(
				self.command_buffer,
				vk::PipelineBindPoint::GRAPHICS,
				bucket.pipeline,
			).unwrap();
			self.device.device.cmd_set_viewport(
				self.command_buffer.command_buffer,
				0,
				&bucket.pipeline.get_viewport(),
			);
			self.device.device.cmd_set_scissor(
				self.command_buffer.command_buffer,
				0,
				&bucket.pipeline.get_scissor(),
			);
			bucket.render();
		}
		self.device.device.end_command_buffer(
			self.command_buffer.command_buffer,
		).unwrap();
		let command_buffers = vec![self.command_buffer.command_buffer];
		let submit_info = vk::SubmitInfo::builder()
			.wait_semaphores(self.semaphore_wait)
			.wait_dst_stage_mask(&[vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT])
			.command_buffers(&command_buffers)
			.signal_semaphores(self.semaphore_signal)
			.build();
		self.device.device.queue_submit(
			self.command_buffer.present_queue,
			&[submit_info],
			self.command_buffer.fence_submit,
		).unwrap();
	}}
}
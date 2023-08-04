use std::sync::Arc;

use ash::vk;
use vpb::ProgramData;

use crate::{ViewportDepthRange, PipelineInfo, BlockCamera2d, BlockModelExample, EnginePipeline, ObjectBlockStructure, VertexUI, InputState, RenderState, CameraState2d, Camera};

pub struct PipelineUIExample {
	pipeline_info: Arc<PipelineInfo>,
	pipeline_block_structure: Arc<ObjectBlockStructure>,
	object_block_structure: Arc<ObjectBlockStructure>,
	camera: Arc<dyn Camera>,
}

impl PipelineUIExample {
	pub fn new(
		program_data: &ProgramData,
		camera: Arc<dyn Camera>,
	) -> Self { unsafe {
		let pipeline_block_structure = Arc::new(ObjectBlockStructure {
			spawners: vec![
				Box::new(vpb::BlockSpawner::<BlockCamera2d>::new(
					&program_data.device,
					vpb::BindingId(0), vpb::SetId(0),
				))
			],
		});
		let object_block_structure = Arc::new(ObjectBlockStructure {
			spawners: vec![
				Box::new(vpb::BlockSpawner::<BlockModelExample>::new(
					&program_data.device,
					vpb::BindingId(1), vpb::SetId(1),
				))
			],
		});
		let pipeline_info = Arc::new(PipelineInfo::new::<VertexUI>(
			program_data,
			"ui_geometry",
			true,
			ViewportDepthRange::UI,
			vk::PolygonMode::FILL,
			&pipeline_block_structure,
			&object_block_structure,
			|| {
				vec![]
			},
		));
		Self {
			pipeline_info,
			pipeline_block_structure,
			object_block_structure,
			camera,
		}
	}}
}

impl EnginePipeline for PipelineUIExample {
	fn get_pipeline_info(
		&self,
	) -> Arc<PipelineInfo> {
		self.pipeline_info.clone()
	}

	fn get_pipeline_block_structure(
		&self,
	) -> Arc<ObjectBlockStructure> {
		self.pipeline_block_structure.clone()
	}

	fn get_object_block_structure(
		&self,
	) -> Arc<ObjectBlockStructure> {
		self.object_block_structure.clone()
	}

	fn recreate_pipeline(
		&mut self,
		program_data: &ProgramData,
	) {
		let pipeline_info = vpb::gmuc!(self.pipeline_info);
		pipeline_info.recreate_pipeline::<VertexUI>(
			program_data,
			&self.pipeline_block_structure,
			&self.object_block_structure,
		);
	}

	fn update_block_states(
		&mut self,
		program_data: &ProgramData,
		input_state: &InputState,
		render_state: &RenderState,
	) {
		self.camera.update(
			&program_data.device,
			Some(render_state.frame),
			&self.pipeline_info.block_states[0],
		)
	}
}
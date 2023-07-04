use std::sync::Arc;

use ash::vk;
use nalgebra::Matrix4;

use crate::{ViewportDepthRange, PipelineInfo, BlockCamera, ProgramData, BlockModel, EnginePipeline, ObjectBlockStructure, VertexUI, InputState, RenderState};

pub struct PipelineUI {
	pipeline_info: Arc<PipelineInfo>,
	pipeline_block_structure: Arc<ObjectBlockStructure>,
	object_block_structure: Arc<ObjectBlockStructure>,
}

impl PipelineUI {
	pub fn new(
		program_data: &ProgramData,
	) -> Self { unsafe {
		let pipeline_block_structure = Arc::new(ObjectBlockStructure {
			spawners: vec![
				Box::new(vpb::BlockSpawner::<BlockCamera>::new(
					&program_data.device,
					0, 0,
				))
			],
		});
		let object_block_structure = Arc::new(ObjectBlockStructure {
			spawners: vec![
				Box::new(vpb::BlockSpawner::<BlockModel>::new(
					&program_data.device,
					1, 1,
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
		));
		Self {
			pipeline_info,
			pipeline_block_structure,
			object_block_structure,
		}
	}}
}

impl EnginePipeline for PipelineUI {
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
		let view = Matrix4::identity();
		let projection = Matrix4::new_orthographic(
			0.0,
			program_data.window.extent.width as f32,
			0.0,
			program_data.window.extent.height as f32,
			-100.0,
			100.0,
		);
		let camera_block = BlockCamera {
			view,
			projection,
		};
		self.pipeline_info.block_states[0].update(
			&program_data.device,
			&camera_block,
			Some(render_state.frame),
		)
	}
}
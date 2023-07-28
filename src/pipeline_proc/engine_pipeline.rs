use std::sync::Arc;

use ash::vk;

use crate::{PipelineInfo, ProgramData, InputState, RenderState, rendering::RenderingState};

pub struct ObjectBlockStructure {
	pub spawners: Vec<Box<dyn vpb::BlockSpawnerGen>>,
}

pub trait EnginePipeline {
	fn get_pipeline_info(
		&self,
	) -> Arc<PipelineInfo>;

	fn get_pipeline_block_structure(
		&self,
	) -> Arc<ObjectBlockStructure>;

	fn get_object_block_structure(
		&self,
	) -> Arc<ObjectBlockStructure>;

	fn recreate_pipeline(
		&mut self,
		program_data: &ProgramData,
	);
	
	fn update_block_states(
		&mut self,
		program_data: &ProgramData,
		input_state: &InputState,
		render_state: &RenderState,
	);
}
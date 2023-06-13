use std::sync::Arc;

use crate::{EnginePipeline, ProgramData};

pub fn create_object_block_states(
	program_data: &ProgramData,
	engine_pipeline: &Arc<dyn EnginePipeline>,
) -> Vec<Arc<vpb::BlockState>> {
	let structure = engine_pipeline.get_object_block_structure();
	structure.spawners.iter().map(
		|x|
		x.spawn(
			&program_data.device,
			&program_data.instance,
			&program_data.descriptor_pool.descriptor_pool,
			program_data.frame_count,
		)
	).collect()
}
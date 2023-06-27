use std::sync::Arc;

use crate::ObjectStateBuffers;

/// All object states have this sub state. Fundemental
/// state regardless of object type.
pub struct SubState {
	pub name: String,
	pub block_states: Option<Vec<Arc<vpb::BlockState>>>,
	pub buffers: ObjectStateBuffers,
}
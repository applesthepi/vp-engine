use std::sync::Arc;

use vpb::ProgramData;

use crate::StaticDirtyState;

use self::state::StaticState;

pub mod state;

/// Static objects are NOT controled much from parents,
/// they are controled internaly and flagged privatly
/// for only (VB, IB) buffers & (BS) buffers.
pub trait ObjectStatic {
	fn state(&self) -> Arc<StaticState>;

	fn dirty(
		&mut self,
		dirty_state: StaticDirtyState,
	) {
		let mut state = self.state();
		let state = vpb::gmuc!(state);
		state.dirty_state |= dirty_state;
	}

	fn update_vib(
		&mut self,
		program_data: &ProgramData,
	);

	fn update_bs(
		&mut self,
		program_data: &ProgramData,
		frame: usize,
	);

	fn update_block_states(
		&mut self,
		program_data: &ProgramData,
		frame: usize,
		frame_count: usize,
	) {
		let mut state = self.state();
		let state = vpb::gmuc!(state);
		let dirty_state = &mut state.dirty_state;
		let bs_left = &mut state.bs_left;
		if bit_compare!(*dirty_state, StaticDirtyState::VIB) {
			self.update_vib(program_data);
		}
		let bs_state = bit_compare!(*dirty_state, StaticDirtyState::BS);
		if bs_state || *bs_left > 0 {
			self.update_bs(program_data, frame);
			if bs_state {
				*bs_left = frame_count as u8 - 1;

			} else {
				*bs_left -= 1;
			}
		}
		*dirty_state = StaticDirtyState::empty();
	}
}
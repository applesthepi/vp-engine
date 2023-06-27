use std::sync::Arc;

use crate::DynamicDirtyState;

use self::state::DynamicState;

pub mod state;

/// Dynamic objects are flagged and controled from parents like
/// flagging dirty for particular or general buffers after changing
/// internal data.
pub trait ObjectDynamic {
	fn state(&self) -> Arc<DynamicState>;

	fn dirty(
		&mut self,
		dirty_state: DynamicDirtyState,
	) {
		let mut state = self.state();
		let state = vpb::gmuc!(state);
		state.dirty_state |= dirty_state;
	}

	fn update_vb(
		&mut self,
		device: &vpb::Device,
	);

	fn update_ib(
		&mut self,
		device: &vpb::Device,
	);

	fn update_bs(
		&mut self,
		device: &vpb::Device,
		frame: usize,
	);

	fn update_block_states(
		&mut self,
		device: &vpb::Device,
		frame: usize,
		frame_count: usize,
	) {
		let mut state = self.state();
		let dirty_state = state.as_ref().dirty_state;
		let bs_left = &mut vpb::gmuc!(state).bs_left;
		// Position: BS
		// Mesh: VB & IB
		let mut vb = false;
		let mut ib = false;
		let mut bs = false;
		if bit_compare!(dirty_state, DynamicDirtyState::VB) {
			vb = true;
			self.update_vb(device);
		}
		if bit_compare!(dirty_state, DynamicDirtyState::IB) {
			ib = true;
			self.update_ib(device);
		}
		let bs_state = bit_compare!(dirty_state, DynamicDirtyState::BS);
		if bs_state || *bs_left > 0 {
			bs = true;
			self.update_bs(device, frame);
			if bs_state {
				*bs_left = frame_count as u8 - 1;
			} else {
				*bs_left -= 1;
			}
		}
		if bit_compare!(dirty_state, DynamicDirtyState::Position) {
			if !bs {
				self.update_bs(device, frame);
			}
		}
		if bit_compare!(dirty_state, DynamicDirtyState::Mesh) {
			if !vb {
				self.update_vb(device);
			}
			if !ib {
				self.update_ib(device);
			}
		}
		vpb::gmuc!(self.state()).dirty_state = DynamicDirtyState::empty();
	}
}
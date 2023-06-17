#![feature(get_mut_unchecked)]
#![feature(core_intrinsics)]
#![feature(return_position_impl_trait_in_trait)]

mod program;
pub use program::*;
mod scene;
pub use scene::*;
pub mod pipelines;
// pub use pipelines::*;
mod vertex;
pub use vertex::*;
mod objects;
pub use objects::*;
mod blocks;
pub use blocks::*;
mod pipeline_proc;
pub use pipeline_proc::*;
mod input_state;
pub use input_state::*;
mod render_state;
pub use render_state::*;
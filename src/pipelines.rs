pub mod simple;
use std::sync::Arc;

use ash::vk::{self, ShaderStageFlags};
use shaderc::ShaderKind;
use vpb::create_stage_infos;

use crate::ProgramData;
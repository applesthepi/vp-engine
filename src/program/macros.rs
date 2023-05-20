macro_rules! pd_window {
	($program_data:expr) => {
		vpb::gmuc!($program_data.window)
	};
}
macro_rules! pd_instance {
	($program_data:expr) => {
		vpb::gmuc!($program_data.instance)
	};
}
macro_rules! pd_surface {
	($program_data:expr) => {
		vpb::gmuc!($program_data.surface)
	};
}
macro_rules! pd_device {
	($program_data:expr) => {
		vpb::gmuc!($program_data.device)
	};
}
macro_rules! pd_vdevice {
	($program_data:expr) => {
		vpb::gmuc!($program_data.device).device
	};
}
macro_rules! pd_swapchain {
	($program_data:expr) => {
		vpb::gmuc!($program_data.swapchain)
	};
}
macro_rules! pd_command_pool {
	($program_data:expr) => {
		vpb::gmuc!($program_data.command_pool)
	};
}
macro_rules! pd_command_buffer_setup {
	($program_data:expr) => {
		vpb::gmuc!($program_data.command_buffer_setup)
	};
}
macro_rules! pd_command_buffer_draw {
	($program_data:expr) => {
		vpb::gmuc!($program_data.command_buffer_draw)
	};
}

pub(crate) use pd_window;
pub(crate) use pd_instance;
pub(crate) use pd_surface;
pub(crate) use pd_device;
pub(crate) use pd_vdevice;
pub(crate) use pd_swapchain;
pub(crate) use pd_command_pool;
pub(crate) use pd_command_buffer_setup;
pub(crate) use pd_command_buffer_draw;
pub mod ui;

pub trait Object {
	fn name(&self) -> &String;
	fn render(&self, command_buffer: vk::CommandBuffer,);
}
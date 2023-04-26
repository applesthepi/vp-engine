use std::sync::Arc;

pub mod ui;

pub trait Object {
	fn name(&self) -> &String;
	fn vertex_buffer(&self) -> Arc<dyn vpb::VertexBuffer>;
	fn index_buffer(&self) -> Arc<dyn vpb::IndexBuffer>;
}
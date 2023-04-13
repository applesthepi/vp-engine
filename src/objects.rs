pub mod ui;

pub trait Object {
	fn name(&self) -> &String;
}
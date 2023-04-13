use std::sync::Arc;

use crate::Object;

pub struct Bucket {
	pub name: String,
	pipeline: Box<dyn vpb::Pipeline>,
	objects: Vec<Arc<dyn Object>>,
}

impl Bucket {
	pub fn new(
		name: &str,
		pipeline: Box<dyn vpb::Pipeline>,
	) -> Self {
		let name = name.to_string();
		let objects: Vec<Arc<dyn Object>> = Vec::with_capacity(1024);
		Self {
			name,
			pipeline,
			objects,
		}
	}

	pub fn get_object(
		&self,
		name: &str,
	) -> Arc<dyn Object> {
		self.objects.iter().find(
			|x|
			x.name() == name
		).expect(format!("no object with name \"{}\" inside bucket \"{}\"", name, self.name).as_str()).clone()
	}

	pub fn add_object(
		&mut self,
		object: Arc<dyn Object>,
	) {
		self.objects.push(object);
	}
}
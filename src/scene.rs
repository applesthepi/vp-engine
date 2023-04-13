mod bucket;
use std::sync::Arc;

pub use bucket::*;

pub struct Scene {
	buckets: Vec<Box<Bucket>>,
}

impl Scene {
	pub fn new(

	) -> Self {
		let buckets: Vec<Box<Bucket>> = Vec::with_capacity(8);
		Self {
			buckets,
		}
	}

	pub fn get_bucket(
		&mut self,
		name: &str,
	) -> &mut Bucket {
		self.buckets.iter_mut().find(
			|x|
			x.name == name
		).expect(format!("no bucket with name \"{}\"", name).as_str())
	}

	pub fn render(
		&self,
	) {

	}
}
use crate::Scene;

pub trait UpdateElement {
	fn update(&mut self, scene: &mut Scene);
}
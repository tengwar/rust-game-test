use specs::{VecStorage};

#[derive(Component, Debug)]
#[component(VecStorage)]
pub struct Velocity {
	pub x: f32,
	pub y: f32
}

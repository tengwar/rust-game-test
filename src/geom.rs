use specs::{VecStorage};

#[derive(Component, Debug)]
#[component(VecStorage)]
pub struct Position {
	pub x: f32,
	pub y: f32
}

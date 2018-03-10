use specs::{VecStorage};

#[derive(Component, Debug)]
#[component(VecStorage)]
pub struct Position { // TODO: move it to a geometry module?
	pub x: f32,
	pub y: f32
}

#[derive(Component, Debug)]
#[component(VecStorage)]
pub struct Velocity {
	pub x: f32,
	pub y: f32
}

use specs::{ReadStorage, System};
use physics::Position;

pub struct Renderer;

impl<'a> System<'a> for Renderer {
	type SystemData = ReadStorage<'a, Position>;

	fn run(&mut self, position: Self::SystemData) {
		use specs::Join;

		for position in position.join() {
			println!("Hello, {:?}", &position);
		}
	}
}

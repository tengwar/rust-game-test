use specs::{Fetch, ReadStorage, System};
use geom::Position;

extern crate ggez;
use ggez::graphics as gfx;


pub struct Renderer;

impl<'a> System<'a> for Renderer {
	type SystemData = (Fetch<'a, ggez::Context>,
	                   ReadStorage<'a, Position>);

	fn run(&mut self, data: Self::SystemData) {
		use specs::Join;

		let (ctx, position) = data;

		for position in position.join() {
			gfx::circle(ctx, gfx::DrawMode::Fill, position, 2.0, 1.0);
			println!("Hello, {:?}", &position);
		}
	}
}

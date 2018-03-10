extern crate specs;
#[macro_use]
extern crate specs_derive;

use specs::{RunNow, World};
mod physics;
mod draw;

use physics::Position;
use draw::Renderer;

fn main() {
    // World
    let mut world = World::new();
    world.register::<Position>();

    // Entities
    let ball = world.create_entity().with(Position { x: 4.0, y: 7.0 }).build();
    let floor = world.create_entity().with(Position { x: 0.0, y: 0.0 }).build();

    // Run
    let mut hello_world = Renderer;
    hello_world.run_now(&world.res);
}

extern crate specs;
#[macro_use]
extern crate specs_derive;
use specs::{RunNow, World};

extern crate ggez;
use ggez::{Context, ContextBuilder};
use ggez::conf;

mod geom;
mod physics;
mod draw;

use geom::Position;
use draw::Renderer;

fn main() {
    // World
    let mut world = World::new();
    world.register::<Position>();

    // Entities
    let ball = world.create_entity().with(Position { x: 4.0, y: 7.0 }).build();
    let floor = world.create_entity().with(Position { x: 0.0, y: 0.0 }).build();

    // ggez context
    let mut cb = ContextBuilder::new("game", "author")
        .window_setup(conf::WindowSetup::default()
            .title("Game")
        )
        .window_mode(conf::WindowMode::default()
            .dimensions(640, 480)
        );
    let ctx = &mut cb.build().expect("ggez context creation failed");
    world.add_resource(ctx);

    // Run
    let mut hello_world = Renderer;
    hello_world.run_now(&world.res);
}

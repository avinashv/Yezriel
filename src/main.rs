mod map;
mod player;

mod prelude {
    pub use bracket_lib::prelude::*;

    pub use crate::map::*;
    pub use crate::player::*;

    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
}

use prelude::*;

struct State {
    map: Map,
    player: Player,
}

impl State {
    fn new() -> Self {
        Self {
            map: Map::new(),
            player: Player::new(Point::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2)),
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        // game loop
        ctx.cls();
        self.player.update(ctx, &self.map);
        self.map.render(ctx);
        self.player.render(ctx);
    }
}

fn main() -> BError {
    let ctx = BTermBuilder::simple80x50()
        .with_title("Yezriel")
        .with_fps_cap(60.0)
        .build()?;

    main_loop(ctx, State::new())
}

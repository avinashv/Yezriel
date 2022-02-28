mod prelude {
    pub use bracket_lib::prelude::*;
}

use prelude::*;

struct State {}
impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print(1, 1, "Hello, world!");
    }
}

fn main() -> BError {
    let ctx = BTermBuilder::simple80x50()
        .with_title("Yezriel")
        .with_fps_cap(60.0)
        .build()?;

    main_loop(ctx, State {})
}

use tetra::graphics::{self, Color, Texture};
use tetra::input::{self, Key};
use tetra::math::Vec2;
use tetra::{Context, ContextBuilder, State};

struct GameState {
  paddle_texture: Texture,
}

impl State for GameState {
  fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
    graphics::clear(ctx, Color::rgb(0.392, 0.584, 0.929));

    Ok(())
  }
}

fn main() -> tetra::Result {
    ContextBuilder::new("Pong", 640, 480)
      .quit_on_escape(true)
      .build()?
      .run(|ctx| {
        let paddle_texture = Texture::new(ctx, "./resources/player1.png")?;

        Ok(GameState {paddle_texture})
      })
}

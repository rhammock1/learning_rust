use tetra::graphics::{self, Color, Texture};
use tetra::input::{self, Key};
use tetra::math::Vec2;
use tetra::{Context, ContextBuilder, State};

// It will be easier to use these values as floats and cast to i32 when
// we need to.
const WINDOW_HEIGHT: f32 = 480.0;
const WINDOW_WIDTH: f32 = 640.0;

struct GameState {
  // We add texture to our game struct so that the texture stays
  // loaded until the game is closed.
  // Texture is effectively an id, so it's cheap to clone.
  paddle_texture: Texture,
}

impl GameState {
  fn new(ctx: &mut Context) -> tetra::Result<GameState> {
    // Texture is a type that represents image data that has been loaded
    // into graphics memory.
    let paddle_texture = Texture::new(ctx, "./resources/player1.png")?;
    Ok(GameState {paddle_texture})
  }
}

impl State for GameState {
  fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
    graphics::clear(ctx, Color::rgb(0.392, 0.584, 0.929));

    // Draws the paddle to the screen at position (16, 16).
    // According to the docs, the second parameter is of the type
    // Into<DrawParams>, but Vec2 is automatically converted to this type.
    self.paddle_texture.draw(ctx, Vec2::new(16.0, 16.0));

    Ok(())
  }
}

fn main() -> tetra::Result {
    ContextBuilder::new("Pong", WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32)
      .quit_on_escape(true)
      .build()?
      // Because our constructor function's signature and the run closure's
      // signature are the same, we can use GameState::new directly.
      .run(GameState::new)
}

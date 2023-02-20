use tetra::graphics::{self, Color, Texture};
use tetra::input::{self, Key};
use tetra::math::Vec2;
use tetra::{Context, ContextBuilder, State};

// It will be easier to use these values as floats and cast to i32 when
// we need to.
const WINDOW_HEIGHT: f32 = 480.0;
const WINDOW_WIDTH: f32 = 640.0;

const PADDLE_SPEED: f32 = 16.0;

struct Entity {
  texture: Texture,
  position: Vec2<f32>,
}

impl Entity {
  fn new(texture: Texture, position: Vec2<f32>) -> Entity {
    Entity {
      texture,
      position,
    }
  }
}

struct GameState {
  player1: Entity,
  player2: Entity,
}

impl GameState {
  fn new(ctx: &mut Context) -> tetra::Result<GameState> {
    // Texture is a type that represents image data that has been loaded
    // into graphics memory.
    let player1_texture = Texture::new(ctx, "./resources/player1.png")?;
    let player1_position = Vec2::new(
      16.0,
      // Offset so that the paddle is vertically centered on start up.
      (WINDOW_HEIGHT - player1_texture.height() as f32) / 2.0,
    );

    let player2_texture = Texture::new(ctx, "./resources/player2.png")?;
    let player2_position = Vec2::new(
      WINDOW_WIDTH - player2_texture.width() as f32 - 16.0,
      // Offset so that the paddle is vertically centered on start up.
      (WINDOW_HEIGHT - player2_texture.height() as f32) / 2.0,
    );

    Ok(GameState {
      player1: Entity::new(player1_texture, player1_position),
      player2: Entity::new(player2_texture, player2_position),
    })
  }
}

impl State for GameState {
  fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
    graphics::clear(ctx, Color::rgb(0.392, 0.584, 0.929));

    // Draws the paddle to the screen at position (16, 16).
    // According to the docs, the second parameter is of the type
    // Into<DrawParams>, but Vec2 is automatically converted to this type.
    self.player1.texture.draw(ctx, self.player1.position);
    self.player2.texture.draw(ctx, self.player2.position);

    Ok(())
  }

  fn update(&mut self, ctx: &mut Context) -> tetra::Result {
    /*             Player 1 Controls               */
    if input::is_key_down(ctx, Key::W) {
      self.player1.position.y -= PADDLE_SPEED;
    }

    if input::is_key_down(ctx, Key::S) {
      self.player1.position.y += PADDLE_SPEED;
    }

    /*             Player 2 Controls               */
    if input::is_key_down(ctx, Key::Up) {
      self.player2.position.y -= PADDLE_SPEED;
    }

    if input::is_key_down(ctx, Key::Down) {
      self.player2.position.y += PADDLE_SPEED;
    }

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

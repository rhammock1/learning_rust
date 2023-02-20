use tetra::graphics::{self, Color, Rectangle, Texture};
use tetra::input::{self, Key}; // Key Press
use tetra::math::Vec2;
use tetra::window;
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
  ball: Entity,
}

impl GameState {
  fn new(ctx: &mut Context) -> tetra::Result<GameState> {
    // Texture is a type that represents image data that has been loaded
    // into graphics memory.

    /* Player1 Set Up */
    let player1_texture = Texture::new(ctx, "./resources/player1.png")?;
    let player1_position = Vec2::new(
      16.0,
      // Offset so that the paddle is vertically centered on start up.
      (WINDOW_HEIGHT - player1_texture.height() as f32) / 2.0,
    );

    /* Player2 Set Up */
    let player2_texture = Texture::new(ctx, "./resources/player2.png")?;
    let player2_position = Vec2::new(
      WINDOW_WIDTH - player2_texture.width() as f32 - 16.0,
      // Offset so that the paddle is vertically centered on start up.
      (WINDOW_HEIGHT - player2_texture.height() as f32) / 2.0,
    );

    /* Ball Set Up */
    let ball_texture = Texture::new(ctx, "./resources/ball.png")?;
    let ball_position = Vec2::new(
      WINDOW_WIDTH / 2.0 - ball_texture.width() as f32 / 2.0,
      WINDOW_HEIGHT / 2.0 - ball_texture.height() as f32 / 2.0,
    );

    Ok(GameState {
      player1: Entity::new(player1_texture, player1_position),
      player2: Entity::new(player2_texture, player2_position),
      ball: Entity::new(ball_texture, ball_position),
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
    self.ball.texture.draw(ctx, self.ball.position);

    Ok(())
  }

  fn update(&mut self, ctx: &mut Context) -> tetra::Result {
    /*             Player 1 Controls               */
    if input::is_key_down(ctx, Key::W) {
      // Up
      self.player1.position.y -= PADDLE_SPEED;
    }

    if input::is_key_down(ctx, Key::S) {
      // Down
      self.player1.position.y += PADDLE_SPEED;
    }

    // Make sure the paddle doesn't go off the screen.
    if self.player1.position.y > WINDOW_HEIGHT - self.player1.texture.height() as f32 {
      self.player1.position.y = WINDOW_HEIGHT - self.player1.texture.height() as f32;
    } else if self.player1.position.y < 0 as f32 {
      self.player1.position.y = 0 as f32;
    }

    /*             Player 2 Controls               */
    if input::is_key_down(ctx, Key::Up) {
      // Up
      self.player2.position.y -= PADDLE_SPEED;
    }

    if input::is_key_down(ctx, Key::Down) {
      // Down
      self.player2.position.y += PADDLE_SPEED;
    }

    // Make sure the paddle doesn't go off the screen.
    if self.player2.position.y > WINDOW_HEIGHT - self.player2.texture.height() as f32 {
      self.player2.position.y = WINDOW_HEIGHT - self.player2.texture.height() as f32;
    } else if self.player2.position.y < 0 as f32 {
      self.player2.position.y = 0 as f32;
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

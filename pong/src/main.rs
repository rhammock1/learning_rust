use tetra::graphics::{self, Color, Rectangle, Texture, text::{Font, Text}};
use tetra::input::{self, Key}; // Key Press
use tetra::math::Vec2;
use tetra::{Context, ContextBuilder, State};

// It will be easier to use these values as floats and cast to i32 when
// we need to.
const WINDOW_HEIGHT: f32 = 480.0;
const WINDOW_WIDTH: f32 = 640.0;

// Ball and Paddle Constraints
const PADDLE_SPEED: f32 = 16.0;
const PADDLE_SPIN: f32 = 4.0;
const BALL_SPEED: f32 = 5.0;
const BALL_ACC: f32 = 0.05;

// Game Paramters
const WINNING_SCORE: i32 = 5;

enum EntityType {
  Player,
  Ball,
}

struct Entity {
  texture: Texture,
  position: Vec2<f32>,
  velocity: Vec2<f32>,
  score: i32,
  entity_type: EntityType,
}

impl Entity {
  fn new(texture: Texture, position: Vec2<f32>) -> Entity {
    Entity::with_velocity(texture, position, Vec2::zero(), EntityType::Player)
  }

  fn with_velocity(texture: Texture, position: Vec2<f32>, velocity: Vec2<f32>, entity_type: EntityType) -> Entity {
    Entity {
      texture,
      position,
      velocity,
      entity_type,
      score: 0,
    }
  }

  // To make the ball collide with the paddles and the walls, we will
  // implement *axis-aligned bounding boxes* (AABBs). This technique
  // takes a rectange and does some math to determine if that rectangle
  // intersects with another rectangle.

  fn width(&self) -> f32 {
    self.texture.width() as f32
  }

  fn height(&self) -> f32 {
    self.texture.height() as f32
  }

  fn bounds(&self) -> Rectangle {
    Rectangle::new(
      self.position.x,
      self.position.y,
      self.width(),
      self.height(),
    )
  }

  fn center(&self) -> Vec2<f32> {
    Vec2::new(
      self.position.x + self.width() / 2.0,
      self.position.y + self.height() / 2.0,
    )
  }

  fn reset(&mut self) -> () {
    match self.entity_type {
      EntityType::Player => {
        self.position.y = (WINDOW_HEIGHT - self.texture.height() as f32) / 2.0
      },
      EntityType::Ball => {
        // used to reset the ball to the center of the screen
        self.position.x = WINDOW_WIDTH / 2.0 - self.width() / 2.0;
        self.position.y = WINDOW_HEIGHT / 2.0 - self.height() / 2.0;
        self.velocity.x = -(self.velocity.x.signum()) * BALL_SPEED;
        self.velocity.y = 0.0;
      }
    }
  }
}

struct GameState {
  player1: Entity,
  player2: Entity,
  ball: Entity,
  score: Text,
  winner: String,
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
    let ball_veloctiy = Vec2::new(-BALL_SPEED, 0.0);

    /* Score Set Up */
    let score_text = format!("{} - {}", 0, 0);
    let score_text = Text::new(score_text, Font::vector(ctx, "./resources/Arial.ttf", 32.0)?);

    Ok(GameState {
      player1: Entity::new(player1_texture, player1_position),
      player2: Entity::new(player2_texture, player2_position),
      ball: Entity::with_velocity(ball_texture, ball_position, ball_veloctiy, EntityType::Ball),
      score: score_text,
      winner: String::from(""),
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
    if self.winner == "" {
      self.ball.texture.draw(ctx, self.ball.position);
    }

    // Draw the score
    let score_text = format!("{} - {}", self.player1.score, self.player2.score);
    self.score = Text::new(score_text, Font::vector(ctx, "./resources/Arial.ttf", 32.0)?);
    self.score.draw(ctx, Vec2::new(WINDOW_WIDTH / 2.2, 16.0)); // Doesn't exactly center the text
    // need to get the bounds of the text and use that to calculate the center

    if self.winner != "" {
      let win_text = format!("{} Wins!", self.winner);
      let mut winner_text = Text::new(win_text, Font::vector(ctx, "./resources/Arial.ttf", 48.0)?);
      winner_text.draw(ctx, Vec2::new(WINDOW_WIDTH / 2.0, WINDOW_HEIGHT / 2.0));
      // Add play again button that resets everything
    }

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

    /*             Ball Controls               */
    let player1_bounds = self.player1.bounds();
    let player2_bounds = self.player2.bounds();
    let ball_bounds = self.ball.bounds();

    let paddle_hit = if ball_bounds.intersects(&player1_bounds) {
      Some(&self.player1)
    } else if ball_bounds.intersects(&player2_bounds) {
      Some(&self.player2)
    } else {
      None
    };

    if let Some(paddle) = paddle_hit {
      // Increase the ball's velocity, then flip it.
      self.ball.velocity.x = 
        -(self.ball.velocity.x + (BALL_ACC * self.ball.velocity.x.signum()));
      
      // Calculat ethe offset between the paddle and the ball, as a number
      // between -1 and 1.
      let offset = (paddle.center().y - self.ball.center().y) / paddle.height();

      // Apply the spin to the ball
      self.ball.velocity.y += PADDLE_SPIN * -offset;
    }

    self.ball.position += self.ball.velocity;

    if self.ball.position.y <= 0.0 
      || self.ball.position.y + self.ball.height() >= WINDOW_HEIGHT {
        self.ball.velocity.y = -self.ball.velocity.y;
    }

    /*        Score Counting          */
    if self.ball.position.x < 0.0 {
      self.player2.score += 1;
      self.player1.reset();
      self.player2.reset();
      self.ball.reset();
    }

    if self.ball.position.x > WINDOW_WIDTH {
      self.player1.score += 1;
      self.player1.reset();
      self.player2.reset();
      self.ball.reset();
    }

    if self.player1.score == WINNING_SCORE {
      self.winner = String::from("Player 1");
    } else if self.player2.score == WINNING_SCORE {
      self.winner = String::from("Player 2");
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

// {self} here allows us to use simple 'graphics' but not 'terra::graphics' in the future
use tetra::graphics::{self, Color, Texture, Rectangle};
use tetra::graphics::text::{Font, Text};
use tetra::input::{self, Key};
use tetra::math::Vec2;
use tetra::window;
use tetra::{Context, ContextBuilder, State};
use rand::Rng;


// Size of the window
const WINDOW_HEIGHT: f32 = 480.0;
const WINDOW_WIDTH: f32 = 640.0;
// Paddle movement speed
const PADDLE_SPEED: f32 = 8.0;
// Ball movement speed
const BALL_SPEED: f32 = 5.0;

const PADDLE_SPIN: f32 = 4.0;

const PLAYER1_TEXT_OFFSET: Vec2<f32> = Vec2::new(16.0, 16.0);
const PLAYER2_TEXT_OFFSET: Vec2<f32> = Vec2::new(WINDOW_WIDTH - 92.0, 16.0);

const MAX_POINTS: i32 = 10;

// Holds info about player
struct PlayerScore{
    score: i32,
}

impl PlayerScore{
    fn new(score: i32) -> PlayerScore{
        PlayerScore{score}
    }

}

// Holds info about game entity - paddle, ball or any other moving object
struct Entity {
    texture: Texture,
    position: Vec2<f32>,
    velocity: Vec2<f32>,
}

impl Entity {
    // Constructor of a new player entity
    fn new(texture: Texture, position: Vec2<f32>) -> Entity{
        Entity::with_velocity(texture, position, Vec2::zero())
    }

    // Constructor of a new ball entity
    fn with_velocity(texture: Texture, position: Vec2<f32>, velocity: Vec2<f32>) -> Entity{
        Entity{texture, position, velocity}
    }


    // Size of entity
    fn width(&self) -> f32{
        self.texture.width() as f32
    }

    fn height(&self) -> f32{
        self.texture.height() as f32
    }

    // Rectangle bounds of an entity
    fn bounds(&self) -> Rectangle{
        Rectangle::new(
            self.position.x,
            self.position.y,
            self.width(),
            self.height(),
        )
    }

    // Gives the center of the object
    fn centre(&self) -> Vec2<f32>{
        Vec2::new(
            self.position.x + (self.width() / 2.0),
            self.position.y + (self.height() / 2.0),
        )
    }
}

/*
Context is a struct that holds all of the 'global' state managed by the framework, 
such as window settings and connections to the graphics/audio/input hardware. 
Any function in Tetra's API that requires access to this state will take a reference to a
Context as the first parameter
*/

/*
State is a trait exposed by Tetra, which is implemented for the type that stores your game's state. 
It exposes various methods that will be called during the game loop, and you can override these in 
order to define your game's behaviour.
*/

struct GameState {
    player1: Entity,
    player2: Entity,
    ball: Entity,
    player1_score: PlayerScore,
    player2_score: PlayerScore,
    player1_text: Text,
    player2_text: Text,

}


impl GameState {
    // Constructor for game state
    fn new(ctx: &mut Context) -> tetra::Result<GameState>{
        // First player
        // Just load the texture without drawing it
        let player1_texture = Texture::new(ctx, "./resources/player1.png")?;
        // Set the player's position
        let player1_position = Vec2::new(
            16.0, (WINDOW_HEIGHT - player1_texture.height() as f32) / 2.0
            );
        // Create an entity of first player
        let player1 = Entity::new(player1_texture, player1_position);

        // Second player
        let player2_texture = Texture::new(ctx, "./resources/player2.png")?;
        let player2_position = Vec2::new(
            WINDOW_WIDTH - player2_texture.width() as f32 - 16.0,
            (WINDOW_HEIGHT - player2_texture.height() as f32) / 2.0,
        );
        let player2 = Entity::new(player2_texture, player2_position);

        // Ball
        let ball_texture = Texture::new(ctx, "./resources/ball.png")?;
        let ball_position = Vec2::new(
            WINDOW_WIDTH / 2.0 - ball_texture.width() as f32 / 2.0,
            WINDOW_HEIGHT / 2.0 - ball_texture.height() as f32 / 2.0,
        );
        let ball_velocity = GameState::set_ball_velocity();
        let ball = Entity::with_velocity(ball_texture, ball_position, ball_velocity);

        // Score
        let player1_score = PlayerScore::new(0);
        let player2_score = PlayerScore::new(0);

        // Text
        let player1_text = Text::new(
                format!("Score: {}", player1_score.score), Font::bmfont(ctx, "./resources/DejaVuSansMono.fnt")?,
            );
        let player2_text = Text::new(
                format!("Score: {}", player2_score.score), Font::bmfont(ctx, "./resources/DejaVuSansMono.fnt")?,
            );

        Ok(GameState{
            player1,
            player2,
            ball,
            player1_text,
            player2_text,
            player1_score,
            player2_score,
        })
    }

    // Sets random direction of ball movement
    fn set_ball_velocity() -> Vec2<f32> {
        let first_hit_player = rand::thread_rng().gen_range(0..2);
        let mut ball_velocity = Vec2::new(0.0, 0.0);
        if first_hit_player == 0 {
            ball_velocity = Vec2::new(BALL_SPEED, 0.0);
        } else if first_hit_player == 1 {
            ball_velocity = Vec2::new(-BALL_SPEED, 0.0);
        }
        ball_velocity 
    }
}

impl State for GameState {
    // Rendering function
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result{
        // Fill the screen with background color
        graphics::clear(ctx, Color::rgb(0.392, 0.584, 0.929));
        // Draw both players' paddles
        self.player1.texture.draw(ctx, self.player1.position);
        self.player2.texture.draw(ctx, self.player2.position);

        // Draw the ball
        self.ball.texture.draw(ctx, self.ball.position);

        // Draw the text
        self.player1_text.draw(ctx, PLAYER1_TEXT_OFFSET);
        self.player2_text.draw(ctx, PLAYER2_TEXT_OFFSET);

        // It should return Result
        Ok(())
    }

    // Game state updating function. Works 60 times a second
    fn update(&mut self, ctx: &mut Context) -> tetra::Result{
        // Move the first player's paddle up when W is pressed
        if input::is_key_down(ctx, Key::W){
            self.player1.position.y -= PADDLE_SPEED;
        }
        // Move the first player's paddle down when S is pressed
        if input::is_key_down(ctx, Key::S){
            self.player1.position.y += PADDLE_SPEED;
        }

        // Move the second player's paddle up when UP arrow is pressed
        if input::is_key_down(ctx, Key::O){
            self.player2.position.y -= PADDLE_SPEED;
        }

        // Move the second player's paddle down when DOWN arrow is pressed
        if input::is_key_down(ctx, Key::K){
            self.player2.position.y += PADDLE_SPEED;
        }

        // Check if ball intersects with any paddle
        let player1_bounds = self.player1.bounds();
        let player2_bounds = self.player2.bounds();
        let ball_bounds = self.ball.bounds();

        let paddle_hit = if ball_bounds.intersects(&player1_bounds){
            Some(&self.player1)
        } else if ball_bounds.intersects(&player2_bounds){
            Some(&self.player2)
        }  else {
            None
        } ;

        if let Some(paddle) = paddle_hit {
            // Flip ball's direction
            self.ball.velocity.x = -self.ball.velocity.x;

            // Calculate the offset between the paddle and the ball
            let offset = (paddle.centre().y - self.ball.centre().y) / paddle.height();

            // Apply the spin ti the ball
            self.ball.velocity.y += PADDLE_SPIN * -offset;
        }

        // Make sure the ball can't move out of the screen
        if self.ball.position.y <= 0.0 || self.ball.position.y + self.ball.height() >= WINDOW_HEIGHT{
            self.ball.velocity.y = -self.ball.velocity.y;
        }

        // Pick a winner
        if self.ball.position.x < 0.0 {
            self.player2_score.score += 1;
            self.player2_text = Text::new(
                    format!("Score: {}", self.player2_score.score), Font::bmfont(ctx, "./resources/DejaVuSansMono.fnt")?,
                );
            // Reset the ball to the middle of the screen
            self.ball.position = Vec2::new(
                WINDOW_WIDTH / 2.0 - self.ball.width() as f32 / 2.0,
                WINDOW_HEIGHT / 2.0 - self.ball.height() as f32 / 2.0,
            );
            // Random player makes the first hit
            let first_hit_player = rand::thread_rng().gen_range(0..2);
            if first_hit_player == 0 {
                self.ball.velocity = Vec2::new(BALL_SPEED, 0.0);
            } else if first_hit_player == 1 {
                self.ball.velocity = Vec2::new(-BALL_SPEED, 0.0);
            }
        }

        if self.ball.position.x > WINDOW_WIDTH {
            self.player1_score.score += 1;
            self.player1_text = Text::new(
                    format!("Score: {}", self.player1_score.score), Font::bmfont(ctx, "./resources/DejaVuSansMono.fnt")?,
                );
            // Reset the ball to the middle of the screen
            self.ball.position = Vec2::new(
                WINDOW_WIDTH / 2.0 - self.ball.width() as f32 / 2.0,
                WINDOW_HEIGHT / 2.0 - self.ball.height() as f32 / 2.0,
            );

            // Random player makes the first hit
            let first_hit_player = rand::thread_rng().gen_range(0..2);
            if first_hit_player == 0 {
                self.ball.velocity = Vec2::new(BALL_SPEED, 0.0);
            } else if first_hit_player == 1 {
                self.ball.velocity = Vec2::new(-BALL_SPEED, 0.0);
            }

        }


        // If one player gets max points - the game is over
        if self.player1_score.score == MAX_POINTS {
            window::quit(ctx);
            println!("PLAYER 1 WINS!");
        } else if self.player2_score.score == MAX_POINTS {
            window::quit(ctx);
            println!("PLAYER 2 WINS!");
        }

        // Update ball's position each time
        self.ball.position += self.ball.velocity;
        Ok(())
    }
}

fn main() -> tetra::Result{
    // Create a game window
    ContextBuilder::new("Pong", WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32)
        // Close the windows when hitting Esc
        .quit_on_escape(true)
        // Build the whole game
        .build()?
        // Running the builder we create a new game state
        .run(|ctx| GameState::new(ctx))

}



/*

TODO

-Make a good menu after one player winning 
-Make a random player to do the first hit. THe ball should not mantain it's speed after resetting to the center of the screen (DONE)
-Make the random angle of the first hit
*/
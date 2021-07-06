// {self} here allows us to use simple 'graphics' but not 'terra::graphics' in the future
use tetra::graphics::{self, Color, Texture};
use tetra::input::{self, Key};
use tetra::math::Vec2;
use tetra::{Context, ContextBuilder, State};



const WINDOW_HEIGHT: f32 = 480.0;
const WINDOW_WIDTH: f32 = 640.0;


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
    paddle_texture: Texture
}

impl GameState {
      // Constructor for game state
    fn new(ctx: &mut Context) -> tetra::Result<GameState>{
        // Just load the texture without drawing it
        let paddle_texture = Texture::new(ctx, "./resources/player1.png")?;

        Ok(GameState{paddle_texture})
    }
}

impl State for GameState {
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result{
        // Fill the screen with background color
        graphics::clear(ctx, Color::rgb(0.392, 0.584, 0.929));
        // Draw the first player paddle at position 16 16
        self.paddle_texture.draw(ctx, Vec2::new(16.0, 16.0));

        // It should return Result
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
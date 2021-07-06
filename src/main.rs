use tetra::graphics::{self, Color};
use tetra::{Context, ContextBuilder, State};

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

struct GameState {}
impl State for GameState {}

fn main() -> tetra::Result{
    ContextBuilder::new("Pong", 640, 480)
    .quit_on_escape(true)
    .build()?
    .run(|_| {Ok(GameState{})})

}
mod input_system;
mod game_objects;
mod game_states;
mod utils;

use game_states::*;
use utils::*;

fn main() {
    let (mut rl, thread) = init_window();
    let mut active_state = GameLoop::new();

    // Each frame
    while !rl.window_should_close() {
        active_state.update(&rl);
        active_state.draw(&mut rl, &thread);
    }
}
mod input_system;
mod game_objects;
mod game_states;
mod utils;

use game_states::*;
use utils::*;

fn main() {
    let (mut rl, thread) = init_window();
    let mut g_state: Box<dyn GameState> = Box::new(MainMenu::new());

    // Each frame
    while !rl.window_should_close() {
        // Change current game state if needed
        if !g_state.is_active() {
            g_state = g_state.get_next_state(); 
        }

        g_state.update(&rl);
        g_state.draw(&mut rl, &thread);
    }

}
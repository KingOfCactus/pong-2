mod ui_system;
mod input_system;
mod game_objects;
mod game_scenes;
mod networking;
mod utils;

use game_scenes::*;
use utils::*;


fn main() {
    if is_debug_session() { debug() }     
    let (mut rl, thread) = init_window();
    let mut scene: Box<dyn GameScene> = Box::new(MainMenu::new());

    // Each frame
    while !rl.window_should_close() {
        if !scene.is_active() {
            scene = scene.get_next_scene(&rl); 
        }

        scene.update(&rl);
        scene.draw(&mut rl, &thread);
    }
}
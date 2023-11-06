mod input_system;
mod game_objects;
mod game_scenes;
mod utils;

use game_scenes::*;
use utils::*;

fn main() {
    let (mut rl, thread) = init_window();
    let mut scene: Box<dyn GameScene> = Box::new(MainMenu::new());

    // Each frame
    while !rl.window_should_close() {
        if !scene.is_active() {
            scene = scene.get_next_scene(); 
        }

        scene.update(&rl);
        scene.draw(&mut rl, &thread);
    }
}
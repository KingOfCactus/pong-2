mod main_menu;
mod game_loop;

use raylib::prelude::*;
use crate::game_objects::*;

use self::main_menu::*;

pub trait GameState {
    fn is_active(&self) -> bool;
    fn get_next_state(&self) -> Box<dyn GameState>;

    fn update(&mut self, rl: &RaylibHandle);
    fn draw(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread); // Needs to be the last called method, since it drops the RaylibHandle
}

pub struct GameLoop {
    score: i32,
    hiscore: i32,
    debug_mode: bool,

    player: Ball,
    left_paddle: Paddle,
    right_paddle: Paddle,

    is_active: bool
}

pub struct MainMenu {
    title: Text,
    play: Button,
    config: Button,
    quit: Button,

    is_active: bool
}
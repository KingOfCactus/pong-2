mod main_menu;
mod game_loop;

use raylib::prelude::*;
use crate::{game_objects::*, input_system::PlayerInput};

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
    score_color: Color,
    
    ball: Ball,
    checkpoint: i32,
    respawn_timer: f32,
    bounced_vertically: bool,
    
    left_paddle: Paddle,
    right_paddle: Paddle,
    
    players_input: Vec<PlayerInput>,

    is_active: bool,
    debug_mode: bool
}

pub struct MainMenu {
    title: Text,
    singleplayer: Button,
    quit: Button,
    hiscore: Text,
    
    is_active: bool
}
mod main_menu;
mod game_loop;
use std::panic;

use raylib::prelude::*;
use crate::game_objects::*;
use crate::ui_system::*;
use crate::input_system::PlayerInput;

use self::main_menu::*;

#[derive(Clone, Copy, PartialEq)]
pub enum GameMode { None, Singleplayer, Multiplayer }

pub trait GameScene {
    fn is_active(&self) -> bool;
    fn get_next_scene(&self, rl: &RaylibHandle) -> Box<dyn GameScene>;
    
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
    debug_mode: bool,
    game_mode: GameMode,
}

pub struct MainMenu {
    current_screen: Box<dyn UIScreen>,
    is_active: bool
}
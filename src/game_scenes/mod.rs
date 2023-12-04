mod main_menu;
mod game_loop;

use raylib::prelude::*;
use crate::{game_objects::*, input_system::PlayerInput};

use self::main_menu::*;

#[derive(Clone, Copy, PartialEq)]
pub enum GameMode { Singleplayer, Multiplayer }

pub trait GameScene {
    fn is_active(&self) -> bool;
    fn get_next_scene(&self) -> Box<dyn GameScene>;
    
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
    title: Text,
    hiscore: Text,

    singleplayer: Button,
    multiplayer: Button,
    quit: Button,

    on_mltplyr_screen: bool,
    local_multiplayer: Button,
    online_multiplayer: Button,

    selected_mode: GameMode,
    is_active: bool
}
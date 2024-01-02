use super::*;
use crate::{utils::{SCREEN_SIZE, get_highscore, self}, input_system::InputDevice};

const HOVERING_BTN_COLOR: Color = Color::WHITE;
const BTN_COLOR: Color = Color::new(150, 150, 150, 255);

pub struct Text {
    color: Color,
    text: String,
    size: i32,
    pos: Vector2,
}

pub struct Button {
    rect: Rectangle,
    text: String,
    pos: Vector2,
    focused: bool,
}

impl Text {
    pub fn new(text: &str, relative_pos: Vector2, color: Color, size: i32,) -> Text {
        Text {
            pos: Vector2 { 
                x: SCREEN_SIZE.x * relative_pos.x - measure_text(text, size) as f32 / 2.0,
                y: SCREEN_SIZE.y * relative_pos.y 
            },
            text: text.to_string(), color: color, size: size,
        }
    }
}

impl Button {
    pub fn new(text: &str, relative_pos: Vector2) -> Button {
        Button {
            text: text.to_string(),
            pos: Vector2 {
                x: SCREEN_SIZE.x * relative_pos.x - measure_text(&text, 20) as f32 / 2.0,
                y: SCREEN_SIZE.y * relative_pos.y
            },
            
            focused: false,
            rect: Rectangle::new (
                SCREEN_SIZE.x * relative_pos.x as f32 - (measure_text(&text, 20) + 30) as f32 / 2.0, 
                SCREEN_SIZE.y * relative_pos.y - 10.0, measure_text(&text, 20) as f32 + 30.0, 40.0
            )
        }
    }
}

impl GameScene for MainMenu {
    fn update(self: &mut Self, rl: &RaylibHandle) {
        let mouse_pos = rl.get_mouse_position();

        // Update buttons
        self.singleplayer.focused = self.singleplayer.rect.check_collision_point_rec(mouse_pos) && !self.on_mltplyr_screen && !self.on_devices_screen;
        self.multiplayer.focused = self.multiplayer.rect.check_collision_point_rec(mouse_pos) && !self.on_mltplyr_screen;
        self.quit.focused = self.quit.rect.check_collision_point_rec(mouse_pos) && !self.on_mltplyr_screen;

        self.local_multiplayer.focused = self.local_multiplayer.rect.check_collision_point_rec(mouse_pos) && self.on_mltplyr_screen;
        self.online_multiplayer.focused = self.online_multiplayer.rect.check_collision_point_rec(mouse_pos) && self.on_mltplyr_screen;

        // Select devices buttons 
        for button in &mut self.select_devices_btns {
            button.focused = button.rect.check_collision_point_rec(mouse_pos) && self.on_devices_screen;
        }   

        // Exit if mouse isn't clicking
        if !rl.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) { return; }

        // Main screen
        if self.singleplayer.focused { self.start_game(false); }
        if self.multiplayer.focused { self.on_devices_screen = true; }
        if self.quit.focused { self.quit(); }

        // Multiplayer screen
        if self.local_multiplayer.focused { self.start_game(true); }
        if self.online_multiplayer.focused { self.quit(); }
        
        // Devices screen
        if self.select_devices_btns[0].focused { self.change_selected_device(0, 1, rl) }
        if self.select_devices_btns[1].focused { self.change_selected_device(0, -1, rl) }
        if self.select_devices_btns[2].focused { self.change_selected_device(1, 1, rl) }
        if self.select_devices_btns[3].focused { self.change_selected_device(1, -1, rl) }

        return;
        todo!("Remove long btn's boolean expressions");
    }

    fn draw(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread) {
        // Clear screen
        let mut draw_handle = rl.begin_drawing(thread);
        draw_handle.clear_background(Color::BLACK);
        
        // Multiplayer Screen
        if self.on_mltplyr_screen {
            draw_handle.draw_text(&self.local_multiplayer.text, self.local_multiplayer.pos.x as i32, self.local_multiplayer.pos.y as i32,
                20 as i32, if self.local_multiplayer.focused {HOVERING_BTN_COLOR} else {BTN_COLOR});

            draw_handle.draw_text(&self.online_multiplayer.text, self.online_multiplayer.pos.x as i32, self.online_multiplayer.pos.y as i32,
                20 as i32, if self.online_multiplayer.focused {HOVERING_BTN_COLOR} else {BTN_COLOR});
            
            return;
        }

        // Input Devices Screen
        else if self.on_devices_screen {
            // Draw texts 
            for text in &self.select_devices_txts {
                draw_handle.draw_text(&text.text, text.pos.x as i32, text.pos.y as i32, 
                    text.size, text.color);
            }

            // Draw buttons 
            for button in &self.select_devices_btns {
                draw_handle.draw_text(&button.text, button.pos.x as i32, button.pos.y as i32,
                    20 as i32, if button.focused {HOVERING_BTN_COLOR} else {BTN_COLOR});
            }

            return;
        }

        // Draw title
        draw_handle.draw_text(&self.title.text, self.title.pos.x as i32, self.title.pos.y as i32, 
                               self.title.size, self.title.color);

        // Draw buttons
        draw_handle.draw_text(&self.singleplayer.text, self.singleplayer.pos.x as i32, self.singleplayer.pos.y as i32,
                               20 as i32, if self.singleplayer.focused {HOVERING_BTN_COLOR} else {BTN_COLOR});

        draw_handle.draw_text(&self.multiplayer.text, self.multiplayer.pos.x as i32, self.multiplayer.pos.y as i32,
                               20 as i32, if self.multiplayer.focused {HOVERING_BTN_COLOR} else {BTN_COLOR});
        
        draw_handle.draw_text(&self.quit.text, self.quit.pos.x as i32, self.quit.pos.y as i32,
                               20 as i32, if self.quit.focused {HOVERING_BTN_COLOR} else {BTN_COLOR});
    
        // Draw score text
        draw_handle.draw_text(&self.hiscore.text, self.hiscore.pos.x as i32, self.hiscore.pos.y as i32, 
                               self.hiscore.size as i32, self.hiscore.color);

        // Draw rects
        // draw_handle.draw_rectangle_rec(&self.singleplayer.rect, Color::GRAY);
        // draw_handle.draw_rectangle_rec(&self.multiplayer.rect, Color::GRAY);
        // draw_handle.draw_rectangle_rec(&self.quit.rect, Color::GRAY);
    }

    fn is_active(&self) -> bool { return self.is_active; }
    fn get_next_scene(&self) -> Box<dyn GameScene> { return Box::new(GameLoop::new(self.selected_mode)); }
}

impl MainMenu {
    fn quit(self: &mut Self) { todo!("Implement this") }

    fn start_game(self: &mut Self, selected_multiplayer: bool) {
        if selected_multiplayer { self.selected_mode = GameMode::Multiplayer }
        self.is_active = false;
    }

    fn change_selected_device(self: &mut Self, player_id: usize, step: i32, rl: &RaylibHandle) {
        let mut avaliable_devices = utils::get_connected_devices(&rl);
        let mut device_id = self.selected_devices[player_id] + step;
        let devices_amount = avaliable_devices.len() as i32;

        if device_id < 0 { device_id = devices_amount - 1 }
        if device_id >= devices_amount { device_id = 0 }
        self.selected_devices[player_id] = device_id;
        
        let mut text = &mut self.select_devices_txts[player_id];
        text.text = avaliable_devices[device_id as usize].get_name();
    }

    pub fn new() -> MainMenu {
        return MainMenu {
            title: Text::new(
                "Pong 2: The Enemy is Now Another", Vector2::new(0.5, 0.1), 
                Color::GOLD, 26
            ),
            
            hiscore: Text::new(
                &format!("HiScore: {}", get_highscore()), Vector2::new(0.5, 0.95),
                Color::WHITE, 16
            ),

            singleplayer: Button::new("Singleplayer", Vector2::new(0.5, 0.4)),
            multiplayer: Button::new("Multiplayer", Vector2::new(0.5, 0.5)),
            quit: Button::new("Quit", Vector2::new(0.5, 0.6)),
            

            on_mltplyr_screen: false,
            local_multiplayer: Button::new("Local Multiplayer", Vector2::new(0.5, 0.4)),
            online_multiplayer: Button::new("Online Multiplayer", Vector2::new(0.5, 0.5)),


            on_devices_screen: false,
            selected_devices: vec![-1, -1],
            select_devices_txts: vec![
                Text::new("Set Player 1 Device", Vector2::new(0.5, 0.4), Color::new(10, 255, 255, 150), 20),
                Text::new("Set Player 2 Device", Vector2::new(0.5, 0.5), Color::new(255, 40, 0, 130), 20)
            ],

            select_devices_btns: vec![
                // Player 1
                Button::new(">", Vector2::new(0.7, 0.4)),
                Button::new("<", Vector2::new(0.3, 0.4)),
                
                // Player 2
                Button::new(">", Vector2::new(0.7, 0.5)),
                Button::new("<", Vector2::new(0.3, 0.5)),
            ],
            
            is_active: true,
            selected_mode: GameMode::Singleplayer
        }                
    }
}
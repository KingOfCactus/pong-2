mod ui_screens;
use regex::Regex;

use super::*;
use crate::utils::*;

const HOVERING_BTN_COLOR: Color = Color::WHITE;
const BTN_COLOR: Color = Color::new(150, 150, 150, 255);

pub enum MenuScreen { TitleScreen, DeviceScreen, ConnectScreen, MultiplayerScreen }

struct TitleScreen {
    title_txt: Text,
    hiscore_txt: Text,

    singleplayer_btn: Button,
    multiplayer_btn: Button,
    quit_btn: Button,

    is_active: bool,
    next_screen: MenuScreen
}

struct DeviceScreen {
    title_txt: Text,
    device_1_txt: Text,
    device_2_txt: Text,

    selected_devices: Vec<i32>,
    device_1_btns: Vec<Button>,
    device_2_btns: Vec<Button>,
    start_btn: Button,

    is_active: bool,
    next_screen: MenuScreen
}

impl GameScene for MainMenu {
    fn update(self: &mut Self, rl: &RaylibHandle) {
        if !self.current_screen.is_active() {
            self.current_screen = self.current_screen.get_next_screen(&rl); 
        }

        self.current_screen.update(&rl);

        // self.remove_ip_field.update(rl, rl.get_mouse_position());
        // self.connect_btns[4].enabled = self.remove_ip_field.is_ipv4();

        // // Return if mouse isn't clicking
        // if !rl.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) {
        //     return;
        // }
        
        // // Check if buttons were clicked
        // let mouse_pos = rl.get_mouse_position();
        // match self.current_screen {
        //     MenuScreen::ConnectScreen => {
        //     },

        //     MenuScreen::MultiplayerScreen => {
        //         if self.local_multiplayer.is_focused(mouse_pos) { self.show_device_screen(false); }
        //         if self.online_multiplayer.is_focused(mouse_pos) { self.show_connect_screen(); }
        //     },
            
        //     MenuScreen::TitleScreen => {
        //         if self.singleplayer.is_focused(mouse_pos) { self.show_device_screen(true); }
        //         if self.multiplayer.is_focused(mouse_pos) { self.show_multiplayer_screen(); }
        //         if self.quit.is_focused(mouse_pos) { self.quit(); }
        //     },

        //     MenuScreen::DeviceScreen => {
        //         // Player 1
        //         if self.select_devices_btns[0].is_focused(mouse_pos) { self.select_input_device(0,  1, rl) } // '>'
        //         if self.select_devices_btns[1].is_focused(mouse_pos) { self.select_input_device(0, -1, rl) } // '<'
                
        //         if self.selected_mode == GameMode::Multiplayer {
        //             // Player 2
        //             if self.select_devices_btns[2].is_focused(mouse_pos) { self.select_input_device(1,  1, rl) } // '>'
        //             if self.select_devices_btns[3].is_focused(mouse_pos) { self.select_input_device(1, -1, rl) } // '<'
        //             if self.select_devices_btns[4].is_focused(mouse_pos) { self.start_game() }
        //         }
        //         else { if self.select_devices_btns[2].is_focused(mouse_pos) { self.start_game() } }
        //     }
        // }
    }

    fn draw(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread) {
        let mouse_pos = rl.get_mouse_position();
        
        let mut elements = self.current_screen.get_elements();
        let mut draw_handle = rl.begin_drawing(thread);
        draw_handle.clear_background(Color::BLACK);
        
        

        // Draw texts 
        for text in &elements.texts {
            draw_handle.draw_text(&text.text, text.pos.x as i32, text.pos.y as i32, 
                text.size, text.color);
        }

        // Draw buttons 
        for button in &elements.buttons {
            draw_handle.draw_text(&button.text, button.pos.x as i32, button.pos.y as i32,
                20 as i32, if button.is_focused(mouse_pos) && button.enabled {HOVERING_BTN_COLOR} else {BTN_COLOR});
        }

        // Draw text fields 
        for field in &elements.fields {
            draw_handle.draw_rectangle_rec(field.rects[0], field.colors[0]);
            draw_handle.draw_rectangle_rec(field.rects[1], field.colors[1]);
            draw_handle.draw_text(&field.text.text, field.text.pos.x as i32, field.text.pos.y as i32, 
                field.text.size, field.text.color);
        }
    }

    fn is_active(&self) -> bool { return self.is_active; }
    fn get_next_scene(&self, rl: &RaylibHandle) -> Box<dyn GameScene> { 
        let devices = (get_device_by_id(self.selected_devices[0]),
                           get_device_by_id(self.selected_devices[1]));

        return Box::new(GameLoop::new(self.selected_mode, devices));
    }
}


impl MainMenu { 
    fn start_game(self: &mut Self) {
        if self.selected_mode == GameMode::Multiplayer { 
            // Make sure that the devices were selected
            if self.selected_devices[0] == -1 || self.selected_devices[1] == -1 {
                return;
            }
        }
        else {
            // Make sure that the devices were selected
            if self.selected_devices[0] == -1 { return; }
            self.selected_devices[1] = self.selected_devices[0];
        }

        self.is_active = false;
    }

    fn select_input_device(self: &mut Self, player_id: usize, step: i32, rl: &RaylibHandle) {
        let mut avaliable_devices = get_connected_devices(&rl);
        let mut device_id = self.selected_devices[player_id] + step;
        let devices_amount = avaliable_devices.len() as i32;

        if device_id < 0 { device_id = devices_amount - 1 }
        if device_id >= devices_amount { device_id = 0 }
        self.selected_devices[player_id] = device_id;
        println!("{}, {}", self.selected_devices[0], self.selected_devices[1]);

        // Make sure the device wasn't selected already
        if self.selected_devices[0] == self.selected_devices[1] {
            let other_player = i32::abs(player_id as i32 - 1) as usize;
            self.select_input_device(other_player, step * -1, rl);
        }
        
        let text = &mut self.select_devices_txts[player_id + 1];
        text.text = avaliable_devices[device_id as usize].get_name();
        text.centralize();
    }

    // fn show_device_screen(self: &mut Self, is_singleplayer: bool) {
    //     let buttons = &mut self.select_devices_btns;
    //     let texts = &mut self.select_devices_txts;

    //     if is_singleplayer {
    //         self.selected_mode = GameMode::Singleplayer;

    //         // Remove player 2 text and buttons
    //         let mut len = buttons.len() as usize - 1;
    //         buttons[len].pos.y = buttons[len-1].pos.y * 1.075;
    //         buttons[len].rect.y = buttons[len].pos.y;

    //         buttons.remove(len - 1);
    //         buttons.remove(len - 2);

    //         len = texts.len() as usize - 1;
    //         texts.remove(len);
    //     }
    //     else { self.selected_mode = GameMode::Multiplayer; }

    //     //self.current_screen = MenuScreen::DeviceScreen;
    // }

    pub fn new() -> MainMenu {
        return MainMenu {
            current_screen: Box::new(TitleScreen::new()),

            local_multiplayer: Button::new(true, "Local Multiplayer", Vector2::new(0.5, 0.4)),
            online_multiplayer: Button::new(true, "Online Multiplayer", Vector2::new(0.5, 0.5)),

            connect_txts: vec![
                Text::new("Select Player and Device:", Vector2::new(0.270, 0.25), Color::WHITE, 20),
                Text::new("Player 1 (Ball)", Vector2::new(0.270, 0.375), Color::new(10, 255, 255, 150), 20),
                Text::new("Device", Vector2::new(0.270, 0.475), Color::GRAY, 20),

                Text::new("Remote Player Address:", Vector2::new(0.7415, 0.25), Color::WHITE, 20),
                // Text::new("xxx.xxx.xxx.xxx", Vector2::new(0.75, 0.375), Color::GRAY, 20),

                // Text::new("Remote client already\nselected player 1\nPing: 120 ms...", Vector2::new(0.3, 0.7), Color::GRAY, 20),
                // Text::new("Started connection,\nwaiting response...\n", Vector2::new(0.3, 0.7), Color::GRAY, 20),
                Text::new("\n(TODO)\n", Vector2::new(0.270, 0.7), Color::GRAY, 20),

                Text::new("\nWaiting for connection...\n", Vector2::new(0.7415, 0.7), Color::GRAY, 20)
                // Text::new("Remote client already\nselected player 1\nPing: 120 ms...", Vector2::new(0.3, 0.7), Color::GRAY, 20),
                // Text::new("Started connection,\nwaiting response...\n", Vector2::new(0.7, 0.7), Color::GRAY, 20),
                // Text::new("Timeout. Did the other\nplayer forgot to press\nthe 'Connect' button?", Vector2::new(0.75, 0.7), Color::GRAY, 20),
            ],
            

            remove_ip_field: TextField::new(Regex::new("[.,0-9]").expect("Invalid regex"), 
                                            "---.---.---.---", 185.0, 20, Vector2::new(0.7415, 0.375), 5.0, 
                                            vec![Color::WHITE, Color::new(30, 30, 30, 255)], 15),

            connect_btns: vec![
                // Player Id
                Button::new(true, ">", Vector2::new(0.470, 0.375)),
                Button::new(true, "<", Vector2::new(0.070, 0.375)),
                
                // Input Device
                Button::new(true, ">", Vector2::new(0.470, 0.475)),
                Button::new(true, "<", Vector2::new(0.070, 0.475)),

                Button::new(false, "Connect", Vector2::new(0.7415, 0.475)),
            ],



            selected_devices: vec![-1, -1],
            select_devices_txts: vec![
                Text::new("Select Players Input:", Vector2::new(0.5, 0.25), Color::WHITE, 20),
                Text::new("Player 1", Vector2::new(0.5, 0.4), Color::new(10, 255, 255, 150), 20),
                Text::new("Player 2", Vector2::new(0.5, 0.5), Color::new(255, 40, 0, 130), 20)
            ],

            select_devices_btns: vec![
                // Player 1
                Button::new(true, ">", Vector2::new(0.7, 0.4)),
                Button::new(true, "<", Vector2::new(0.3, 0.4)),
                
                // Player 2
                Button::new(true, ">", Vector2::new(0.7, 0.5)),
                Button::new(true, "<", Vector2::new(0.3, 0.5)),

                Button::new(true, "Start", Vector2::new(0.5, 0.75)),
            ],
            
            is_active: true,
            selected_mode: GameMode::Singleplayer,
        }            
    }
}
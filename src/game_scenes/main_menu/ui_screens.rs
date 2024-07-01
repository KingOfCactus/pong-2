use std::panic;

use super::*;

impl TitleScreen {
    fn pressed_singleplayer(self: &mut Self) {
        self.selected_mode = GameMode::Singleplayer;
        self.next_screen = MenuScreen::DeviceScreen;
        self.is_active = false;
    }

    fn pressed_multiplayer(self: &mut Self) {
        self.selected_mode = GameMode::Multiplayer;
        self.next_screen = MenuScreen::DeviceScreen;
        self.is_active = false;
    }

    pub fn new() -> TitleScreen {
        TitleScreen {
            title_txt: Text::new(
                "Pong 2: The Enemy is Now Another", Vector2::new(0.5, 0.1), 
                Color::GOLD, 26
            ),
            
            hiscore_txt: Text::new(
                &format!("HiScore: {}", get_highscore()), Vector2::new(0.5, 0.95),
                Color::WHITE, 16
            ),

            singleplayer_btn: Button::new(true, "Singleplayer", Vector2::new(0.5, 0.4)),
            multiplayer_btn: Button::new(true, "Multiplayer", Vector2::new(0.5, 0.5)),
            quit_btn: Button::new(true, "Quit", Vector2::new(0.5, 0.6)),
            
            is_active: true,
            selected_mode: GameMode::None,
            next_screen: MenuScreen::TitleScreen,
        }
    }
}

impl UIScreen for TitleScreen {
    fn update(self: &mut Self, rl: &RaylibHandle) {
             if self.singleplayer_btn.is_pressed(&rl) { self.pressed_singleplayer(); }
        else if self.multiplayer_btn.is_pressed(&rl) { self.pressed_multiplayer() }
        else if self.quit_btn.is_pressed(&rl) { todo!("Implement this"); }
    }

    
    fn get_elements(self: &mut Self, rl: &RaylibHandle) -> ScreenElements {
        ScreenElements::new(rl,
            vec![self.title_txt.clone(), self.hiscore_txt.clone()],
            vec![self.singleplayer_btn.clone(), self.multiplayer_btn.clone(), self.quit_btn.clone()], 
            vec![]
        )
    }

    fn get_next_screen(&self, rl: &RaylibHandle) -> Box<dyn UIScreen> {
        match self.next_screen {
            MenuScreen::DeviceScreen => return Box::new(DeviceScreen::new(self.selected_mode)),
            MenuScreen::MultiplayerScreen => todo!(),
            _ => todo!()
        }
    }

    fn get_next_scene(&self, rl: &RaylibHandle) -> Box<dyn GameScene> {
        panic!("This screen doesn't lead to a scene, should've called 'get_next_screen' instead.");
    }
    fn is_active(&self) -> bool { self.is_active }
    fn goes_to_scene(&self) -> bool { false }
}


impl DeviceScreen {
    fn update_start_btn(self: &mut Self) {
        if self.selected_gamemode == GameMode::Singleplayer {
            self.start_btn.enabled = self.selected_devices[0] >= 0;
        }
        else {
            self.start_btn.enabled = self.selected_devices[0] >= 0 && self.selected_devices[1] >= 0;
        }
    }

    fn change_device(self: &mut Self, rl: &RaylibHandle, player_id: usize, step: i32) {
        let mut connected_devices = get_connected_devices(&rl);
        let devices_amount = connected_devices.len() as i32;

        let mut selected_id = self.selected_devices[player_id] + step;
        if selected_id < 0 { selected_id = devices_amount - 1 }
        if selected_id >= devices_amount { selected_id = 0 }
        self.selected_devices[player_id] = selected_id;

        // Make sure the device wasn't selected 
        if self.selected_devices[0] == self.selected_devices[1] {
            let other_player = i32::abs(player_id as i32 - 1) as usize;
            self.change_device(rl, other_player, step * -1);
        }
        
        let text = if player_id == 0 { &mut self.device_1_txt } else { &mut self.device_2_txt };
        text.text = connected_devices[selected_id as usize].get_name();
        text.centralize();
    }

    pub fn new(mode: GameMode) -> DeviceScreen {
        let mut is_singleplayer = false;
        let mut device_txt_colors = vec![
            Color::new(010, 255, 255, 150), // Player 1
            Color::new(255, 040, 000, 130)  // Player 2
        ];
        
        if mode != GameMode::Multiplayer {
            device_txt_colors[1] = ScreenElements::DISABLED_COLOR;
            is_singleplayer = true;
        }

        return DeviceScreen {
            title_txt: Text::new("Select Players Input:", Vector2::new(0.5, 0.25), Color::WHITE, 20),
            device_1_txt: Text::new("Player 1", Vector2::new(0.5, 0.4), device_txt_colors[0], 20),
            device_2_txt: Text::new("Player 2", Vector2::new(0.5, 0.5), device_txt_colors[1], 20),

            device_1_btns: vec![
                Button::new(true, ">", Vector2::new(0.7, 0.4)),
                Button::new(true, "<", Vector2::new(0.3, 0.4)),
            ],

            device_2_btns: vec![
                Button::new(!is_singleplayer, ">", Vector2::new(0.7, 0.5)),
                Button::new(!is_singleplayer, "<", Vector2::new(0.3, 0.5)),
            ],
            
            selected_devices: vec![-1, -1],
            start_btn: Button::new(false, "Start", Vector2::new(0.5, 0.75)),

            is_active: true,
            selected_gamemode: mode,
        };
    }
}

impl UIScreen for DeviceScreen {
    fn get_next_scene(&self, rl: &RaylibHandle) -> Box<dyn GameScene> {
        let devices = (get_device_by_id(self.selected_devices[0]),
                       get_device_by_id(self.selected_devices[1]));

        return Box::new(GameLoop::new(self.selected_gamemode, devices));
    }

    fn update(self: &mut Self, rl: &RaylibHandle) {
        if self.start_btn.is_pressed(rl) { self.is_active = false; }

             if self.device_1_btns[0].is_pressed(&rl) { self.change_device(rl, 0, -1); }
        else if self.device_1_btns[1].is_pressed(&rl) { self.change_device(rl, 0,  1); }

             if self.device_2_btns[0].is_pressed(&rl) { self.change_device(rl, 1, -1); }
        else if self.device_2_btns[1].is_pressed(&rl) { self.change_device(rl, 1,  1); }

        self.update_start_btn();
    }

    fn get_elements(self: &mut Self, rl: &RaylibHandle) -> ScreenElements {
        let mut buttons: Vec<Button> = vec![self.start_btn.clone()];
        buttons.append(&mut self.device_1_btns.clone());
        buttons.append(&mut self.device_2_btns.clone());

        return ScreenElements::new(rl, 
            vec![self.title_txt.clone(), self.device_1_txt.clone(), self.device_2_txt.clone()], 
            buttons, vec![]
        )
    }
    
    fn get_next_screen(&self, rl: &RaylibHandle) -> Box<dyn UIScreen> {
        panic!("There's no screen after this one, should've called 'get_next_scene' instead.");
    }
    fn is_active(&self) -> bool { self.is_active }
    fn goes_to_scene(&self) -> bool { true }
    
}

use std::panic;

use super::*;

impl TitleScreen {
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
    fn get_next_screen(&self, rl: &RaylibHandle) -> Box<dyn UIScreen> {
        match self.next_screen {
            MenuScreen::DeviceScreen => return Box::new(DeviceScreen::new(self.selected_mode)),
            MenuScreen::MultiplayerScreen => return Box::new(MultiplayerScreen::new()),
            _ => panic!("Invalid next screen, how did you manage to do this?")
        }
    }

    fn update(self: &mut Self, rl: &RaylibHandle) {
        if self.singleplayer_btn.is_pressed(&rl) { 
            self.selected_mode = GameMode::Singleplayer;
            self.next_screen = MenuScreen::DeviceScreen;
            self.is_active = false;
            return; 
        }

        if self.multiplayer_btn.is_pressed(&rl) { 
            self.selected_mode = GameMode::Multiplayer;
            self.next_screen = MenuScreen::MultiplayerScreen;
            self.is_active = false;
            return;
        }

        if self.quit_btn.is_pressed(&rl) { 
            todo!("Implement this");
        }
    }

    
    fn get_elements(self: &mut Self, rl: &RaylibHandle) -> ScreenElements {
        ScreenElements::new(rl,
            vec![self.title_txt.clone(), self.hiscore_txt.clone()],
            vec![self.singleplayer_btn.clone(), self.multiplayer_btn.clone(), self.quit_btn.clone()], 
            vec![]
        )
    }

    fn goes_to_scene(&self) -> bool { false }
    fn is_active(&self) -> bool { self.is_active }
    fn get_next_scene(&self, rl: &RaylibHandle) -> Box<dyn GameScene> {
        panic!("This screen doesn't lead to a scene, should've called 'get_next_screen' instead.");
    }
}

impl MultiplayerScreen {
    pub fn new() -> MultiplayerScreen {
        MultiplayerScreen {
            local_multiplayer: Button::new(true, "Local Multiplayer", Vector2::new(0.5, 0.4)),
            online_multiplayer: Button::new(true, "Online Multiplayer", Vector2::new(0.5, 0.5)),
            next_screen: MenuScreen::DeviceScreen,
            is_active: true
        }
    }
}

impl UIScreen for MultiplayerScreen {
    fn get_next_screen(&self, rl: &RaylibHandle) -> Box<dyn UIScreen> {
        match self.next_screen {
            MenuScreen::DeviceScreen => return Box::new(DeviceScreen::new(GameMode::Multiplayer)),
            MenuScreen::ConnectScreen => return Box::new(ConnectScreen::new()),
            _ => panic!("Invalid next screen, how did you manage to do this?")
        }
    }

    fn update(self: &mut Self, rl: &RaylibHandle) {
        if self.local_multiplayer.is_pressed(rl) {
            self.is_active = false;
            self.next_screen = MenuScreen::DeviceScreen;
        }

        if self.online_multiplayer.is_pressed(rl) {
            self.is_active = false;
            self.next_screen = MenuScreen::ConnectScreen;
        }
    }

    fn get_elements(self: &mut Self, rl: &RaylibHandle) -> ScreenElements {
        return ScreenElements::new(rl, vec![], 
            vec![self.local_multiplayer.clone(), self.online_multiplayer.clone()], vec![]
        );
    }

    fn goes_to_scene(&self) -> bool { false }
    fn is_active(&self) -> bool { self.is_active }
    fn get_next_scene(&self, rl: &RaylibHandle) -> Box<dyn GameScene> {
        panic!("This screen doesn't lead to a scene, should've called 'get_next_screen' instead.");
    }
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

        let mut new_id = self.selected_devices[player_id] + step;
        if new_id < 0 { new_id = devices_amount - 1 }
        if new_id >= devices_amount { new_id = 0 }
        self.selected_devices[player_id] = new_id;

        // Make sure the device wasn't selected 
        if self.selected_devices[0] == self.selected_devices[1] {
            let other_player = i32::abs(player_id as i32 - 1) as usize;
            self.change_device(rl, other_player, step * -1);
        }
        
        let text = if player_id == 0 { &mut self.device_1_txt } else { &mut self.device_2_txt };
        text.text = connected_devices[new_id as usize].get_name();
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
                Button::new(true, "<", Vector2::new(0.3, 0.4)),
                Button::new(true, ">", Vector2::new(0.7, 0.4))
            ],

            device_2_btns: vec![
                Button::new(!is_singleplayer, "<", Vector2::new(0.3, 0.5)),
                Button::new(!is_singleplayer, ">", Vector2::new(0.7, 0.5))
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
    
    fn goes_to_scene(&self) -> bool { true }
    fn is_active(&self) -> bool { self.is_active }
    fn get_next_screen(&self, rl: &RaylibHandle) -> Box<dyn UIScreen> {
        panic!("There's no screen after this one, should've called 'get_next_scene' instead.");
    }
}


impl ConnectScreen {
    fn change_player(self: &mut Self, rl: &RaylibHandle, step: i32) {
        let mut new_id = self.player_id + step;
        if new_id < 0 { new_id = 1 }
        if new_id >= 2 { new_id = 0 }
        self.player_id  = new_id;

        self.player_txt.text = self.player_names[new_id as usize].clone();
        self.player_txt.color = self.player_colors[new_id as usize];
        self.player_txt.centralize();
    }

    fn change_device(self: &mut Self, rl: &RaylibHandle, step: i32) {
        let mut connected_devices = get_connected_devices(&rl);
        let devices_amount = connected_devices.len() as i32;

        let mut new_id = self.device_id + step;
        if new_id < 0 { new_id = devices_amount - 1 }
        if new_id >= devices_amount { new_id = 0 }
        self.device_id  = new_id;
        
        let device_name = connected_devices[new_id as usize].get_name();
        self.device_txt.text = device_name;
        self.device_txt.centralize();
    }

    pub fn new() -> ConnectScreen {
        return ConnectScreen {
            title_txt: Text::new("Select Player and Device:", Vector2::new(0.270, 0.25), Color::WHITE, 20),
            connect_btn: Button::new(false, "Connect", Vector2::new(0.7415, 0.475)),
            
            device_id: -1,
            player_id: -1,

            player_names: vec![
                "Player 1 (Ball)".to_string(),
                "Player 2 (Paddles)".to_string()
            ],
            player_colors: vec![
                Color::new(010, 255, 255, 150), // Player 1
                Color::new(255, 040, 000, 130)  // Player 2
            ],

            player_txt: Text::new("Player", Vector2::new(0.270, 0.375), Color::new(10, 255, 255, 150), 20),
            device_txt: Text::new("Device", Vector2::new(0.270, 0.475), Color::GRAY, 20),
            
            device_btns: vec![ 
                Button::new(true, "<", Vector2::new(0.070, 0.475)),
                Button::new(true, ">", Vector2::new(0.470, 0.475))
            ],
            player_btns: vec![
                Button::new(true, "<", Vector2::new(0.070, 0.375)),
                Button::new(true, ">", Vector2::new(0.470, 0.375))
            ],
            
            remote_ip_txt: Text::new("Remote Player Address:", Vector2::new(0.7415, 0.25), Color::WHITE, 20),

            remote_ip_field: TextField::new(Regex::new("[.,0-9]").expect("Invalid regex"), 
                                            "---.---.---.---", 185.0, 20, Vector2::new(0.7415, 0.375), 5.0, 
                                            vec![Color::WHITE, Color::new(30, 30, 30, 255)], 15),
            
            is_active: true,
            remote_info_txt: Text::new("\n(TODO)\n", Vector2::new(0.270, 0.7), Color::GRAY, 20),
            connection_status_txt: Text::new("\nWaiting for connection...\n", Vector2::new(0.7415, 0.7), Color::GRAY, 20),
            // Text::new("Remote client already\nselected player 1\nPing: 120 ms...", Vector2::new(0.3, 0.7), Color::GRAY, 20),
            // Text::new("Started connection,\nwaiting response...\n", Vector2::new(0.7, 0.7), Color::GRAY, 20),
            // Text::new("Timeout. Did the other\nplayer forgot to press\nthe 'Connect' button?", Vector2::new(0.75, 0.7), Color::GRAY, 20),
        }
    }
}

impl UIScreen for ConnectScreen {
    fn get_next_screen(&self, rl: &RaylibHandle) -> Box<dyn UIScreen> {
        todo!()
    }
    
    fn update(self: &mut Self, rl: &RaylibHandle) {
             if self.device_btns[0].is_pressed(rl) { self.change_device(rl, -1) }
        else if self.device_btns[1].is_pressed(rl) { self.change_device(rl,  1) }

        else if self.player_btns[0].is_pressed(rl) { self.change_player(rl, -1) }
        else if self.player_btns[1].is_pressed(rl) { self.change_player(rl,  1) }

        self.remote_ip_field.update(rl);
        self.connect_btn.enabled =  self.remote_ip_field.is_ipv4() && 
                                    self.device_id + self.player_id >= 0;
    }

    fn get_elements(self: &mut Self, rl: &RaylibHandle) -> ScreenElements {
        let mut buttons: Vec<Button> = vec![self.connect_btn.clone()];
        buttons.append(&mut self.device_btns.clone());
        buttons.append(&mut self.player_btns.clone());

        return ScreenElements::new(rl, 
            vec![self.title_txt.clone(), self.player_txt.clone(), self.device_txt.clone(), 
                 self.remote_ip_txt.clone(), self.remote_info_txt.clone(), self.connection_status_txt.clone()], 
            buttons, vec![self.remote_ip_field.clone()]
        )
    }


    fn goes_to_scene(&self) -> bool { false }
    fn is_active(&self) -> bool { self.is_active }
    fn get_next_scene(&self, rl: &RaylibHandle) -> Box<dyn GameScene> {
        panic!("This screen doesn't lead to a scene, should've called 'get_next_screen' instead.");
    }
}
use super::*;
 
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
        let mut connected_devices = InputUtils::get_connected_devices(&rl);
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
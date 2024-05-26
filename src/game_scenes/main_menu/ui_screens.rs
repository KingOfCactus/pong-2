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
            next_screen: MenuScreen::TitleScreen,
        }
    }
}

impl UIScreen for TitleScreen {
    fn update(self: &mut Self, rl: &RaylibHandle) {
        if self.singleplayer_btn.is_pressed(&rl) { self.next_screen = MenuScreen::DeviceScreen; }
        if self.multiplayer_btn.is_focused(&rl) { self.next_screen = MenuScreen::MultiplayerScreen }
        if self.quit_btn.is_focused(&rl) { todo!("Implement this"); }
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
            MenuScreen::DeviceScreen => return Box::new(DeviceScreen::new()),
            MenuScreen::MultiplayerScreen => todo!(),
            _ => todo!()
        }
    }

    fn is_active(&self) -> bool { self.is_active }
}


impl DeviceScreen {
    pub fn new() -> DeviceScreen {
        DeviceScreen {
            title_txt: Text::new("Select Players Input:", Vector2::new(0.5, 0.25), Color::WHITE, 20),
            device_1_txt: Text::new("Player 1", Vector2::new(0.5, 0.4), Color::new(010, 255, 255, 150), 20),
            device_2_txt: Text::new("Player 2", Vector2::new(0.5, 0.5), Color::new(255, 040, 000, 130), 20),

            device_1_btns: vec![
                Button::new(true, ">", Vector2::new(0.7, 0.4)),
                Button::new(true, "<", Vector2::new(0.3, 0.4)),
            ],

            device_2_btns: vec![
                Button::new(true, ">", Vector2::new(0.7, 0.5)),
                Button::new(true, "<", Vector2::new(0.3, 0.5)),
            ],
            
            selected_devices: vec![-1, -1],
            start_btn: Button::new(true, "Start", Vector2::new(0.5, 0.75)),

            is_active: true,
            next_screen: MenuScreen::DeviceScreen,
        }
    }
}

impl UIScreen for DeviceScreen {
    fn get_next_screen(&self, rl: &RaylibHandle) -> Box<dyn UIScreen> {
        todo!()
    }

    fn update(self: &mut Self, rl: &RaylibHandle) {
        todo!()
    }

    fn get_elements(self: &mut Self, rl: &RaylibHandle) -> ScreenElements {
        todo!()
    }

    fn is_active(&self) -> bool { self.is_active }
}

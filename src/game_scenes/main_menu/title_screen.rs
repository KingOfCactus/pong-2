use super::*;

impl TitleScreen {
    pub fn new() -> TitleScreen {
        TitleScreen {
            title_txt: Text::new(
                "Pong 2: The Enemy is Now Another", Vector2::new(0.5, 0.1), 
                Color::GOLD, 26
            ),
            
            hiscore_txt: Text::new(
                &format!("HiScore: {}", MiscUtils::get_highscore()), Vector2::new(0.5, 0.95),
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
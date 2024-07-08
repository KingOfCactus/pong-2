use super::*;

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
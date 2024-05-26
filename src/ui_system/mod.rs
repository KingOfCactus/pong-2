mod elements;

use raylib::prelude::*;
use regex::Regex;

#[derive(Clone)]
pub struct ScreenElements {
    pub texts: Vec<Text>,
    pub fields: Vec<TextField>,
    pub buttons: Vec<(Button, Color)>,
}

impl ScreenElements {
    pub fn new(rl: &RaylibHandle, texts: Vec<Text>, buttons: Vec<Button>, fields: Vec<TextField>) -> ScreenElements {
        let mut _buttons = Vec::with_capacity(buttons.len());

        for i in 0..buttons.len() {
            _buttons[i] = (buttons[i].clone(), buttons[i].get_color(rl));
        }

        return ScreenElements{
            texts: texts, 
            fields: fields, 
            buttons: _buttons
        };
    }
}

pub trait UIScreen {
    fn is_active(&self) -> bool;
    fn get_next_screen(&self, rl: &RaylibHandle) -> Box<dyn UIScreen>;

    fn update(self: &mut Self, rl: &RaylibHandle);
    fn get_elements(self: &mut Self, rl: &RaylibHandle) -> ScreenElements;
}


#[derive(Clone)]
pub struct Text {
    pub relative_pos: Vector2,
    pub color: Color,
    pub text: String,
    pub pos: Vector2,
    pub size: i32
}

#[derive(Clone)]
pub struct Button {
    pub rect: Rectangle,
    pub enabled: bool,
    pub text: String,
    pub pos: Vector2,
}

#[derive(Clone)]
pub struct TextField {
    pub rects: Vec<Rectangle>,
    pub pos: Vector2,

    pub colors: Vec<Color>,
    pub max_length: usize,
    pub format: Regex,

    pub text: Text,
    pub value: String,
    pub placeholder: String,
}
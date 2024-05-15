mod elements;

use raylib::prelude::*;
use regex::Regex;

#[derive(Clone)]
pub struct ScreenElements {
    pub texts: Vec<Text>,
    pub buttons: Vec<Button>,
    pub fields: Vec<TextField>,
}

pub trait UIScreen {
    fn is_active(&self) -> bool;
    fn get_next_screen(&self, rl: &RaylibHandle) -> Box<dyn UIScreen>;

    fn update(self: &mut Self, rl: &RaylibHandle);
    fn get_elements(self: &mut Self) -> ScreenElements;
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
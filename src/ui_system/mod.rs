mod elements;

use raylib::prelude::*;
use regex::Regex;

#[derive(Clone)]
pub struct ScreenElements {
    pub texts: Text,
    pub rects: Rectangle
}

pub trait UIScreen {
    fn update(self: &mut Self, rl: &RaylibHandle);
    fn get_elements() -> ScreenElements;
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
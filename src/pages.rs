use iced::{widget::{text, Text}, Theme};


pub mod display;
pub struct Display;


pub fn title(title_text: &str) -> Text<'_> {
    text(title_text).size(32)
}
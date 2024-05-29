use iced::Sandbox;
use iced::widget::{component, row};
use crate::sidebar::Sidebar;
use crate::sidebar::Page;
use crate::pages;


#[derive(Debug, Clone, Copy)]
pub enum Message {
    SwitchPage(Page)
}

pub struct MainWindow {
    current_page: Page
}

impl Sandbox for MainWindow {
    type Message = Message;

    fn new() -> Self {
        Self {current_page: Page::Display}
    }

    fn title(&self) -> String {
        String::from("Settings")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::SwitchPage(page) => {
                self.current_page = page;
                println!("Switched to {:?} page", page);
            }
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        row![
            component(Sidebar::new()),
            component(pages::Display)
        ].into()

    }

    fn theme(&self) -> iced::Theme {
        iced::Theme::CatppuccinFrappe
    }
}
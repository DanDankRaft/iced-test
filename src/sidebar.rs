use iced::widget::{button, column, row, horizontal_space, svg, text, Component};
use iced::{Color, Element, Length, Pixels, Theme};
use std::fmt::Display;
use std::path::Path;
use crate::window::Message;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub enum Page {
    #[default] Display,
    Audio
}

impl Display for Page {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Page::Display => write!(f, "Display"),
            Page::Audio => write!(f, "Audio")
        }
    }
}

impl Page {
    fn array_of() -> [Page; 2] {
        [Page::Display, Page::Audio]
    }
}

#[derive(Debug, Clone)]
pub enum SidebarEvent {
    SwitchPage(Page)
}


pub struct Sidebar;

#[derive(Default)]
pub struct SidebarState {
    current_page: Page,
}

impl Sidebar {
    pub fn new() -> Self {
        Self {}
    }
}

impl Component<Message> for Sidebar {
    type State = SidebarState;

    type Event = SidebarEvent;

    fn update(
        &mut self,
        state: &mut Self::State,
        event: Self::Event,
    ) -> Option<Message> {
        match event {
            SidebarEvent::SwitchPage(page) => {
                state.current_page = page;
                Some(Message::SwitchPage(page))
            }
        }
    }

    fn view(
        &self,
        state: &Self::State,
    ) -> Element<'_, Self::Event> {

        // Keyed column implementation
        let buttons = Page::array_of()
        .into_iter()
        .map(|page| -> Element<_> {
            let mut button = button(
                row![
                    svg(svg::Handle::from_path(Path::new("./src/icons/display.svg")))
                        .width(32)
                        .height(32)
                        .style(iced::theme::Svg::custom_fn(white_svg)),
                    horizontal_space()
                        .width(10),
                    text(page)
                        .size(32)
                        .vertical_alignment(iced::alignment::Vertical::Center)
                ].align_items(iced::Alignment::Center)
            );
            if state.current_page != page {
                button = button.on_press(SidebarEvent::SwitchPage(page))
            }

            button
                .width(Length::Fixed(175.0))
                .style(iced::theme::Button::custom(ButtonStyle))
                .into()
        });

        column(buttons).spacing(1).into()
    }
}

struct ButtonStyle;

impl iced::widget::button::StyleSheet for ButtonStyle {

    // Will this work? Only time will tell...
    type Style = Theme;

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(iced::Background::Color(Color::from_rgba(1.0, 1.0, 1.0, 0.0))),
            text_color: Color::WHITE,
            ..Default::default()
        }
    }

    fn disabled(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(iced::Background::Color(Color::from_rgba(1.0, 1.0, 1.0, 0.2))),
            ..self.active(style)
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(iced::Background::Color(Color::from_rgba(1.0, 1.0, 1.0, 0.1))),
            ..self.active(style)
        }
    }

    fn pressed(&self, style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(iced::Background::Color(Color::from_rgba(1.0, 1.0, 1.0, 0.25))),
            ..self.active(style)
        }
    }
}

fn white_svg(_theme: &Theme) -> svg::Appearance {
    svg::Appearance {
        color: Some(Color::WHITE)
    }
}
use iced::widget::{column, text, Component};
use iced::Element;
use crate::window::Message;

pub struct DisplayEvent {
    // ...
}

impl Component<Message> for crate::pages::Display {
    type State = ();

    type Event = DisplayEvent;

    fn update(
        &mut self,
        state: &mut Self::State,
        event: Self::Event,
    ) -> Option<Message> {
        // TODO ...
        None
    }

    fn view(
        &self,
        state: &Self::State,
    ) -> Element<'_, Self::Event> {
        crate::pages::title("Display Settings").into()
        // text("Display Settings").into()
    }
}
use crate::message::Message;
use iced::{settings::Window, Application, Column, Command, Element, Settings, Text};

#[derive(Default)]
pub(crate) struct AppWindow {}

impl Application for AppWindow {
    type Message = Message;

    fn new() -> (Self, Command<Message>) {
        (Self::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("Applesauce!")
    }

    fn update(&mut self, _message: Self::Message) -> Command<Message> {
        Command::none()
    }

    fn view(&mut self) -> Element<'_, Message> {
        Column::new()
            .push(Text::new("Welcome to the Applesauce Mandelbrot app!").size(50))
            .into()
    }
}

pub const fn settings() -> Settings {
    Settings {
        window: Window {
            size: (800, 600),
            resizable: true,
        },
    }
}

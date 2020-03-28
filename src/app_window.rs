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

#[allow(clippy::integer_arithmetic)] // TODO: Remove when window size determined dynamically
pub const fn settings() -> Settings {
    // TODO: Determine screen resolution from `iced`, Rust or OS
    let width: u32 = 3440 / 2;
    let height: u32 = 1440 / 2;

    Settings {
        window: Window {
            size: (width, height),
            resizable: true,
        },
    }
}

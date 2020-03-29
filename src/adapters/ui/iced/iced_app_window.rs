use crate::{message::Message, ports::ui::RectSize, Error, Result};
use iced::{settings::Window, Application, Column, Command, Element, Settings, Text};
use std::{cmp::max, convert::TryFrom};

#[derive(Default)]
pub struct IcedAppWindow {}

impl Application for IcedAppWindow {
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

pub fn settings(rect_size: &RectSize) -> Result<Settings> {
    #[allow(clippy::cast_possible_truncation)]
    match u32::try_from(max(rect_size.0, rect_size.1)).is_ok() {
        true => Ok(Settings {
            window: Window {
                size: (rect_size.0 as u32, rect_size.1 as u32),
                resizable: true,
            },
        }),
        false => Err(Error::AdapterUiIcedScreenDimsTooLarge(rect_size.clone())),
    }
}

use crate::{
    adapters::ui::{iced::MandelMessage, MandelApp},
    ports::ui::{Color, Rect},
};

#[derive(Clone, Debug)]
pub struct MandelCanvas {
    message: <MandelApp as iced::Application>::Message,
    view_rect: Rect<f64>,
    bg_color: Color<f32>,
}

impl MandelCanvas {
    pub fn new(view_rect: Rect<f64>, bg_color: Color<f32>) -> Self {
        Self {
            message: MandelMessage::ClearCanvas(bg_color),
            view_rect,
            bg_color,
        }
    }

    pub const fn bg_color(&self) -> Color<f32> {
        self.bg_color
    }

    pub fn set_bg_color(&mut self, bg_color: Color<f32>) -> &mut Self {
        self.bg_color = bg_color;
        self
    }

    pub const fn message(&self) -> &<MandelApp as iced::Application>::Message {
        &self.message
    }

    pub fn set_message(&mut self, message: <MandelApp as iced::Application>::Message) -> &mut Self {
        self.message = message;
        self
    }
}

impl iced::canvas::Drawable for MandelCanvas {
    fn draw(&self, frame: &mut iced::canvas::Frame) {
        match &self.message {
            MandelMessage::ClearCanvas(color) => {
                let mandel_space = iced::canvas::Path::new(|path| {
                    path.rectangle(iced::Point::new(0.0, 0.0), frame.size())
                });
                frame.fill(&mandel_space, iced::canvas::Fill::Color(color.into()));
            }
        }
    }
}

impl From<Color<f32>> for iced::Color {
    fn from(color: Color<f32>) -> Self {
        Self {
            r: color.0,
            g: color.1,
            b: color.2,
            a: color.3,
        }
    }
}

impl From<&Color<f32>> for iced::Color {
    fn from(color: &Color<f32>) -> Self {
        (*color).into()
    }
}

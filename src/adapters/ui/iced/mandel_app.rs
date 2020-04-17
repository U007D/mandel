use crate::ports::ui::Color;
use crate::{
    adapters::ui::iced::{MandelCanvas, MandelMessage, MandelUserSettings},
    ports::ui::{Point, Rect},
};
use iced;

#[derive(Debug)]
pub struct MandelApp {
    mandelbrot: iced::canvas::layer::Cache<MandelCanvas>,
    canvas: MandelCanvas,
    user_settings: MandelUserSettings,
}

impl iced::Application for MandelApp {
    type Executor = iced::executor::Default;
    type Message = MandelMessage;
    type Flags = MandelUserSettings;

    fn new(user_settings: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        dbg!("MandelApp::new() called");
        (
            Self {
                mandelbrot: iced::canvas::layer::Cache::<MandelCanvas>::default(),
                canvas: MandelCanvas::new(
                    Rect(Point(-2.0, 1.5), Point(1.0, -1.5)),
                    user_settings.canvas_color,
                ),
                user_settings,
            },
            iced::Command::none(),
        )
    }

    fn title(&self) -> String {
        self.user_settings.title.clone()
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        self.canvas.set_message(message);
        let bg_color = self.canvas.bg_color();
        self.canvas.set_bg_color(Color(
            1.0 - bg_color.0,
            1.0 - bg_color.1,
            1.0 - bg_color.2,
            bg_color.3,
        ));
        dbg!("MandelApp::update() called");
        iced::Command::none()
    }

    #[allow(clippy::integer_arithmetic)]
    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        let canvas = iced::Canvas::new()
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .push(self.mandelbrot.with(&self.canvas));
        dbg!("MandelApp::view() called");
        iced::Container::new(canvas)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .into()
    }
}

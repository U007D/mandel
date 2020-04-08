use crate::adapters::ui::iced::app_settings::AppSettings;
use iced;

#[derive(Debug)]
pub struct App {
    app_settings: AppSettings,
}

impl iced::Application for App {
    type Executor = iced::executor::Default;
    type Message = ();
    type Flags = AppSettings;

    fn new(app_settings: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (Self { app_settings }, iced::Command::none())
    }

    fn title(&self) -> String {
        self.app_settings.title.clone()
    }

    fn update(&mut self, _message: Self::Message) -> iced::Command<Self::Message> {
        iced::Command::none()
    }

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        iced::Column::new()
            .push(iced::Text::new("Welcome to my Mandelbrot app!").size(50))
            .into()
    }
}

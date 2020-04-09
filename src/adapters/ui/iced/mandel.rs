use crate::adapters::ui::iced::mandel_settings::MandelSettings;
use iced;

#[derive(Debug)]
pub struct Mandel {
    app_settings: MandelSettings,
}

impl iced::Application for Mandel {
    type Executor = iced::executor::Default;
    type Message = ();
    type Flags = MandelSettings;

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

use crate::consts::msg;
use crate::{
    adapters::ui::{iced::mandel_user_settings::MandelUserSettings, MandelCanvas},
    ports::ui::Canvas,
};
use iced;

#[derive(Debug)]
pub struct Mandel<'a> {
    user_settings: MandelUserSettings,
    canvas: MandelCanvas<'a>,
}

impl iced::Application for Mandel<'_> {
    type Executor = iced::executor::Default;
    type Message = ();
    type Flags = MandelUserSettings;

    fn new(user_settings: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            Self {
                canvas: MandelCanvas::new(user_settings.window_settings.size())
                    .expect(msg::ERR_ADAPTER_UI_WINDOW_TOO_LARGE),
                user_settings,
            },
            iced::Command::none(),
        )
    }

    fn title(&self) -> String {
        self.user_settings.title.clone()
    }

    fn update(&mut self, _message: Self::Message) -> iced::Command<Self::Message> {
        iced::Command::none()
    }

    #[allow(clippy::integer_arithmetic)]
    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        self.user_settings.i += 1;
        iced::Column::new()
            .push(
                iced::Text::new(format!(
                    "Welcome to my Mandelbrot app! ({})",
                    self.user_settings.i
                ))
                .size(50),
            )
            .into()
    }
}

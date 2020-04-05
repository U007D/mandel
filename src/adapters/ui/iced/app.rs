use super::AdditionalSettings;
use crate::{
    ports::ui::{AppBuilderTrait, AppTrait},
    Result,
};
use iced;

pub struct App {
    pub(super) iced_settings: iced::Settings<AdditionalSettings>,
}

impl AppTrait for App {
    #[allow(clippy::new_ret_no_self)]
    fn new<AB: AppBuilderTrait + Default>() -> AB {
        AB::default()
    }

    fn run(self) -> Result<()> {
        <Self as iced::Application>::run(self.iced_settings);
        Ok(())
    }
}

impl iced::Application for App {
    type Executor = iced::executor::Default;
    type Message = ();
    type Flags = AdditionalSettings;

    fn new(
        additional_settings: <Self as iced::Application>::Flags,
    ) -> (Self, iced::Command<Self::Message>) {
        (
            Self {
                iced_settings: iced::Settings {
                    window: iced::window::Settings::default(),
                    flags: additional_settings,
                    default_font: None,
                    antialiasing: false,
                },
            },
            iced::Command::none(),
        )
    }

    fn title(&self) -> String {
        self.iced_settings.flags.title.clone()
    }

    fn update(&mut self, _message: Self::Message) -> iced::Command<Self::Message> {
        iced::Command::none()
    }

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        iced::Column::new()
            .push(iced::Text::new("Welcome to the \"Applesauce\" Mandelbrot app_trait!").size(50))
            .into()
    }
}

use crate::{
    adapters::ui::{iced::app_bootstrapper::AppBootstrapper, iced::app_settings::AppSettings},
    consts::msg,
    ports::ui::{AppBuilderTrait, Pair, WindowState},
    Error, Result,
};
use iced;
use std::convert::{TryFrom, TryInto};

#[derive(Debug, Default)]
pub struct AppBuilder {
    title: Option<String>,
    window_state: Option<WindowState>,
    iced_settings: Option<iced::Settings<AppSettings>>,
}

impl AppBuilder {
    fn window_dimensions(&self) -> Pair<usize> {
        self.window_state.as_ref().map_or_else(
            || WindowState::default().dimensions(),
            WindowState::dimensions,
        )
    }

    fn is_window_resizable(&self) -> bool {
        self.window_state.as_ref().map_or_else(
            || WindowState::default().is_resizable(),
            WindowState::is_resizable,
        )
    }
}

impl AppBuilderTrait for AppBuilder {
    type App = AppBootstrapper;

    fn new() -> Self {
        Self::default()
    }

    fn build(self) -> Result<Self::App, Error> {
        Ok(Self::App {
            app_settings: self.try_into()?,
        })
    }

    fn set_title(mut self, title: impl ToString) -> Self {
        self.title.replace(title.to_string());
        self
    }

    fn set_window_state(mut self, window_state: WindowState) -> Self {
        self.window_state.replace(window_state);
        self
    }
}

impl TryFrom<AppBuilder> for iced::Settings<AppSettings> {
    type Error = Error;

    fn try_from(ab: AppBuilder) -> Result<Self, Self::Error> {
        Ok(Self {
            flags: AppSettings {
                title: ab
                    .title
                    .clone()
                    .unwrap_or_else(|| String::from(msg::UNTITLED)),
                window_state: ab.window_state.clone().unwrap_or_else(WindowState::default),
            },
            window: ab.try_into()?,
            default_font: None,
            antialiasing: false,
        })
    }
}

impl TryFrom<AppBuilder> for iced::window::Settings {
    type Error = Error;

    fn try_from(ab: AppBuilder) -> Result<Self, Self::Error> {
        Ok(Self {
            size: ab
                .window_dimensions()
                .try_into()
                .map_err(|e| Error::AdapterUiIcedScreenDimsTooLarge(ab.window_dimensions(), e))?,
            resizable: ab.is_window_resizable(),
            decorations: true,
        })
    }
}

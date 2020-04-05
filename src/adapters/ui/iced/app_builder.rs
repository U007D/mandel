use crate::consts::msg;
use crate::ports::ui::Pair;
use crate::{
    ports::ui::{AppBuilderTrait, WindowState},
    App, Error, Result,
};
use iced;
use std::convert::{TryFrom, TryInto};

#[derive(Debug, Default)]
pub struct AppBuilder {
    title: Option<String>,
    window_state: Option<WindowState>,
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
    type App = App;

    fn build(self) -> Result<Self::App, Error> {
        Ok(Self::App {
            iced_settings: self.try_into()?,
        })
    }

    fn set_title(mut self, title: impl ToString) -> Self {
        self.title = Some(title.to_string());
        self
    }

    fn set_window_state(mut self, window_state: WindowState) -> Self {
        self.window_state = Some(window_state);
        self
    }
}

impl TryFrom<AppBuilder> for iced::Settings<AppSettings> {
    type Error = Error;

    fn try_from(mut ab: AppBuilder) -> Result<Self, Self::Error> {
        Ok(Self {
            flags: AppSettings::from(ab.title.take()),
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

#[derive(Debug, Clone)]
pub struct AppSettings {
    pub(super) title: String,
}

impl From<Option<String>> for AppSettings {
    fn from(opt_string: Option<String>) -> Self {
        Self {
            title: opt_string.unwrap_or_else(|| String::from(msg::UNTITLED)),
        }
    }
}

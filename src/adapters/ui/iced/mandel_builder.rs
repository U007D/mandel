use crate::{
    adapters::ui::iced::{MandelBootstrapper, MandelUserSettings},
    consts::msg,
    ports::ui::{AppBuilder, Size, WindowSettings},
    Error, Result,
};
use iced;
use std::convert::{TryFrom, TryInto};

#[derive(Debug, Default)]
pub struct MandelBuilder {
    title: Option<String>,
    window_state: Option<WindowSettings>,
    iced_settings: Option<iced::Settings<MandelUserSettings>>,
}

impl MandelBuilder {
    fn window_size(&self) -> Size<usize> {
        self.window_state
            .as_ref()
            .map_or_else(|| WindowSettings::default().size(), WindowSettings::size)
    }

    fn is_window_resizable(&self) -> bool {
        self.window_state.as_ref().map_or_else(
            || WindowSettings::default().is_resizable(),
            WindowSettings::is_resizable,
        )
    }
}

impl AppBuilder for MandelBuilder {
    type App = MandelBootstrapper;

    fn new() -> Self {
        Self::default()
    }

    fn build(self) -> Result<Self::App, Error> {
        Ok(Self::App {
            iced_settings: self.try_into()?,
        })
    }

    fn set_title(mut self, title: impl ToString) -> Self {
        self.title.replace(title.to_string());
        self
    }

    fn set_window_state(mut self, window_state: WindowSettings) -> Self {
        self.window_state.replace(window_state);
        self
    }
}

impl TryFrom<MandelBuilder> for iced::Settings<MandelUserSettings> {
    type Error = Error;

    fn try_from(ab: MandelBuilder) -> Result<Self, Self::Error> {
        Ok(Self {
            flags: MandelUserSettings {
                title: ab
                    .title
                    .clone()
                    .unwrap_or_else(|| String::from(msg::UNTITLED)),
                window_settings: ab
                    .window_state
                    .clone()
                    .unwrap_or_else(WindowSettings::default),
                i: 0,
            },
            window: ab.try_into()?,
            default_font: None,
            antialiasing: false,
        })
    }
}

impl TryFrom<MandelBuilder> for iced::window::Settings {
    type Error = Error;

    fn try_from(ab: MandelBuilder) -> Result<Self, Self::Error> {
        Ok(Self {
            size: ab
                .window_size()
                .try_into()
                .map_err(|e| Error::AdapterUiIcedScreenDimsTooLarge(ab.window_size(), e))?,
            resizable: ab.is_window_resizable(),
            decorations: true,
        })
    }
}

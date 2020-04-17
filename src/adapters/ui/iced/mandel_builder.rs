use crate::{
    adapters::ui::iced::{MandelBootstrapper, MandelUserSettings},
    consts::msg,
    ports::ui::{AppBuilder, Color, Size, WindowSettings},
    Error, Result,
};
use iced;
use std::convert::{TryFrom, TryInto};

#[derive(Debug, Default)]
pub struct MandelBuilder {
    title: Option<String>,
    window_state: Option<WindowSettings>,
    iced_settings: Option<iced::Settings<MandelUserSettings>>,
    canvas_color: Option<Color<f32>>,
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
    type ColorValueType = f32;

    fn new() -> Self {
        Self::default()
    }

    fn build(self) -> Result<Self::App, Error> {
        Ok(Self::App {
            iced_settings: self.try_into()?,
        })
    }

    fn set_title<S: ToString>(mut self, title: S) -> Self {
        self.title.replace(title.to_string());
        self
    }

    fn set_window_settings(mut self, window_state: WindowSettings) -> Self {
        self.window_state.replace(window_state);
        self
    }

    fn set_canvas_color(mut self, canvas_color: Color<Self::ColorValueType>) -> Self {
        self.canvas_color = Some(canvas_color);
        self
    }
}

impl TryFrom<MandelBuilder> for iced::Settings<MandelUserSettings> {
    type Error = Error;

    fn try_from(mb: MandelBuilder) -> Result<Self, Self::Error> {
        Ok(Self {
            flags: MandelUserSettings {
                title: mb
                    .title
                    .clone()
                    .unwrap_or_else(|| String::from(msg::UNTITLED)),
                window_settings: mb
                    .window_state
                    .clone()
                    .unwrap_or_else(WindowSettings::default),
                i: 0,
                canvas_color: mb.canvas_color.unwrap_or_else(Color::default),
            },
            window: mb.try_into()?,
            default_font: None,
            antialiasing: false,
        })
    }
}

impl TryFrom<MandelBuilder> for iced::window::Settings {
    type Error = Error;

    fn try_from(mb: MandelBuilder) -> Result<Self, Self::Error> {
        Ok(Self {
            size: mb
                .window_size()
                .try_into()
                .map_err(|e| Error::AdapterUiIcedScreenDimsTooLarge(mb.window_size(), e))?,
            resizable: mb.is_window_resizable(),
            decorations: true,
        })
    }
}

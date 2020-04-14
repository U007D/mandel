use crate::{
    adapters::ui::MandelApp,
    consts::msg,
    error::Error,
    ports::ui::{AppBuilder, WindowSettings},
};

#[derive(Debug, Default)]
pub struct MandelBuilder {
    title: Option<String>,
    window_settings: Option<WindowSettings>,
}

impl AppBuilder for MandelBuilder {
    type App = MandelApp;
    type Error = Error;

    fn new() -> Self {
        Self::default()
    }

    fn build(self) -> Result<Self::App, Self::Error> {
        Ok(Self::App {
            title: self.title.unwrap_or_else(|| String::from(msg::UNTITLED)),
            window_settings: self.window_settings.unwrap_or_else(WindowSettings::default),
        })
    }

    fn set_title<S: ToString>(mut self, title: S) -> Self {
        self.title = Some(title.to_string());
        self
    }

    fn set_window_settings(mut self, window_settings: WindowSettings) -> Self {
        self.window_settings = Some(window_settings);
        self
    }
}

use crate::{ports::ui::WindowSettings, App, Result};

pub trait AppBuilder {
    type App: App;
    type Error: std::error::Error;

    fn new() -> Self;
    fn build(self) -> Result<Self::App, Self::Error>;
    fn set_title<S: ToString>(self, title: S) -> Self;
    fn set_window_settings(self, window_settings: WindowSettings) -> Self;
}

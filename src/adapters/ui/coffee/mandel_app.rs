use crate::{
    adapters::ui::MandelCanvas,
    ports::ui::{App, WindowSettings},
    Error,
};
use ::coffee::{graphics::WindowSettings as CoffeeWindowSettings, Game};
use std::convert::TryInto;

#[derive(Debug)]
pub struct MandelApp {
    pub(super) title: String,
    pub(super) window_settings: WindowSettings,
}

impl App for MandelApp {
    type Error = Error;

    fn run(self) -> Result<(), Self::Error> {
        <MandelCanvas as Game>::run(CoffeeWindowSettings {
            title: self.title.clone(),
            size: self.window_settings.size().try_into()?,
            resizable: self.window_settings.is_resizable(),
            fullscreen: self.window_settings.is_full_screen(),
            maximized: self.window_settings.is_maximized(),
        })?;
        Ok(())
    }
}

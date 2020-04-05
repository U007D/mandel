use crate::{
    adapters::ui::{iced::app_settings::AppSettings, App},
    AppBuilder, AppTrait, Error, Result,
};
use std::convert::{TryFrom, TryInto};

#[derive(Debug)]
pub struct AppBootstrapper {
    pub app_settings: iced::Settings<AppSettings>,
}

impl AppTrait for AppBootstrapper {
    fn run(self) -> Result<()> {
        <App as iced::Application>::run(self.app_settings);
        Err(Error::AppReturnedUnexpectedly)
    }
}

impl TryFrom<AppBuilder> for AppBootstrapper {
    type Error = Error;

    fn try_from(ab: AppBuilder) -> Result<Self, Self::Error> {
        Ok(Self {
            app_settings: ab.try_into()?,
        })
    }
}

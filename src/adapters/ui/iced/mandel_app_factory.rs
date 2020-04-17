use crate::{
    adapters::ui::{iced::MandelUserSettings, MandelApp},
    error::Error,
    ports::ui::App,
};

/// Why does `MandelAppFactory` exist?  See `MandelBuilder`'s declaration for the explanation.
#[derive(Debug)]
pub struct MandelAppFactory {
    pub iced_settings: iced::Settings<MandelUserSettings>,
}

impl App for MandelAppFactory {
    fn run(self) -> Result<(), Error> {
        <MandelApp as iced::Application>::run(self.iced_settings);
        Err(Error::AppReturnedUnexpectedly)
    }
}

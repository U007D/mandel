use crate::{
    adapters::ui::{iced::MandelUserSettings, MandelApp},
    error::Error,
    ports::ui::App,
};

/// Why does `AppBootstrapper` need to exist?
/// `iced` is a framework (as opposed to a library).  Further, `iced::Application` uses a singleton
/// pattern where the constructor is actually `Application::run()`, not `Application::new()`.
///
/// `::run()` instantiates an `iced::Application` and the framework calls `::new()` internally.
/// Because of this there is no (simple) way for the Builder to return an instance (e.g. `App`)
/// that the user can call `iced::Application::run()` on--`iced` [does not allow an instance of
/// `iced::Application`
/// to be run](https://github.com/hecrj/iced/issues/265#issuecomment-609941853).
///
/// `iced::Settings` must be returned in such a way that `iced::Application` is not invoked and
/// the user of this crate can call `.run()` on (to conform to this crate's app domain API.  That
/// is `AppBootstrapper`'s purpose.
///
/// `AppBootStrapper` holds the `iced::Settings` type, and invokes the
/// `iced::Application::run()` constructor passing in `iced::Settings` which contains both `iced`'s
/// required settings as well as `mandel`'s custom settings (this latter as the `.flags` field).
///
/// The framework will pass just the custom portion (`iced::Settings::flags`) to
/// `Application::new`, where that secondary constructor will instantiate and populate *that*
/// instance with the custom data required by `mandel.
#[derive(Debug)]
pub struct MandelBootstrapper {
    pub iced_settings: iced::Settings<MandelUserSettings>,
}

impl App for MandelBootstrapper {
    fn run(self) -> Result<(), Error> {
        <MandelApp<'_> as iced::Application>::run(self.iced_settings);
        Err(Error::AppReturnedUnexpectedly)
    }
}

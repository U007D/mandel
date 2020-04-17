use crate::{
    adapters::ui::iced::{MandelAppFactory, MandelUserSettings},
    consts::msg,
    ports::ui::{AppBuilder, Color, Size, WindowSettings},
    Error, Result,
};
use iced;
use std::convert::{TryFrom, TryInto};

#[allow(clippy::doc_markdown)]
/// `MandelBuilder` should ordinarily `build()` the "App" instance that the caller can then `run()`.
/// But `iced` is a framework (as opposed to a library), and a strongly opinionated one at that.  
/// `iced`'s design calls for an `Application` to be constructed exclusively via the
/// `Application::run()` associated function, *not* via a more typical `::new()`.
///
/// In fact, `::run()` merely passes initialization settings to the `iced` framework, and the
/// *framework* calls `Application::new()` internally.  Put another way, if the caller is holding
/// an *instance* of `Application`, there is no way to pass this to the framework--the framework
/// *must* do the "App" construction itself, internally,
/// [as per `iced`'s author](https://github.com/hecrj/iced/issues/265#issuecomment-609941853).
///
/// As a result, in the case of `iced`, `MandelBuilder` cannot return an instance of `MandelApp`,
/// as that instance (and its state) would not be usable by the framework.  Instead, `MandelBuilder`
/// must effectively return an "AppFactory", which, when `run()`, will instantiate the "App" in the
/// way `iced` expects.  `MandelBuilder` is technically `MandelAppFactoryBuilder` when adapting
/// `iced` to `Mandel`'s domain API, but this is a implementation detail particular to `iced`
/// which has no business in the domain API.  Thus the caller can use `MandelBuilder `idiomatically
/// (e.g. `MandelBuilder::build().set_...().build().run()`) and need not worry (or even be aware) of
/// these implementation details.
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
    type App = MandelAppFactory;
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
                canvas_color: mb.canvas_color.unwrap_or_else(Color::default),
                title: mb
                    .title
                    .clone()
                    .unwrap_or_else(|| String::from(msg::UNTITLED)),
                window_settings: mb
                    .window_state
                    .clone()
                    .unwrap_or_else(WindowSettings::default),
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

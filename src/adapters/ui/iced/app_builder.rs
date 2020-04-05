use crate::consts::msg;
use crate::{
    ports::ui::{AppBuilderTrait, WindowState},
    App, Error, Result,
};
use iced;
use std::convert::TryInto;

#[derive(Debug, Default)]
pub struct AppBuilder {
    title: Option<String>,
    window_state: Option<WindowState>,
}

impl AppBuilderTrait for AppBuilder {
    type AppTraitImpl = App;

    fn build(mut self) -> Result<Self::AppTraitImpl, Error> {
        let dims = self.window_state.as_ref().map_or_else(
            || WindowState::default().dimensions(),
            WindowState::dimensions,
        );
        Ok(Self::AppTraitImpl {
            iced_settings: iced::Settings {
                window: iced::window::Settings {
                    size: dims
                        .try_into()
                        .map_err(|e| Error::AdapterUiIcedScreenDimsTooLarge(dims, e))?,
                    resizable: self.window_state.as_ref().map_or_else(
                        || WindowState::default().is_resizable(),
                        WindowState::is_resizable,
                    ),
                    decorations: true,
                },
                flags: AdditionalSettings {
                    title: self
                        .title
                        .take()
                        .unwrap_or_else(|| String::from(msg::UNTITLED)),
                },
                default_font: None,
                antialiasing: false,
            },
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

#[derive(Debug, Clone)]
pub struct AdditionalSettings {
    pub(super) title: String,
}

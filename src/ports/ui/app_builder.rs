use crate::{ports::ui::WindowSettings, App, Result};

pub trait AppBuilder {
    type App: App;

    fn new() -> Self;
    fn build(self) -> Result<Self::App>;
    fn set_title(self, title: impl ToString) -> Self;
    fn set_window_state(self, window_state: WindowSettings) -> Self;
}

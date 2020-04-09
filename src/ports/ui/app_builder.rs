use crate::{ports::ui::WindowState, App, Result};

pub trait AppBuilder {
    type App: App;

    fn new() -> Self;
    fn build(self) -> Result<Self::App>;
    fn set_title(self, title: impl ToString) -> Self;
    fn set_window_state(self, window_state: WindowState) -> Self;
}

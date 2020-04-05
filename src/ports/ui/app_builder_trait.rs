use crate::{ports::ui::WindowState, AppTrait, Result};

pub trait AppBuilderTrait {
    type App: AppTrait;

    fn new() -> Self;
    fn build(self) -> Result<Self::App>;
    fn set_title(self, title: impl ToString) -> Self;
    fn set_window_state(self, window_state: WindowState) -> Self;
}

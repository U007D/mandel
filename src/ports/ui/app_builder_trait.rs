use crate::{ports::ui::WindowState, AppTrait, Result};

pub trait AppBuilderTrait {
    type AppTraitImpl: AppTrait;

    fn build(self) -> Result<Self::AppTraitImpl>;
    fn set_title(self, title: impl ToString) -> Self;
    fn set_window_state(self, window_state: WindowState) -> Self;
}

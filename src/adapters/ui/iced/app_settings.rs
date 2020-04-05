use crate::ports::ui::WindowState;

#[derive(Clone, Debug, Default)]
pub struct AppSettings {
    pub title: String,
    pub window_state: WindowState,
}

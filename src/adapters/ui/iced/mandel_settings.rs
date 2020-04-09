use crate::ports::ui::WindowState;

#[derive(Clone, Debug, Default)]
pub struct MandelSettings {
    pub title: String,
    pub window_state: WindowState,
}

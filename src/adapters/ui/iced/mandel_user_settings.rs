use crate::ports::ui::WindowSettings;

#[derive(Clone, Debug, Default)]
pub struct MandelUserSettings {
    pub title: String,
    pub window_settings: WindowSettings,
    pub i: usize,
}

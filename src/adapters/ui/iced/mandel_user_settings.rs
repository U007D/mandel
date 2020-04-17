use crate::ports::ui::{Color, WindowSettings};

#[derive(Clone, Debug, Default)]
pub struct MandelUserSettings {
    pub canvas_color: Color<f32>,
    pub title: String,
    pub window_settings: WindowSettings,
}

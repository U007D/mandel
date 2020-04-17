use crate::ports::ui::{Color, WindowSettings};

#[derive(Clone, Debug, Default)]
pub struct MandelUserSettings {
    pub title: String,
    pub window_settings: WindowSettings,
    pub i: usize,
    pub canvas_color: Color<f32>,
}

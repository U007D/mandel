use crate::{
    ports::ui::{Color, WindowSettings},
    App, Result,
};
use num::Num;

pub trait AppBuilder {
    type App: App;
    type ColorValueType: Num;

    fn new() -> Self;
    fn build(self) -> Result<Self::App>;
    fn set_title<S: ToString>(self, title: S) -> Self;
    fn set_window_settings(self, window_settings: WindowSettings) -> Self;
    fn set_canvas_color(self, color: Color<Self::ColorValueType>) -> Self;
}

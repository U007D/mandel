use crate::ports::ui::Color;

#[derive(Clone, Debug, PartialEq)]
pub enum MandelMessage {
    ClearCanvas(Color<f32>),
}

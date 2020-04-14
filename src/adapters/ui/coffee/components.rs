use crate::ports::ui::Color;
use ::coffee::graphics::Color as CoffeeColor;

impl From<Color<f32>> for CoffeeColor {
    fn from(color: Color<f32>) -> Self {
        Self::new(color.0, color.1, color.2, color.3)
    }
}

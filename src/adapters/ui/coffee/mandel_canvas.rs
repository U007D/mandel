use crate::ports::ui::Color;
use coffee::graphics::{Color as CoffeeColor, Frame, Window};
use coffee::load::Task;
use coffee::{Game, Timer};

#[derive(Debug)]
pub struct MandelCanvas {
    color: Color<f32>,
    foreign_color: CoffeeColor,
}

impl Game for MandelCanvas {
    type Input = ();
    type LoadingScreen = ();

    fn load(_window: &Window) -> Task<Self>
    where
        Self: Sized,
    {
        Task::succeed(Self::default)
    }

    fn draw(&mut self, frame: &mut Frame<'_>, _timer: &Timer) {
        frame.clear(self.color.into());
    }
}

impl Default for MandelCanvas {
    fn default() -> Self {
        let color = Color::default();
        Self {
            color,
            foreign_color: color.into(),
        }
    }
}

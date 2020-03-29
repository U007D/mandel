use coffee::graphics::{Color, Frame, Window};
use coffee::input::KeyboardAndMouse;
use coffee::load::Task;
use coffee::{Game, Timer};

pub(super) struct MandelMvc {}

impl Game for MandelMvc {
    type Input = KeyboardAndMouse;
    type LoadingScreen = ();

    fn load(_window: &Window) -> Task<Self>
    where
        Self: Sized,
    {
        Task::succeed(|| Self {})
    }

    fn draw(&mut self, frame: &mut Frame<'_>, _timer: &Timer) {
        frame.clear(Color {
            r: 0.3,
            g: 0.3,
            b: 0.6,
            a: 1.0,
        });
    }
}

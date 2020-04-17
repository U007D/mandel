mod size;
mod window;
use crate::Result;
pub use size::Size;
pub use window::{NamedWindowSize, WindowSettings};

pub trait App {
    fn run(self) -> Result<()>;
}

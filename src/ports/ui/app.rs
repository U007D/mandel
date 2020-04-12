mod size;
mod window;
use crate::Result;
pub use size::Size;
pub use window::{NamedWindowDimensions, WindowDimensions, WindowSettings};

pub trait App {
    fn run(self) -> Result<()>;
}

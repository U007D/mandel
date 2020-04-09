mod pair;
mod window;
use crate::Result;
pub use pair::Pair;
pub use window::{NamedWindowDimensions, WindowDimensions, WindowState};

pub trait App {
    fn run(self) -> Result<()>;
}

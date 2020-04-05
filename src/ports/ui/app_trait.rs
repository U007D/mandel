mod pair;
mod window;
use crate::Result;
pub use pair::Pair;
pub use window::{NamedWindowDimensions, WindowDimensions, WindowState};

pub trait AppTrait {
    fn run(self) -> Result<()>;
}

mod pair;
mod window;
use crate::{AppBuilderTrait, Result};
pub use pair::Pair;
pub use window::{NamedWindowDimensions, WindowDimensions, WindowState};

pub trait AppTrait {
    fn new<AB: AppBuilderTrait + Default>() -> AB;
    fn run(self) -> Result<()>;
}

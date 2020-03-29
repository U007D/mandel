use crate::adapters::ui::coffee::WindowSizeState;
use crate::Result;

pub trait Windowable {
    fn new<WB: WindowBuildable + Default>() -> WB;
    fn open(&self) -> Result<()>;
}

pub trait WindowBuildable {
    type WindowableImpl: Windowable;

    fn build(self) -> Self::WindowableImpl;
    fn set_dimensions(self, sd: &ScreenDimension) -> Self;
    fn set_title(self, title: impl ToString) -> Self;
    fn set_window_size_state(self, window_size_state: WindowSizeState) -> Self;
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct RectSize(pub usize, pub usize);

#[allow(dead_code)]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ScreenDimension {
    Eighth,
    Exact(RectSize),
    Full,
    Quarter,
}

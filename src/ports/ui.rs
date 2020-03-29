use crate::Result;

pub trait Window {
    fn new<WB: WindowBuilder + Default>() -> WB;
    fn open(&self) -> Result<()>;
}

pub trait WindowBuilder {
    type WindowAdapter: Window;

    fn build(self) -> Self::WindowAdapter;
    fn dimensions(self, dim: ScreenDimension) -> Self;
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

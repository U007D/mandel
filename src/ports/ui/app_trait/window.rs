use super::Pair;

#[allow(dead_code, clippy::pub_enum_variant_names)]
#[derive(Debug)]
pub enum NamedWindowDimensions {
    EighthScreen,
    FullScreen,
    QuarterScreen,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct WindowDimensions(Pair<usize>);

impl From<NamedWindowDimensions> for WindowDimensions {
    fn from(nsd: NamedWindowDimensions) -> Self {
        match nsd {
            // TODO: Make proper impl
            NamedWindowDimensions::EighthScreen => Self(Pair::from((100, 100))),
            // TODO: Make dynamic, remove `integer_arithmetic` allowance
            #[allow(clippy::integer_arithmetic)]
            NamedWindowDimensions::QuarterScreen => Self(Pair::from((3840 / 2, 2160 / 2))),
            // TODO: Make proper impl
            NamedWindowDimensions::FullScreen => Self(Pair::from((1000, 1000))),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Eq, PartialEq)]
pub enum WindowState {
    //    NonResizable(WindowDimensions),
    Resizable(WindowDimensions),
    //    Maximized,
    //    FullScreen,
}

impl WindowState {
    #[allow(clippy::unused_self)]
    pub const fn is_resizable(&self) -> bool {
        true
    }

    pub fn dimensions(&self) -> Pair<usize> {
        match self {
            Self::Resizable(wd) => wd.0,
        }
    }
}
impl Default for WindowState {
    fn default() -> Self {
        Self::Resizable(WindowDimensions::from(NamedWindowDimensions::QuarterScreen))
    }
}

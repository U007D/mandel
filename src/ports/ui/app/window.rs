use crate::ports::ui::Size;

#[allow(dead_code, clippy::pub_enum_variant_names)]
#[derive(Debug)]
pub enum NamedWindowDimensions {
    EighthScreen,
    FullScreen,
    QuarterScreen,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct WindowDimensions(Size<usize>);

impl From<NamedWindowDimensions> for WindowDimensions {
    fn from(nsd: NamedWindowDimensions) -> Self {
        match nsd {
            // TODO: Make proper impl
            NamedWindowDimensions::EighthScreen => Self(Size::from((100, 100))),
            // TODO: Make dynamic, remove `integer_arithmetic` allowance
            #[allow(clippy::integer_arithmetic)]
            NamedWindowDimensions::QuarterScreen => Self(Size::from((3840 / 2, 2160 / 2))),
            // TODO: Make proper impl
            NamedWindowDimensions::FullScreen => Self(Size::from((1000, 1000))),
        }
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WindowSettings {
    //    NonResizable(WindowDimensions),
    Resizable(WindowDimensions),
    //    Maximized,
    //    FullScreen,
}

impl WindowSettings {
    #[allow(clippy::unused_self)]
    pub const fn is_resizable(&self) -> bool {
        true
    }

    pub fn size(&self) -> Size<usize> {
        match self {
            Self::Resizable(wd) => Size((wd.0).0, (wd.0).1),
        }
    }
}
impl Default for WindowSettings {
    fn default() -> Self {
        Self::Resizable(WindowDimensions::from(NamedWindowDimensions::QuarterScreen))
    }
}

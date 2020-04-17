use crate::ports::ui::Size;

#[allow(dead_code, clippy::pub_enum_variant_names)]
#[derive(Debug)]
pub enum NamedWindowSize {
    EighthScreen,
    FullScreen,
    QuarterScreen,
}

impl From<NamedWindowSize> for Size<usize> {
    fn from(nwd: NamedWindowSize) -> Self {
        match nwd {
            // TODO: Make proper impl
            NamedWindowSize::EighthScreen => Self::from((100, 100)),
            // TODO: Make dynamic, remove `integer_arithmetic` allowance
            #[allow(clippy::integer_arithmetic)]
            NamedWindowSize::QuarterScreen => Self::from((3840 / 2, 2160 / 2)),
            // TODO: Make proper impl
            NamedWindowSize::FullScreen => Self::from((1000, 1000)),
        }
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WindowSettings {
    //    NonResizable(WindowDimensions),
    Resizable(Size<usize>),
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
            Self::Resizable(wd) => Size(wd.0, wd.1),
        }
    }
}
impl Default for WindowSettings {
    fn default() -> Self {
        Self::Resizable(NamedWindowSize::QuarterScreen.into())
    }
}

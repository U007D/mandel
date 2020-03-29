use crate::ports::ui::RectSize;
use crate::{
    adapters::ui::coffee::{Window, WindowSizeState},
    Error,
};
use coffee::graphics::WindowSettings as CoffeeWindowSettings;
use std::cmp::max;
use std::convert::TryFrom;

pub(super) struct WindowSettings(pub(super) CoffeeWindowSettings);

impl TryFrom<&Window> for WindowSettings {
    type Error = Error;

    fn try_from(wa: &Window) -> Result<Self, Self::Error> {
        Ok(Self(CoffeeWindowSettings {
            title: wa.title.clone(),
            size: <(u32, u32)>::try_from(&wa.size)?,
            resizable: Resizable::from(&wa.size_state).0,
            fullscreen: FullScreen::from(&wa.size_state).0,
            maximized: Maximized::from(&wa.size_state).0,
        }))
    }
}

pub(super) struct Resizable(bool);

impl From<&WindowSizeState> for Resizable {
    fn from(value: &WindowSizeState) -> Self {
        match value {
            WindowSizeState::IntermediateResizable => Self(true),
            _ => Self(false),
        }
    }
}

pub(super) struct FullScreen(bool);

impl From<&WindowSizeState> for FullScreen {
    fn from(value: &WindowSizeState) -> Self {
        match value {
            WindowSizeState::FullScreen => Self(true),
            _ => Self(false),
        }
    }
}

pub(super) struct Maximized(bool);

impl From<&WindowSizeState> for Maximized {
    fn from(value: &WindowSizeState) -> Self {
        match value {
            WindowSizeState::Maximized => Self(true),
            _ => Self(false),
        }
    }
}

impl TryFrom<&RectSize> for (u32, u32) {
    type Error = Error;

    fn try_from(rect_size: &RectSize) -> Result<Self, Self::Error> {
        #[allow(clippy::cast_possible_truncation)]
        match u32::try_from(max(rect_size.0, rect_size.1)).is_ok() {
            true => Ok((rect_size.0 as u32, rect_size.1 as u32)),
            false => Err(Error::AdapterUiIcedScreenDimsTooLarge(rect_size.clone())),
        }
    }
}

mod iced_app_window;
use crate::ports::ui::{RectSize, ScreenDimension, Window, WindowBuilder};
use crate::Error;
use iced::Application;
use iced_app_window::{settings, IcedAppWindow};

#[derive(Debug, PartialEq)]
pub struct WindowAdapter {
    size: RectSize,
}

impl Window for WindowAdapter {
    #[allow(clippy::new_ret_no_self)]
    fn new<WB: WindowBuilder + Default>() -> WB {
        WB::default()
    }

    fn open(&self) -> Result<(), Error> {
        IcedAppWindow::run(settings(&self.size)?);
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq)]
pub struct WindowBuilderAdapter {
    size: Option<RectSize>,
}

#[allow(clippy::integer_arithmetic)] // TODO: Remove when window size determined dynamically
fn dim_to_rect_size(dim: ScreenDimension) -> RectSize {
    // TODO: Determine screen resolution from `iced`, Rust or OS
    let width: usize = 3440 / 2;
    let height: usize = 1440 / 2;

    #[allow(clippy::match_same_arms)] // TODO: Remove when window size determined dynamically
    match dim {
        ScreenDimension::Eighth => RectSize(width, height),
        ScreenDimension::Exact(rect_size) => rect_size,
        ScreenDimension::Full => RectSize(width, height),
        ScreenDimension::Quarter => RectSize(width, height),
    }
}

impl WindowBuilder for WindowBuilderAdapter {
    type WindowAdapter = WindowAdapter;

    fn build(self) -> Self::WindowAdapter {
        WindowAdapter {
            size: self
                .size
                .unwrap_or_else(|| dim_to_rect_size(ScreenDimension::Quarter)),
        }
    }

    fn dimensions(self, dim: ScreenDimension) -> Self {
        Self {
            size: Some(dim_to_rect_size(dim)),
        }
    }
}

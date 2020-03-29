mod mandel_mvc;
mod type_converters;
mod window_size_state;

use crate::{
    consts::msg,
    ports::ui::{RectSize, ScreenDimension, WindowBuildable, Windowable},
    Error, Result,
};
use coffee::Game;
use mandel_mvc::MandelMvc;
use std::{convert::TryFrom, env};
use type_converters::WindowSettings;
pub use window_size_state::WindowSizeState;

#[derive(Debug, PartialEq)]
pub struct Window {
    size: RectSize,
    size_state: WindowSizeState,
    title: String,
}

impl Windowable for Window {
    #[allow(clippy::new_ret_no_self)]
    fn new<WB: WindowBuildable + Default>() -> WB {
        WB::default()
    }

    fn open(&self) -> Result<(), Error> {
        Ok(MandelMvc::run(WindowSettings::try_from(self)?.0)?)
    }
}

#[derive(Debug, Default, PartialEq)]
pub struct WindowBuilder {
    size: Option<RectSize>,
    window_size_state: Option<WindowSizeState>,
    title: Option<String>,
}

impl WindowBuildable for WindowBuilder {
    type WindowableImpl = Window;

    fn build(self) -> Self::WindowableImpl {
        Window {
            size: self
                .size
                .unwrap_or_else(|| sd_to_rect_size(&ScreenDimension::Quarter)),
            size_state: self
                .window_size_state
                .unwrap_or_else(|| WindowSizeState::IntermediateResizable),
            title: self
                .title
                .unwrap_or_else(|| app_name().unwrap_or_else(|| String::from(msg::UNTITLED))),
        }
    }

    fn set_dimensions(mut self, sd: &ScreenDimension) -> Self {
        self.size = Some(sd_to_rect_size(sd));
        self
    }

    fn set_title(mut self, title: impl ToString) -> Self {
        self.title = Some(title.to_string());
        self
    }

    fn set_window_size_state(mut self, window_size_state: WindowSizeState) -> Self {
        self.window_size_state = Some(window_size_state);
        self
    }
}

fn app_name() -> Option<String> {
    env::args_os()
        .nth(0)
        .map(|oss| oss.to_string_lossy().to_string())
}

#[allow(clippy::integer_arithmetic)] // TODO: Determine window size dynamically and remove
fn sd_to_rect_size(sd: &ScreenDimension) -> RectSize {
    // TODO: Determine screen resolution from `iced`, Rust or OS
    let width: usize = 3840 / 2;
    let height: usize = 2160 / 2;

    #[allow(clippy::match_same_arms)] // TODO: Determine window size dynamically and remove
    match sd {
        ScreenDimension::Eighth => RectSize(width, height),
        ScreenDimension::Exact(rect_size) => rect_size.clone(),
        ScreenDimension::Full => RectSize(width, height),
        ScreenDimension::Quarter => RectSize(width, height),
    }
}

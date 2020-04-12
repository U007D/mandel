mod app;
mod app_builder;
mod components;
pub use app::{App, NamedWindowDimensions, Size, WindowDimensions, WindowSettings};
pub use app_builder::AppBuilder;
pub use components::{
    canvas::Canvas, co_ord::CoOrd, color::Color, rect::Rect, render::Render,
    view_context::ViewContext,
};

mod app_builder_trait;
mod app_trait;
mod ui_components;
pub use app_builder_trait::AppBuilderTrait;
pub use app_trait::{AppTrait, NamedWindowDimensions, Pair, WindowDimensions, WindowState};
pub use ui_components::{
    canvas_trait::CanvasTrait, co_ord::CoOrd, color::Color, rect::Rect, render_trait::RenderTrait,
    size::Size, view_trait::ViewContextTrait,
};

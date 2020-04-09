use crate::ports::ui::{Canvas, ViewContext};

pub trait Render<C, V>
where
    C: Canvas,
    V: ViewContext,
{
    fn render(view_context: &V, canvas: &mut C);
}

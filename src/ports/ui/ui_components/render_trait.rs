use crate::ports::ui::{CanvasTrait, ViewContextTrait};

pub trait RenderTrait<C, V>
where
    C: CanvasTrait,
    V: ViewContextTrait,
{
    fn render(view_context: &V, canvas: &mut C);
}

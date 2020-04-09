use crate::ports::ui::Rect;
use num::Num;

pub trait ViewContext {
    type Num: Num;

    fn from_rect(rect: Rect<Self::Num>);
    fn rect(&self) -> &Rect<Self::Num>;
}

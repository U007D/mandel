use crate::ports::ui::Point;
use num::Num;

#[derive(Clone, Debug, PartialEq)]
pub struct Rect<N: Num>(pub Point<N>, pub Point<N>);

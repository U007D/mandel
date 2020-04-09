use crate::ports::ui::CoOrd;
use num::Num;

#[derive(Clone, Debug, PartialEq)]
pub struct Rect<N: Num>(CoOrd<N>, CoOrd<N>);

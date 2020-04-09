use crate::ports::ui::{CoOrd, Color, Size};
use num::{Num, PrimInt};

pub trait Canvas {
    type Int: PrimInt;
    type Num: Num;
    fn new(size: Size<Self::Int>) -> Self;
    fn clear(&mut self, color: Color<Self::Num>) -> &mut Self;
    fn pixel(&self, co_ord: CoOrd<Self::Int>) -> Color<Self::Num>;
    fn set_pixel(&mut self, co_ord: CoOrd<Self::Int>, color: Color<Self::Num>) -> &mut Self;
}

use crate::ports::ui::{CoOrd, Color, Size};
use num::{Num, PrimInt};
use std::error::Error;

pub trait Canvas {
    type Error: Error;
    type Int: PrimInt;
    type Num: Num;
    type ForeignCanvas;
    type ForeignColor;

    fn new(size: Size<Self::Int>) -> Result<Self, Self::Error>
    where
        Self: Sized;
    fn clear(&mut self, color: &Color<Self::Num>) -> &mut Self;
    fn pixel(&self, co_ord: &CoOrd<Self::Int>) -> &Color<Self::Num>;
    fn set_pixel(&mut self, co_ord: &CoOrd<Self::Int>, color: &Color<Self::Num>) -> &mut Self;
}

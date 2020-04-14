use crate::ports::ui::{CoOrd, Color, Size};
use num::{Num, PrimInt};
use std::error::Error;

pub trait Canvas {
    type Error: Error;
    type PixelCoordType: PrimInt;
    type ColorValueType: Num;
    type ForeignCanvas;
    type ForeignColor;

    fn new(size: Size<Self::PixelCoordType>) -> Result<Self, Self::Error>
    where
        Self: Sized;
    fn clear(&mut self, color: &Color<Self::ColorValueType>) -> &mut Self;
    fn pixel(&self, co_ord: &CoOrd<Self::PixelCoordType>) -> &Color<Self::ColorValueType>;
    fn set_pixel(
        &mut self,
        co_ord: &CoOrd<Self::PixelCoordType>,
        color: &Color<Self::ColorValueType>,
    ) -> &mut Self;
}

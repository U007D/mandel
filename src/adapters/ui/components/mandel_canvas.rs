use crate::ports::ui::{Canvas, CoOrd, Color, Size};

#[derive(Debug)]
pub struct MandelCanvas {
    size: Size<<Self as Canvas>::Int>,
}

impl Canvas for MandelCanvas {
    type Int = usize;
    type Num = f32;

    fn new(size: Size<Self::Int>) -> Self {
        Self { size }
    }

    fn clear(&mut self, color: Color<Self::Num>) -> &mut Self {
        unimplemented!()
    }

    fn pixel(&self, co_ord: CoOrd<Self::Int>) -> Color<Self::Num> {
        unimplemented!()
    }

    fn set_pixel(&mut self, co_ord: CoOrd<Self::Int>, color: Color<Self::Num>) -> &mut Self {
        unimplemented!()
    }
}

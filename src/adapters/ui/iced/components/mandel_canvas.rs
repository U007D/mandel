use crate::{
    ports::ui::{Canvas, CoOrd, Color, Size},
    Error, Result,
};
use iced;
use std::convert::TryInto;

#[derive(Debug)]
pub struct MandelCanvas<'a> {
    iced_canvas: <Self as Canvas>::ForeignCanvas,
    iced_color_context: <Self as Canvas>::ForeignColor,
}

impl<'a> Canvas for MandelCanvas<'a> {
    type Error = Error;
    type Int = usize;
    type Num = f32;
    type ForeignCanvas = iced::Canvas<'a>;
    type ForeignColor = iced::Color;

    fn new(size: Size<Self::Int>) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        Ok(Self {
            iced_canvas: iced::Canvas::new()
                .width(iced::Length::Units(size.0.try_into()?))
                .height(iced::Length::Units(size.1.try_into()?)),
            iced_color_context: iced::Color::TRANSPARENT,
        })
    }

    fn clear(&mut self, color: &Color<Self::Num>) -> &mut Self {
        self.iced_color_context = iced::Color {
            r: color.0,
            g: color.1,
            b: color.2,
            a: color.3,
        };

        self
    }

    #[inline]
    fn pixel(&self, co_ord: &CoOrd<Self::Int>) -> &Color<Self::Num> {
        unimplemented!()
    }

    #[inline]
    fn set_pixel(&mut self, co_ord: &CoOrd<Self::Int>, color: &Color<Self::Num>) -> &mut Self {
        unimplemented!()
    }
}

impl iced::canvas::Drawable for MandelCanvas<'_> {
    fn draw(&self, frame: &mut iced::canvas::Frame) {
        frame.fill(
            &iced::canvas::Path::new(|p| {
                p.rectangle(
                    iced::Point::ORIGIN,
                    iced::Size::new(frame.width(), frame.height()),
                )
            }),
            iced::canvas::Fill::Color(self.iced_color_context),
        );
    }
}

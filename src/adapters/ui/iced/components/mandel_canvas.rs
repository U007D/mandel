use crate::{
    ports::ui::{Canvas, Color, Point, Size},
    Error, Result,
};
use iced;
use iced_native::layout::{Limits, Node};
use iced_native::{Hasher, Layout};

#[derive(Clone, Debug, PartialEq)]
pub enum DrawCommand {
    Clear(iced::Color),
}

#[derive(Clone, Debug)]
pub struct MandelCanvas<'a> {
    display_list: Vec<DrawCommand>,
    iced_color_context: <Self as Canvas>::ForeignColor,
}

impl MandelCanvas<'_> {
    pub fn display_list(&self) -> impl Iterator<Item = &DrawCommand> + '_ {
        self.display_list.iter()
    }
}

impl<'a> Canvas for MandelCanvas<'a> {
    type Error = Error;
    type PixelCoordType = usize;
    type ColorValueType = f32;
    type ForeignCanvas = iced::Canvas<'a>;
    type ForeignColor = iced::Color;

    fn new(
        size: Size<Self::PixelCoordType>,
        background_color: Color<Self::ColorValueType>,
    ) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        Ok(Self {
            display_list: Vec::new(),
            iced_color_context: background_color.into(),
        })
    }

    fn clear(&mut self, color: &Color<Self::ColorValueType>) -> &mut Self {
        self.display_list.push(DrawCommand::Clear(color.into()));

        self
    }

    #[inline]
    fn pixel(&self, co_ord: &Point<Self::PixelCoordType>) -> &Color<Self::ColorValueType> {
        unimplemented!()
    }

    #[inline]
    fn set_pixel(
        &mut self,
        co_ord: &Point<Self::PixelCoordType>,
        color: &Color<Self::ColorValueType>,
    ) -> &mut Self {
        unimplemented!()
    }
}

impl<Message, Renderer: iced_native::Renderer> iced_native::Widget<Message, Renderer>
    for MandelCanvas<'_>
{
    fn width(&self) -> iced::Length {
        iced::Length::Fill
    }

    fn height(&self) -> iced::Length {
        iced::Length::Fill
    }

    fn layout(&self, _renderer: &Renderer, limits: &Limits) -> Node {
        Node::default()
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        defaults: &Renderer::Defaults,
        layout: Layout<'_>,
        cursor_position: iced::Point,
    ) -> Renderer::Output {
        unimplemented!()
        // self.display_list.iter().for_each(|dc| match dc {
        //     DrawCommand::Clear(color) => frame.fill(
        //         &iced::canvas::Path::new(|p| {
        //             p.rectangle(
        //                 iced::Point::ORIGIN,
        //                 iced::Size::new(frame.width(), frame.height()),
        //             )
        //         }),
        //         iced::canvas::Fill::Color(self.iced_color_context),
        //     ),
        // });
    }

    fn hash_layout(&self, state: &mut Hasher) {
        //unimplemented!()
    }
}

impl<'a, M> From<MandelCanvas<'a>> for iced::Element<'a, M> {
    fn from(mc: MandelCanvas<'a>) -> Self {
        Self::new(mc)
    }
}

use crate::{
    adapters::ui::{iced::mandel_user_settings::MandelUserSettings, MandelCanvas},
    consts::msg,
    ports::ui::{Canvas, Color, Point, Rect},
};
use iced;

#[derive(Clone, Debug, PartialEq)]
pub enum MandelMessage {
    ClearCanvas(Color<f32>),
}

#[derive(Debug)]
pub struct MandelApp<'a> {
    canvas: MandelCanvas<'a>,
    mandelbrot: iced::canvas::layer::Cache<State>,
    state: State,
    user_settings: MandelUserSettings,
}

impl iced::Application for MandelApp<'_> {
    type Executor = iced::executor::Default;
    type Message = MandelMessage;
    type Flags = MandelUserSettings;

    fn new(user_settings: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            Self {
                canvas: MandelCanvas::new(
                    user_settings.window_settings.size(),
                    user_settings.canvas_color,
                )
                .expect(msg::ERR_ADAPTER_UI_WINDOW_TOO_LARGE),
                mandelbrot: iced::canvas::layer::Cache::<State>::default(),
                state: State::new(
                    Rect(Point(-2.0, 1.5), Point(1.0, -1.5)),
                    user_settings.canvas_color,
                ),
                user_settings,
            },
            iced::Command::none(),
        )
    }

    fn title(&self) -> String {
        self.user_settings.title.clone()
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            MandelMessage::ClearCanvas(color) => {
                self.canvas.clear(&color);
            }
        };
        iced::Command::none()
    }

    #[allow(clippy::integer_arithmetic)]
    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        let canvas = iced::Canvas::new()
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .push(self.mandelbrot.with(&self.state));

        iced::Container::new(canvas)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .into()
    }
}

#[derive(Clone, Debug)]
struct State {
    message: MandelMessage,
    view_rect: Rect<f64>,
    default_canvas_color: Color<f32>,
}

impl State {
    pub fn new(view_rect: Rect<f64>, default_canvas_color: Color<f32>) -> Self {
        Self {
            message: MandelMessage::ClearCanvas(default_canvas_color),
            view_rect,
            default_canvas_color,
        }
    }
}

impl iced::canvas::Drawable for State {
    fn draw(&self, frame: &mut iced::canvas::Frame) {
        match &self.message {
            MandelMessage::ClearCanvas(color) => {
                let mandel_space = iced::canvas::Path::new(|path| {
                    path.rectangle(iced::Point::new(0.0, 0.0), frame.size())
                });
                frame.fill(&mandel_space, iced::canvas::Fill::Color(color.into()));
            }
        }
    }
}

impl From<Color<f32>> for iced::Color {
    fn from(color: Color<f32>) -> Self {
        Self {
            r: color.0,
            g: color.1,
            b: color.2,
            a: color.3,
        }
    }
}

impl From<&Color<f32>> for iced::Color {
    fn from(color: &Color<f32>) -> Self {
        (*color).into()
    }
}

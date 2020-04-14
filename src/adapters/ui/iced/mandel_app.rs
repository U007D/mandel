use crate::adapters::ui::iced::components::mandel_canvas::DrawCommand;
use crate::consts::msg;
use crate::ports::ui::Color;
use crate::{
    adapters::ui::{iced::mandel_user_settings::MandelUserSettings, MandelCanvas},
    ports::ui::Canvas,
};
use iced;

#[derive(Clone, Debug, PartialEq)]
pub enum MandelMessage {
    ClearCanvas(Color<f32>),
    ButtonPressed,
}

#[derive(Debug)]
pub struct MandelApp<'a> {
    user_settings: MandelUserSettings,
    canvas: MandelCanvas<'a>,
    button_state: iced::button::State,
    press_count: usize,
}

impl iced::Application for MandelApp<'_> {
    type Executor = iced::executor::Default;
    type Message = MandelMessage;
    type Flags = MandelUserSettings;

    fn new(user_settings: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            Self {
                canvas: MandelCanvas::new(user_settings.window_settings.size())
                    .expect(msg::ERR_ADAPTER_UI_WINDOW_TOO_LARGE),
                user_settings,
                button_state: iced::button::State::default(),
                press_count: 0,
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
            #[allow(clippy::integer_arithmetic)]
            MandelMessage::ButtonPressed => {
                self.press_count += 1;
                self.canvas.clear(&Color(1.0, 1.0, 0.0, 0.0));
            }
        };
        iced::Command::none()
    }

    #[allow(clippy::integer_arithmetic)]
    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        self.user_settings.i += 1;

        let button = iced::Button::new(&mut self.button_state, iced::Text::new("Press to Clear"))
            .on_press(MandelMessage::ButtonPressed);

        let text = iced::Text::new(format!(
            "Welcome to my Mandelbrot app! ({})",
            self.press_count
        ))
        .size(50);

        let mut content = iced::Column::new()
            .align_items(iced::Align::Center)
            .spacing(20)
            .push(text)
            .push(button);

        self.canvas.display_list().for_each(|dc| {
            match dc {
                DrawCommand::Clear(color) => content.push(self.canvas.clone()),
            };
        });

        iced::Container::new(content)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

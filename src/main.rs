use iced::{
    button, image::Handle, mouse::Interaction, Application, Button, Clipboard, Command, Container,
    HorizontalAlignment, Image, Length, Size, Subscription, Text, VerticalAlignment,
};
use image::{buffer::ConvertBuffer, ImageBuffer};
use std::{
    cell::UnsafeCell,
    fs::File,
    hash::Hash,
    path::Path,
    time::{Duration, Instant},
};

mod animated_image;

use animated_image::AnimatedImage;

fn main() {
    State::run(Default::default()).unwrap();
}

#[derive(Debug, Clone)]
enum Message {
    Idle,
    /// animation frame
    Tick,
}

struct State {
    gif_image: AnimatedImage,
}

impl Application for State {
    type Executor = iced::executor::Default;
    type Flags = ();
    type Message = self::Message;

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let path = format!("{}/images/color-example.gif", env!("CARGO_MANIFEST_DIR"));
        //let path = format!("{}/images/animated-splash.gif", env!("CARGO_MANIFEST_DIR"));
        let gif_image = AnimatedImage::from_gif(path).unwrap();
        let app = Self { gif_image };
        (app, Command::none())
    }

    fn title(&self) -> String {
        "Map example".to_owned()
    }

    fn update(&mut self, message: Self::Message, _clipboard: &mut Clipboard) -> Command<Message> {
        match message {
            Message::Tick => {}
            _ => {}
        }
        Command::none()
    }

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        let image = self.gif_image.view();

        Container::new(image)
            .align_x(iced::Align::Center)
            .align_y(iced::Align::Center)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(30)
            .into()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        const FPS: f32 = 60.0;
        iced::time::every(Duration::from_secs_f32(1.0 / FPS)).map(|_tick| Message::Tick)
    }
}

mod style {
    use iced::{button, Color};

    pub struct Button;
    impl button::StyleSheet for Button {
        fn active(&self) -> button::Style {
            button::Style {
                background: Some(Color::BLACK.into()),
                text_color: Color::WHITE,
                ..Default::default()
            }
        }
    }
}

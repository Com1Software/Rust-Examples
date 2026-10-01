use iced::widget::{button, column, text};
use iced::{Alignment, Element, Sandbox, Settings};

pub fn main() -> iced::Result {
    Counter::run(Settings::default())
}

#[derive(Default)]
struct Counter {
    value: i32,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Increment,
    Decrement,
}

impl Sandbox for Counter {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Iced Basic Example")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Increment => self.value += 1,
            Message::Decrement => self.value -= 1,
        }
    }

    fn view(&self) -> Element<Message> {
        column![
            text(format!("Value: {}", self.value)).size(40),
            button("Increment").on_press(Message::Increment),
            button("Decrement").on_press(Message::Decrement),
        ]
        .padding(20)
        .align_items(Alignment::Center)
        .into()
    }
}

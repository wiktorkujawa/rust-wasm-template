use iced::{
    theme,
    widget::{
        column, container, horizontal_space, row,
        scrollable
    },
    Color, Element, Length, Sandbox, Settings
};

mod enums;
use enums::Message;

mod components;
use components::button;

mod structs;
use structs::Steps;

pub fn main() -> iced::Result {
    env_logger::init();

    Tour::run(Settings::default())
}

pub struct Tour {
    steps: Steps,
    debug: bool,
}

impl Sandbox for Tour {
    type Message = Message;

    fn new() -> Tour {
        Tour {
            steps: Steps::new(),
            debug: false,
        }
    }

    fn title(&self) -> String {
        format!("{} - Iced", self.steps.title())
    }

    fn update(&mut self, event: Message) {
        match event {
            Message::BackPressed => {
                self.steps.go_back();
            }
            Message::NextPressed => {
                self.steps.advance();
            }
            Message::StepMessage(step_msg) => {
                self.steps.update(step_msg, &mut self.debug);
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let Tour { steps, .. } = self;

        let mut controls = row![];

        if steps.has_previous() {
            controls = controls.push(
                button("Back")
                    .on_press(Message::BackPressed)
                    .style(theme::Button::Secondary),
            );
        }

        controls = controls.push(horizontal_space(Length::Fill));

        if steps.can_continue() {
            controls = controls.push(
                button("Next")
                    .on_press(Message::NextPressed)
                    .style(theme::Button::Primary),
            );
        }

        let content: Element<_> = column![
            steps.view(self.debug).map(Message::StepMessage),
            controls,
        ]
        .max_width(540)
        .spacing(20)
        .padding(20)
        .into();

        let scrollable = scrollable(
            container(if self.debug {
                content.explain(Color::BLACK)
            } else {
                content
            })
            .width(Length::Fill)
            .center_x(),
        );

        container(scrollable).height(Length::Fill).center_y().into()
    }
}
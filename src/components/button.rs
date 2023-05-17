use iced::{
  widget::{Button, text},
  alignment
};

pub fn button<'a, Message: Clone>(label: &str) -> Button<'a, Message> {
  iced::widget::button(
      text(label).horizontal_alignment(alignment::Horizontal::Center),
  )
  .padding(12)
  .width(100)
}
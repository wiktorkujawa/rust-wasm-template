use iced::Color;
use super::{Language, Layout};
pub enum Step {
  Welcome,
  Slider {
      value: u8,
  },
  RowsAndColumns {
      layout: Layout,
      spacing: u16,
  },
  Text {
      size: u16,
      color: Color,
  },
  Radio {
      selection: Option<Language>,
  },
  Toggler {
      can_continue: bool,
  },
  Image {
      width: u16,
  },
  Scrollable,
  TextInput {
      value: String,
      is_secure: bool,
      is_showing_icon: bool,
  },
  Debugger,
  End,
}


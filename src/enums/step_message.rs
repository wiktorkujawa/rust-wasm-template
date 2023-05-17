#[path = "./language.rs"]
mod language;
use iced::Color;

use crate::enums::{ 
  layout::Layout, language::Language  
};

#[derive(Debug, Clone)]
pub enum StepMessage {
    SliderChanged(u8),
    LayoutChanged(Layout),
    SpacingChanged(u16),
    TextSizeChanged(u16),
    TextColorChanged(Color),
    LanguageSelected(Language),
    ImageWidthChanged(u16),
    InputChanged(String),
    ToggleSecureInput(bool),
    ToggleTextInputIcon(bool),
    DebugToggled(bool),
    TogglerChanged(bool),
}
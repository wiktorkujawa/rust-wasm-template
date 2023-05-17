use iced::{
  widget::{ Slider, slider },
  Color,
  Renderer
};

use crate::enums::StepMessage;

pub fn color_slider<'a>(
  component: f32,
  update: impl Fn(f32) -> Color + 'a,
) -> Slider<'a, f64, StepMessage, Renderer> {
  slider(0.0..=1.0, f64::from(component), move |c| {
      StepMessage::TextColorChanged(update(c as f32))
  })
  .step(0.01)
}
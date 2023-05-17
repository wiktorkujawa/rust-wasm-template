use iced::{
  widget::{ image, container, Container },
  Length
};

use crate::enums::StepMessage;

pub fn ferris<'a>(width: u16) -> Container<'a, StepMessage> {
  container(
      // This should go away once we unify resource loading on native
      // platforms
      if cfg!(target_arch = "wasm32") {
          // image("/wasm-test/images/ferris.png")
          image("wasm-test/images/ferris.png")
          // image("/images/ferris.png")
          // image("images/ferris.png")
          // image("./images/ferris.png")

      } else {
          image(format!("{}/images/ferris.png", env!("CARGO_MANIFEST_DIR")))
      }
      .width(width),
  )
  .width(Length::Fill)
  .center_x()
}
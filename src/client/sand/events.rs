use crate::prelude::*;

#[derive(Event, Debug, Clone)]
pub struct ExampleEvent {
  pub message: String,
  pub timestamp: f32,
}

use crate::prelude::*;

#[derive(Resource, Default, Debug)]
pub struct ExampleResource {
    pub counter: usize,
    pub last_update: f64,
}

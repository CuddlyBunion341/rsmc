use bevy::ecs::event::Event;

#[derive(Event)]
pub struct ColliderUpdateEvent {
    pub position: [f32; 3],
}

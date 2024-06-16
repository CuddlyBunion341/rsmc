use bevy::{ecs::{event::*, query::With, system::Query}, input::keyboard::*, transform::components::Transform};
use crate::{collider::events::ColliderUpdateEvent, player::components::HighlightCube};

pub fn handle_keyboard_events_system(
    mut keyboard_events: EventReader<KeyboardInput>,
    camera_query: Query<&Transform, With<HighlightCube>>,
    mut collider_events: EventWriter<ColliderUpdateEvent>,
) {
    for event in keyboard_events.read() {
        if event.state.is_pressed() {
            match event.key_code {
                bevy::input::keyboard::KeyCode::Escape => std::process::exit(0),
                bevy::input::keyboard::KeyCode::KeyC => {
                    let controller_transform = camera_query.single();
                    println!("Handling event: {:?}", controller_transform.translation);
                    collider_events.send(ColliderUpdateEvent {
                        position: controller_transform.translation.into(),
                    });
                }
                _ => {}
            }
        }
    }
}


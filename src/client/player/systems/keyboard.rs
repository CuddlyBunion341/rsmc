use crate::prelude::*;

pub fn handle_keyboard_events_system(
    mut keyboard_events: EventReader<KeyboardInput>,
    camera_query: Query<&Transform, With<player_components::HighlightCube>>,
    mut collider_events: EventWriter<collider_events::ColliderUpdateEvent>,
) {
    for event in keyboard_events.read() {
        if event.state.is_pressed() {
            match event.key_code {
                bevy::input::keyboard::KeyCode::KeyC => {
                    let controller_transform = camera_query.single();
                    println!("Handling event: {:?}", controller_transform.translation);
                    collider_events.send(collider_events::ColliderUpdateEvent {
                        position: controller_transform.translation.into(),
                    });
                }
                _ => {}
            }
        }
    }
}

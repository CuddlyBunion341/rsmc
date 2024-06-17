use bevy::{ecs::{event::{EventReader, EventWriter}, system::{Query, Res}}, input::{keyboard::KeyCode, mouse::{MouseButton, MouseButtonInput}, ButtonInput}, window::{CursorGrabMode, Window}};
use bevy_fps_controller::controller::FpsController;

use crate::{player::BlockSelection, terrain::{events::BlockUpdateEvent, util::blocks::BlockId}};

pub fn manage_cursor_system(
    btn: Res<ButtonInput<MouseButton>>,
    key: Res<ButtonInput<KeyCode>>,
    mut window_query: Query<&mut Window>,
    mut controller_query: Query<&mut FpsController>,
) {
    let mut window = window_query.single_mut();
    if btn.just_pressed(MouseButton::Left) {
        window.cursor.grab_mode = CursorGrabMode::Locked;
        window.cursor.visible = false;
        for mut controller in &mut controller_query {
            controller.enable_input = true;
        }
    }
    if key.just_pressed(KeyCode::Escape) {
        window.cursor.grab_mode = CursorGrabMode::None;
        window.cursor.visible = true;
        for mut controller in &mut controller_query {
            controller.enable_input = false;
        }
    }
}

pub fn handle_mouse_events_system(
    mut block_update_events: EventWriter<BlockUpdateEvent>,
    mut mouse_events: EventReader<MouseButtonInput>,
    block_selection: Res<BlockSelection>,
) {
    if block_selection.normal.is_none() || block_selection.position.is_none() {
        return;
    }

    let position = block_selection.position.unwrap();
    let normal = block_selection.normal.unwrap();

    for event in mouse_events.read() {
        if event.button == MouseButton::Left && event.state.is_pressed() {
            block_update_events.send(BlockUpdateEvent {
                position,
                block: BlockId::Air,
            });
        } else if event.button == MouseButton::Right && event.state.is_pressed() {
            block_update_events.send(BlockUpdateEvent {
                position: position + normal,
                block: BlockId::Dirt,
            });
        }
    }
}

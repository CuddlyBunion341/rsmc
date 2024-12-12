use crate::prelude::*;
use bevy::input::{keyboard::KeyboardInput, ButtonState};
use chat_events::{ChatFocusStateChangeEvent, ChatMessageSendEvent, FocusState};

const COLOR_UNFOCUSED: Color = Color::rgba(0.0, 0.0, 0.0, 0.0);
const COLOR_FOCUSED: Color = Color::rgba(0.0, 0.0, 0.0, 0.5);
const TEXT_COLOR: Color = Color::rgba(1.0, 1.0, 1.0, 0.5);

const FONT_SIZE: f32 = 20.0;
const PADDING: UiRect = UiRect {
    top: Val::Px(10.0),
    left: Val::Px(10.0),
    bottom: Val::Px(10.0),
    right: Val::Px(10.0),
};

fn create_text_bundle(value: String, style: Style) -> TextBundle {
    TextBundle {
        text: Text {
            sections: vec![TextSection {
                value,
                style: TextStyle {
                    font_size: FONT_SIZE,
                    color: TEXT_COLOR,
                    ..default()
                },
            }],
            ..default()
        },
        style,
        ..default()
    }
}

pub fn setup_chat_container(mut commands: Commands) {
    commands
        .spawn(NodeBundle {
            style: Style {
                margin: UiRect::all(Val::Px(5.0)),
                width: Val::Percent(50.0),
                height: Val::Percent(80.0),
                flex_direction: FlexDirection::ColumnReverse,
                ..default()
            },
            background_color: BackgroundColor(COLOR_UNFOCUSED),
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(create_text_bundle(
                    String::new(),
                    Style {
                        padding: PADDING,
                        height: Val::Px(20.0),
                        ..default()
                    },
                ))
                .insert(chat_components::ChatMessageInputElement { focused: false });

            parent
                .spawn(create_text_bundle(
                    String::new(),
                    Style {
                        flex_direction: FlexDirection::Column,
                        ..default()
                    },
                ))
                .insert(chat_components::ChatMessageContainer { focused: false });
        });
}

pub fn handle_focus_events(
    mut focus_change_events: EventReader<ChatFocusStateChangeEvent>,
    mut chat_container_query: Query<
        (
            &mut BackgroundColor,
            &mut chat_components::ChatMessageContainer,
        ),
        Without<chat_components::ChatMessageInputElement>,
    >,
    mut chat_input_query: Query<
        (
            &mut BackgroundColor,
            &mut chat_components::ChatMessageInputElement,
        ),
        Without<chat_components::ChatMessageContainer>,
    >,
    mut controller_query: Query<&mut FpsController>,
    mut window_query: Query<&mut Window>,
) {
    if let Ok(mut window) = window_query.get_single_mut() {
        if let (Ok((mut container_bg, mut chat_container)), Ok((mut input_bg, mut chat_input))) = (
            chat_container_query.get_single_mut(),
            chat_input_query.get_single_mut(),
        ) {
            for event in focus_change_events.read() {
                match event.state {
                    FocusState::Focus => {
                        info!("Handling focus state");
                        container_bg.0 = COLOR_FOCUSED;
                        chat_container.focused = true;
                        input_bg.0 = COLOR_FOCUSED;
                        chat_input.focused = true;
                        for mut controller in &mut controller_query.iter_mut() {
                            controller.enable_input = false;
                        }
                    }
                    FocusState::Unfocus => {
                        info!("Handling unfocus state");
                        container_bg.0 = COLOR_UNFOCUSED;
                        chat_container.focused = false;
                        input_bg.0 = COLOR_UNFOCUSED;
                        chat_input.focused = false;
                        for mut controller in &mut controller_query.iter_mut() {
                            controller.enable_input = true;
                        }
                        window.cursor.grab_mode = CursorGrabMode::Locked;
                        window.cursor.visible = false;
                    }
                }
            }
        }
    }
}

pub fn send_messages_system(
    mut client: ResMut<RenetClient>,
    mut event_reader: EventReader<ChatMessageSendEvent>,
) {
    for event in event_reader.read() {
        let message = event.0.clone();

        info!("Sending message \"{}\" to server", message);

        client.send_message(
            DefaultChannel::ReliableOrdered,
            bincode::serialize(&NetworkingMessage::ChatMessageSend(message)).unwrap(),
        );
    }
}

pub fn focus_chat_input_system(
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut chat_input_query: Query<&mut chat_components::ChatMessageInputElement>,
    mut focus_change_events: EventWriter<ChatFocusStateChangeEvent>,
    mut chat_state: ResMut<chat_resources::ChatState>,
) {
    if let Ok(chat_input_component) = chat_input_query.get_single_mut() {
        if mouse_button_input.just_pressed(MouseButton::Left) {
            info!("Unfocusing chat via Left click");
            focus_change_events.send(ChatFocusStateChangeEvent {
                state: FocusState::Unfocus,
            });
        }
        if keyboard_input.just_pressed(KeyCode::KeyT) && !chat_input_component.focused {
            info!("Focusing chat via KeyT");
            focus_change_events.send(ChatFocusStateChangeEvent {
                state: FocusState::Focus,
            });
            chat_state.just_focused = true;
        }
        if keyboard_input.just_pressed(KeyCode::Escape) && chat_input_component.focused {
            info!("Unfocusing chat via Escape");
            focus_change_events.send(ChatFocusStateChangeEvent {
                state: FocusState::Unfocus,
            });
        }
    }
}

pub fn handle_window_focus_events(
    mut window_query: Query<&mut Window>,
    mut focus_events: EventReader<ChatFocusStateChangeEvent>,
) {
    if let Ok(mut window) = window_query.get_single_mut() {
        for event in focus_events.read() {
            match event.state {
                FocusState::Unfocus => {
                    window.cursor.grab_mode = CursorGrabMode::Locked;
                    window.cursor.visible = false;
                }
                FocusState::Focus => {}
            }
        }
    }
}

pub fn handle_chat_focus_player_controller_events(
    mut focus_change_events: EventReader<ChatFocusStateChangeEvent>,
    mut controller_query: Query<&mut FpsController>,
) {
    for event in focus_change_events.read() {
        info!("Received event to change player controller focus");
        match event.state {
            FocusState::Focus => {
                for mut controller in &mut controller_query.iter_mut() {
                    controller.enable_input = false;
                }
            }
            FocusState::Unfocus => {
                for mut controller in &mut controller_query.iter_mut() {
                    controller.enable_input = true;
                }
            }
        }
    }
}

pub fn process_chat_input_system(
    mut evr_kbd: EventReader<KeyboardInput>,
    mut chat_input_query: Query<(&mut Text, &mut chat_components::ChatMessageInputElement)>,
    mut send_event_writer: EventWriter<ChatMessageSendEvent>,
    mut chat_state: ResMut<chat_resources::ChatState>,
) {
    if let Ok((mut text, input_component)) = chat_input_query.get_single_mut() {
        if !input_component.focused {
            return;
        }

        let mut chat_input_value = text
            .sections
            .first()
            .map_or(String::new(), |s| s.value.clone());

        for ev in evr_kbd.read() {
            if ev.state != ButtonState::Pressed {
                continue;
            }

            if chat_state.just_focused {
                chat_state.just_focused = false;
                continue;
            }

            match &ev.logical_key {
                Key::Enter if !chat_input_value.trim().is_empty() => {
                    send_event_writer
                        .send(ChatMessageSendEvent(chat_input_value.trim().to_string()));
                    chat_input_value.clear();
                }
                Key::Backspace => {
                    chat_input_value.pop();
                }
                Key::Space => chat_input_value.push(' '),
                Key::Character(input) => {
                    if input.chars().all(|c| !c.is_control()) {
                        chat_input_value.push_str(input);
                    }
                }
                _ => {}
            }
        }

        text.sections.clear();
        text.sections.push(TextSection {
            value: chat_input_value.clone(),
            style: TextStyle {
                font_size: FONT_SIZE,
                color: TEXT_COLOR,
                ..default()
            },
        });
    }
}

pub fn handle_chat_message_sync_event(
    mut sync_events: EventReader<chat_events::ChatSyncEvent>,
    mut send_events: EventWriter<chat_events::SingleChatSendEvent>,
) {
    for event in sync_events.read() {
        event.0.clone().into_iter().for_each(|message| {
            send_events.send(chat_events::SingleChatSendEvent(message));
        })
    }
}

pub fn add_message_to_chat_container_system(
    mut commands: Commands,
    query: Query<(Entity, &chat_components::ChatMessageContainer)>,
    mut events: EventReader<chat_events::SingleChatSendEvent>,
) {
    for event in events.read() {
        if let Ok((entity, _)) = query.get_single() {
            commands.entity(entity).with_children(|parent| {
                parent.spawn(create_text_bundle(
                    event.0.format_string(),
                    Style {
                        margin: UiRect::all(Val::Px(5.0)),
                        ..default()
                    },
                ));
            });
        }
    }
}

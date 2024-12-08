use bevy::input::ButtonState;
use chat_events::{ChatFocusEvent, ChatUnfocusEvent, SendMessageEvent};

use crate::prelude::*;

const COLOR_UNFOCUSED: Color = Color::rgba(0.0, 0.0, 0.0, 0.0);
const COLOR_FOCUSED: Color = Color::rgba(0.0, 0.0, 0.0, 0.5);

pub fn setup_chat_container(mut commands: Commands) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(50.0),
                height: Val::Percent(80.0),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            background_color: BackgroundColor(COLOR_UNFOCUSED),
            ..default()
        })
    .insert(chat_components::ChatMessageContainer { focused: false })
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text {
                    sections: vec![TextSection {
                        value: "Welcome to the chat!".to_string(),
                        style: TextStyle {
                            font_size: 20.0,
                            color: Color::WHITE,
                            ..Default::default()
                        },
                    }],
                    ..Default::default()
                },
                style: Style {
                    margin: UiRect::all(Val::Px(5.0)),
                    ..Default::default()
                },
                ..Default::default()
            });

            parent.spawn(TextBundle {
                style: Style { ..default() },
                ..default()
            }).insert(chat_components::ChatMessageInputElement { focused: false });
        });
}

pub fn send_messages_system(mut client: ResMut<RenetClient>, mut event_reader: EventReader<SendMessageEvent>) {
    for event in event_reader.read() {
        let message = event.0.clone();

        client.send_message(
            DefaultChannel::ReliableOrdered,
            bincode::serialize(&NetworkingMessage::ChatMessageSend(message)).unwrap(),
        )
    }
}

pub fn handle_chat_focus_input_event(
    btn: Res<ButtonInput<MouseButton>>,
    key: Res<ButtonInput<KeyCode>>,
    mut chat_input_query: Query<&mut chat_components::ChatMessageInputElement>,
    mut chat_focus_events: EventWriter<ChatFocusEvent>,
    mut chat_unfocus_events: EventWriter<ChatUnfocusEvent>,
) {
    let chat_input_component = chat_input_query.single_mut();

    if btn.just_pressed(MouseButton::Left) {
        chat_unfocus_events.send(ChatUnfocusEvent());
    }
    if key.just_pressed(KeyCode::KeyT) {
        chat_focus_events.send(ChatFocusEvent());
    }
    if key.just_pressed(KeyCode::Escape) {
        if chat_input_component.focused {
            chat_unfocus_events.send(ChatUnfocusEvent());
        }
    }
}

pub fn handle_window_focus_events(
    mut window_query: Query<&mut Window>,
    mut focus_events: EventReader<ChatFocusEvent>,
) {
    let mut window = window_query.single_mut();
    for _ in focus_events.read() {
        window.cursor.grab_mode = CursorGrabMode::Locked;
        window.cursor.visible = false;
    }
}

pub fn handle_chat_focus_player_events(
    mut focus_events: EventReader<ChatFocusEvent>,
    mut unfocus_events: EventReader<ChatUnfocusEvent>,
    mut controller_query: Query<&mut FpsController>,
) {
    for _ in focus_events.read() {
        for mut controller in &mut controller_query {
            controller.enable_input = true;
        }
    }

    for _ in unfocus_events.read() {
        for mut controller in &mut controller_query {
            controller.enable_input = false;
        }
    }
}

pub fn handle_chat_container_focus_events(
    mut focus_events: EventReader<ChatFocusEvent>,
    mut unfocus_events: EventReader<ChatUnfocusEvent>,
    mut chat_container_query: Query<(
        &mut BackgroundColor,
        &mut chat_components::ChatMessageContainer,
    )>,
) {
    let (mut background_color, mut chat_container) = chat_container_query.single_mut();
    for _ in focus_events.read() {
        background_color.0 = COLOR_FOCUSED.clone();
        chat_container.focused = true;
    }

    for _ in unfocus_events.read() {
        background_color.0 = COLOR_UNFOCUSED.clone();
        chat_container.focused = false;
    }
}

pub fn handle_chat_input_focus_events(
    mut focus_events: EventReader<ChatFocusEvent>,
    mut unfocus_events: EventReader<ChatUnfocusEvent>,
    mut chat_input_query: Query<(
        &mut BackgroundColor,
        &mut chat_components::ChatMessageInputElement
    )>,
) {
    let (mut background_color, mut chat_container) = chat_input_query.single_mut();
    for _ in focus_events.read() {
        background_color.0 = COLOR_FOCUSED.clone();
        chat_container.focused = true;
    }

    for _ in unfocus_events.read() {
        background_color.0 = COLOR_UNFOCUSED.clone();
        chat_container.focused = false;
    }
}

pub fn handle_chat_input_system(
    mut evr_kbd: EventReader<KeyboardInput>,
    mut chat_input_query: Query<(
        &mut Text,
        &mut chat_components::ChatMessageInputElement
    )>,
    mut event_writer: EventWriter<chat_events::SendMessageEvent>

) {
    let (mut text, input_component) = chat_input_query.single_mut();

    if !input_component.focused {
        return;
    }

    let mut chat_input_value = match text.sections.first() {
        Some(text_section) => {
            text_section.value.clone()
        }
        None => {
            String::from("")
        }
    };

    for ev in evr_kbd.read() {

        if ev.state != chat_systems::ButtonState::Pressed {
            return
        }

        match &ev.logical_key {
            Key::Enter => {
                if !chat_input_value.trim().eq("")  {
                    event_writer.send(chat_events::SendMessageEvent(chat_input_value.trim().to_string()));
                    chat_input_value = String::from("");
                }
            }
            Key::Backspace => {
                chat_input_value.pop();
            }
            Key::Space => {
                chat_input_value += " ";
            }
            Key::Character(input) => {
                if input.chars().any(|c| c.is_control()) {
                    continue;
                }
                input.chars().for_each(|char| {
                    chat_input_value.push(char);
                });
            }
            _ => { }
        }
    }

    text.sections.clear();

    text.sections.push(TextSection {
        value: chat_input_value,
        ..Default::default()
    })

}

pub fn handle_events_system(
    mut commands: Commands,
    mut chat_sync_events: EventReader<chat_events::ChatSyncEvent>,
    query: Query<(Entity, &chat_components::ChatMessageContainer)>,
) {
    let (entity, _) = query.single();
    let events = chat_sync_events.read();

    for event in events {
        let messages = event.0.clone();
        let message = messages.last();

        if let Some(message) = message {
            commands.entity(entity).with_children(|parent| {
                parent.spawn(TextBundle {
                    text: Text {
                        sections: vec![TextSection {
                            value: message.format_string(),
                            style: TextStyle {
                                font_size: 20.0,
                                color: Color::WHITE,
                                ..Default::default()
                            },
                        }],
                        ..Default::default()
                    },
                    style: Style {
                        margin: UiRect::all(Val::Px(5.0)),
                        ..Default::default()
                    },
                    ..Default::default()
                });
            });
        }
    }
}

use chat_events::SendMessageEvent;

use crate::prelude::*;

const COLOR_UNFOCUSED: Color = Color::rgba(0.0, 0.0, 0.0, 0.0);
const COLOR_FOCUSED: Color = Color::rgba(0.0, 0.0, 0.0, 0.5);

pub fn setup_chat_container(mut commands: Commands) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(50.0),
                height: Val::Percent(80.0),
                flex_direction: FlexDirection::ColumnReverse,
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
        }).insert(chat_components::ChatMessageInputElement { enable_input: false });
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

pub fn handle_input_system(
    btn: Res<ButtonInput<MouseButton>>,
    key: Res<ButtonInput<KeyCode>>,
    mut window_query: Query<&mut Window>,
    mut controller_query: Query<&mut FpsController>,
    mut chat_query: Query<(
        &mut BackgroundColor,
        &mut chat_components::ChatMessageContainer,
    )>,
    mut chat_input_query: Query<(
        &mut Text,
        &mut chat_components::ChatMessageInputElement
    )>,
    mut event_writer: EventWriter<chat_events::SendMessageEvent>
) {
    let mut window = window_query.single_mut();

    let (mut chat_input_text, _) = chat_input_query.single_mut();
    let (mut chat_container_background, mut chat_container_component) = chat_query.single_mut();

    if btn.just_pressed(MouseButton::Left) {
        window.cursor.grab_mode = CursorGrabMode::Locked;
        window.cursor.visible = false;
        chat_container_component.focused = true;
        chat_container_background.0 = COLOR_UNFOCUSED;
    }
    if key.just_pressed(KeyCode::KeyX) {
        event_writer.send(chat_events::SendMessageEvent(String::from("Test Message")));
    }
    if key.just_pressed(KeyCode::KeyT) {
        window.cursor.grab_mode = CursorGrabMode::None;
        window.cursor.visible = true;

        chat_container_component.focused = true;
        chat_container_background.0 = COLOR_FOCUSED;

        for mut controller in &mut controller_query {
            controller.enable_input = false;
        }
    }
    if key.just_pressed(KeyCode::Escape) {
        chat_container_component.focused = false;
        chat_container_background.0 = COLOR_UNFOCUSED;
        window.cursor.grab_mode = CursorGrabMode::Locked;
        window.cursor.visible = false;

        for mut controller in &mut controller_query {
            controller.enable_input = true;
        }
    }
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

use crate::prelude::*;

pub fn setup_chat_container(
    mut commands: Commands,
) {
    commands.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(50.0), 
            height: Val::Percent(80.0),  
            flex_direction: FlexDirection::ColumnReverse,
            ..default()
        },
        background_color: BackgroundColor(Color::rgba(0.0,0.0,0.0, 0.0)),
        ..default()
    })
    .insert(chat_components::ChatMessageContainer{ enable_input: false }) 
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text {
                    sections: vec![
                        TextSection {
                            value: "Welcome to the chat!".to_string(),
                            style: TextStyle {
                                font_size: 20.0,
                                color: Color::WHITE,
                                ..Default::default()
                            },
                        },
                    ],
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

pub fn send_message_system(
    mut client: ResMut<RenetClient>,
) {
    let message = String::from("Hey there!");

    client.send_message(
        DefaultChannel::ReliableOrdered,
        bincode::serialize(&NetworkingMessage::ChatMessageSend(message)).unwrap()
    )
}

pub fn handle_input_system(
    btn: Res<ButtonInput<MouseButton>>,
    key: Res<ButtonInput<KeyCode>>,
    mut window_query: Query<&mut Window>,
    mut controller_query: Query<&mut FpsController>,
    mut chat_query: Query<(&mut BackgroundColor, &mut chat_components::ChatMessageContainer)>
) {
    let mut window = window_query.single_mut();
    if btn.just_pressed(MouseButton::Left) {
        window.cursor.grab_mode = CursorGrabMode::Locked;
        window.cursor.visible = false;
        for (mut background_color, mut chat_component) in &mut chat_query {
            chat_component.enable_input = true;
            background_color.0 = Color::rgba(0.0, 0.0, 0.0, 0.0)
        }
    }
    if key.just_pressed(KeyCode::KeyT) {
        window.cursor.grab_mode = CursorGrabMode::None;
        window.cursor.visible = true;
        for (mut background_color, mut chat_component) in &mut chat_query {
            chat_component.enable_input = true;
            background_color.0 = Color::rgba(255.0,0.0,0.0, 0.2);
        }
        for mut controller in &mut controller_query {
            controller.enable_input = false;
        }
    }
    if key.just_pressed(KeyCode::Escape) {
        for (mut background_color, mut chat_component) in &mut chat_query {
            chat_component.enable_input = false;
            background_color.0 = Color::rgba(0.0, 0.0, 0.0, 0.0);
            window.cursor.grab_mode = CursorGrabMode::Locked;
            window.cursor.visible = false;
            for mut controller in &mut controller_query {
                controller.enable_input = true;
            }
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

        match message {
            Some(message) => {
                commands.entity(entity).with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![
                                TextSection {
                                    value: message.format_string(),
                                    style: TextStyle {
                                        font_size: 20.0,
                                        color: Color::WHITE,
                                        ..Default::default()
                                    },
                                },
                            ],
                            ..Default::default()
                        },
                        style: Style {
                            margin: UiRect::all(Val::Px(5.0)),
                            ..Default::default()
                        },
                        ..Default::default()
                    });
                });

            },
            None => { }
        }
    }
}

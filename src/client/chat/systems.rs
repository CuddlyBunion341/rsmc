use crate::prelude::*;

pub fn setup_chat_container(
    mut commands: Commands,
) {
    commands.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0), 
            height: Val::Percent(80.0),  
            flex_direction: FlexDirection::ColumnReverse,
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(chat_components::ChatMessageContainer()) 
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

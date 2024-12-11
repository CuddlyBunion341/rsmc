use bevy::input::ButtonState;
use chat_events::{FocusChangeEvent, FocusState, SendMessageEvent};

use crate::prelude::*;

const COLOR_UNFOCUSED: Color = Color::rgba(0.0, 0.0, 0.0, 0.0);
const COLOR_FOCUSED: Color = Color::rgba(0.0, 0.0, 0.0, 0.5);
const TEXT_STYLE: TextStyle = TextStyle {
    font_size: 20.0,
    color: Color::rgba(1.0, 1.0, 1.0, 0.1),
    ..default()
};
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
                style: TEXT_STYLE,
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
                        flex_direction: FlexDirection::Column,
                        ..default()
                    },
            ))
            .insert(chat_components::ChatMessageContainer { focused: false });

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
        });
}

pub fn handle_focus_events(
    mut focus_change_events: EventReader<FocusChangeEvent>,
    mut chat_container_query: Query<(
        &mut BackgroundColor,
        &mut chat_components::ChatMessageContainer,
    )>,
    mut chat_input_query: Query<(
        &mut BackgroundColor,
        &mut chat_components::ChatMessageInputElement,
    )>,
    mut controller_query: Query<&mut FpsController>,
    mut window_query: Query<&mut Window>,
) {
    let mut window = window_query.single_mut();
    let (mut container_bg, mut chat_container) = chat_container_query.single_mut();
    let (mut input_bg, mut chat_input) = chat_input_query.single_mut();

    for event in focus_change_events.read() {
        match event.state {
            FocusState::Focus => {
                container_bg.0 = COLOR_FOCUSED;
                chat_container.focused = true;
                input_bg.0 = COLOR_FOCUSED;
                chat_input.focused = true;
                for mut controller in &mut controller_query {
                    controller.enable_input = false;
                }
            }
            FocusState::Unfocus => {
                container_bg.0 = COLOR_UNFOCUSED;
                chat_container.focused = false;
                input_bg.0 = COLOR_UNFOCUSED;
                chat_input.focused = false;
                for mut controller in &mut controller_query {
                    controller.enable_input = true;
                }
                window.cursor.grab_mode = CursorGrabMode::Locked;
                window.cursor.visible = false;
            }
        }
    }
}

pub fn handle_chat_input_system(
    mut evr_kbd: EventReader<KeyboardInput>,
    mut chat_input_query: Query<(&mut Text, &mut chat_components::ChatMessageInputElement)>,
    mut send_event_writer: EventWriter<SendMessageEvent>,
) {
    let (mut text, input_component) = chat_input_query.single_mut();
    if !input_component.focused {
        return;
    }

    let mut chat_input_value = text.sections.first().map_or(String::new(), |s| s.value.clone());

    for ev in evr_kbd.read() {
        if ev.state != ButtonState::Pressed {
            continue;
        }
        match &ev.logical_key {
            Key::Enter if !chat_input_value.trim().is_empty() => {
                send_event_writer.send(SendMessageEvent(chat_input_value.trim().to_string()));
                chat_input_value.clear();
            }
            Key::Backspace => {
                chat_input_value.pop();
            }
            Key::Space => chat_input_value.push(' '),
            Key::Character(input) if input.chars().all(|c| !c.is_control()) => {
                chat_input_value.push_str(input);
            }
            _ => {}
        }
    }

    text.sections.clear();
    text.sections.push(TextSection {
        value: chat_input_value,
        ..default()
    });
}

pub fn add_message_to_chat_container_system(
    mut commands: Commands,
    query: Query<(Entity, &chat_components::ChatMessageContainer)>,
    mut events: EventReader<chat_events::SingleChatSendEvent>,
) {
    let (entity, _) = query.single();
    for event in events.read() {
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

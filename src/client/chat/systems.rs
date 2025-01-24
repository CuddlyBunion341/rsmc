use crate::prelude::*;
use bevy::input::{keyboard::KeyboardInput, ButtonState};
use bevy_flair::style::components::{ClassList, NodeStyleSheet};
use chat_events::{ChatFocusStateChangeEvent, ChatMessageSendEvent, FocusState};

const MESSAGE_PROMPT: &str = "> ";

pub fn setup_chat_container(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            Node::default(),
            Name::new("root"),
            NodeStyleSheet::new(asset_server.load("chat.css")),
        ))
        .with_children(|parent| {
            parent.spawn((
                Node::default(),
                Name::new("chat_message_container"),
                ClassList::new(),
                chat_components::ChatMessageContainer { focused: false },
                Text::new(""),
            ));

            parent.spawn((
                Node::default(),
                Name::new("chat_message_input"),
                ClassList::new(),
                chat_components::ChatMessageInputElement { focused: false },
                Text::new(MESSAGE_PROMPT),
            ));
        });
}

pub fn handle_focus_events(
    mut focus_change_events: EventReader<ChatFocusStateChangeEvent>,
    mut chat_container_query: Query<
        (&mut ClassList, &mut chat_components::ChatMessageContainer),
        Without<chat_components::ChatMessageInputElement>,
    >,
    mut chat_input_query: Query<
        (
            &mut ClassList,
            &mut chat_components::ChatMessageInputElement,
        ),
        Without<chat_components::ChatMessageContainer>,
    >,
    mut controller_query: Query<&mut FpsController>,
    mut window_query: Query<&mut Window>,
) {
    if let Ok(mut window) = window_query.get_single_mut() {
        if let (
            Ok((mut container_classes, mut chat_container)),
            Ok((mut input_classes, mut chat_input)),
        ) = (
            chat_container_query.get_single_mut(),
            chat_input_query.get_single_mut(),
        ) {
            for event in focus_change_events.read() {
                match event.state {
                    FocusState::Focus => {
                        info!("Handling focus state");
                        container_classes.add_class("focused");
                        chat_container.focused = true;
                        input_classes.add_class("focused");
                        chat_input.focused = true;
                        for mut controller in &mut controller_query.iter_mut() {
                            controller.enable_input = false;
                        }
                    }
                    FocusState::Unfocus => {
                        info!("Handling unfocus state");
                        container_classes.remove_class("focused");
                        chat_container.focused = false;
                        input_classes.remove_class("focused");
                        chat_input.focused = false;
                        for mut controller in &mut controller_query.iter_mut() {
                            controller.enable_input = true;
                        }
                        window.cursor_options.grab_mode = CursorGrabMode::Locked;
                        window.cursor_options.visible = false;
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
                    window.cursor_options.grab_mode = CursorGrabMode::Locked;
                    window.cursor_options.visible = false;
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

        let enable_input = match event.state {
            FocusState::Focus => false,
            FocusState::Unfocus => true,
        };

        for mut controller in &mut controller_query.iter_mut() {
            controller.enable_input = enable_input;
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

        let mut chat_input_value = text.0.clone();

        for event in evr_kbd.read() {
            if event.state != ButtonState::Pressed {
                continue;
            }

            if chat_state.just_focused {
                chat_state.just_focused = false;
                continue;
            }

            info!("Chat state: {}", chat_input_value);

            let mut message = extract_message(&chat_input_value);

            match &event.logical_key {
                Key::Enter if !message.trim().is_empty() => {
                    send_event_writer.send(ChatMessageSendEvent(message.trim().to_string()));
                    message.clear();
                }
                Key::Backspace => {
                    message.pop();
                }
                Key::Space => message.push(' '),
                Key::Character(input) => {
                    if input.chars().all(|c| !c.is_control()) {
                        message.push_str(input);
                    }
                }
                _ => {}
            }

            chat_input_value = MESSAGE_PROMPT.to_string() + &message;
        }

        text.clear();

        text.0 += &chat_input_value;
    }
}

fn extract_message(value: &str) -> String {
    let message = value.trim_start_matches(MESSAGE_PROMPT);
    message.to_string()
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
    mut query: Query<(
        Entity,
        &chat_components::ChatMessageContainer,
        &mut ScrollPosition,
    )>,
    mut events: EventReader<chat_events::SingleChatSendEvent>,
) {
    for event in events.read() {
        if let Ok((entity, _, mut scroll_position)) = query.get_single_mut() {
            // Offset does not need to be exact, just needs to be large enough to see the new message
            scroll_position.offset_y += 100.0;

            commands.entity(entity).with_children(|parent| {
                parent.spawn((
                    Node::default(),
                    Name::new("chat_entry"),
                    Text::new(event.0.message.clone()),
                ));
            });
        }
    }
}

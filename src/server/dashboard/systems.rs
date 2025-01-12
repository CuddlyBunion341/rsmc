use bevy_tui::prelude::*;

use crate::prelude::*;
use ratatui::layout::{Alignment, Constraint, Layout};
use ratatui::prelude::Direction;
use ratatui::style::Style;
use ratatui::text::Span;
use ratatui::widgets::{Paragraph, Wrap};
use ratatui::Frame;

use bevy::app::AppExit;

// This lint doesn't like values passed in but not consumed which is fair. Bevy requires the
// `Res<_>` type to be passed by value so we unfortunately have to disable this lint wherever a
// `Res<_>` is used but not consumed.
#[allow(clippy::needless_pass_by_value)]
pub fn quit_system(key_code: Res<ButtonInput<KeyCode>>, mut event_writer: EventWriter<AppExit>) {
    if key_code.just_pressed(KeyCode::KeyQ) {
        event_writer.send(AppExit);
    }
}

fn render_ui(
    f: &mut Frame,
    keyboard: &ButtonInput<KeyCode>,
    mouse: &ButtonInput<MouseButton>,
    mouse_state: &MouseState,
) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Min(0),
            ]
            .as_ref(),
        )
        .split(f.size());

    let hello_content = Span::styled("Hello Bevy! Press 'q' to quit.", Style::default());
    let hello_paragraph = Paragraph::new(hello_content)
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });

    f.render_widget(hello_paragraph, chunks[0]);

    let keyboard_content = Span::styled(format!("Keyboard: {keyboard:?}"), Style::default());
    let keyboard_paragraph = Paragraph::new(keyboard_content)
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });

    f.render_widget(keyboard_paragraph, chunks[1]);

    let mouse_content = Span::styled(format!("Mouse Buttons: {mouse:?}"), Style::default());
    let mouse_paragraph = Paragraph::new(mouse_content)
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });

    f.render_widget(mouse_paragraph, chunks[2]);

    let mouse_state_content =
        Span::styled(format!("Mouse State: {mouse_state:?}"), Style::default());
    let mouse_state_paragraph = Paragraph::new(mouse_state_content)
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });

    f.render_widget(mouse_state_paragraph, chunks[3]);
}

// This lint doesn't like values passed in but not consumed which is fair. Bevy requires the
// `Res<_>` type to be passed by value so we unfortunately have to disable this lint wherever a
// `Res<_>` is used but not consumed.
#[allow(clippy::needless_pass_by_value)]
pub fn run_basic_ui(
    mut terminal: ResMut<bevy_tui::BevyTerminal>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mouse_state: Res<MouseState>,
) {
    terminal
        .0
        .draw(|f| render_ui(f, &keyboard, &mouse, &mouse_state))
        .expect("failed to draw to terminal");
}

use crate::prelude::*;
use ratatui::layout::{Alignment, Constraint, Layout};
use ratatui::prelude::Direction;
use ratatui::style::Style;
use ratatui::text::Span;
use ratatui::widgets::{Paragraph, Wrap};
use ratatui::Frame;

use bevy::app::AppExit;

pub fn quit_system(key_code: Res<ButtonInput<KeyCode>>, mut event_writer: EventWriter<AppExit>) {
    if key_code.just_pressed(KeyCode::KeyQ) {
        event_writer.send(AppExit);
    }
}

fn render_ui(f: &mut Frame, keyboard: &ButtonInput<KeyCode>) {
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

    let logo_content = r"   
▗▄▄▖  ▗▄▄▖▗▖  ▗▖ ▗▄▄▖    ▗▄▄▖ ▗▄▄▖ ▗▄▄▄▖ ▗▄▖ ▗▖   ▗▄▄▖ ▗▖ ▗▖ ▗▄▖ 
▐▌ ▐▌▐▌   ▐▛▚▞▜▌▐▌       ▐▌ ▐▌▐▌ ▐▌▐▌   ▐▌ ▐▌▐▌   ▐▌ ▐▌▐▌ ▐▌▐▌ ▐▌
▐▛▀▚▖ ▝▀▚▖▐▌  ▐▌▐▌       ▐▛▀▘ ▐▛▀▚▖▐▛▀▀▘▐▛▀▜▌▐▌   ▐▛▀▘ ▐▛▀▜▌▐▛▀▜▌
▐▌ ▐▌▗▄▄▞▘▐▌  ▐▌▝▚▄▄▖    ▐▌   ▐▌ ▐▌▐▙▄▄▖▐▌ ▐▌▐▙▄▄▖▐▌   ▐▌ ▐▌▐▌ ▐▌
                                                                           ";

    let logo_paragraph = Paragraph::new(logo_content)
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });

    f.render_widget(logo_paragraph, chunks[0]);

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
}

pub fn run_basic_ui(
    mut terminal: ResMut<bevy_tui::BevyTerminal>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    terminal
        .0
        .draw(|f| render_ui(f, &keyboard))
        .expect("failed to draw to terminal");
}

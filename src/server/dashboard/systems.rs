use crate::prelude::*;
use ratatui::layout::{Alignment, Constraint, Flex, Layout};
use ratatui::prelude::Direction;
use ratatui::style::{Modifier, Style};
use ratatui::text::{self, Line, Span};
use ratatui::widgets::{Borders, Paragraph, Wrap};
use ratatui::Frame;

use bevy::app::AppExit;

pub fn quit_system(key_code: Res<ButtonInput<KeyCode>>, mut event_writer: EventWriter<AppExit>) {
    if key_code.just_pressed(KeyCode::KeyQ) {
        event_writer.send(AppExit);
    }
}

fn render_ui(f: &mut ratatui::Frame) {
    let chunks = Layout::vertical([Constraint::Length(2), Constraint::Min(0)]).split(f.size());

    let header_chunk = chunks[0];

    let border_block = ratatui::widgets::Block::bordered().borders(Borders::BOTTOM);

    let header_chunks = Layout::horizontal([Constraint::Min(0), Constraint::Min(0)])
        .flex(Flex::SpaceBetween)
        .split(header_chunk);

    let left = header_chunks[0];
    let right = header_chunks[1];

    let logo = Paragraph::new(ratatui::text::Line::from("RSMC Pre Alpha")).block(border_block.clone());
    let exit_text = Paragraph::new(ratatui::text::Line::from("Press 'q' to quit.")).block(border_block.clone());

    f.render_widget(logo, left);
    f.render_widget(exit_text, right);
}

pub fn run_basic_ui(mut terminal: ResMut<bevy_tui::BevyTerminal>) {
    terminal
        .0
        .draw(|f| render_ui(f))
        .expect("failed to draw to terminal");
}

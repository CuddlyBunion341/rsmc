use crate::prelude::*;
use dashboard_events::LogEvent;
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

fn render_ui(f: &mut ratatui::Frame, player_states: Res<player_resources::PlayerStates>) {
    let chunks = Layout::vertical([Constraint::Length(2), Constraint::Min(0)]).split(f.size());

    let header_chunk = chunks[0];

    let border_block = ratatui::widgets::Block::bordered().borders(Borders::BOTTOM);

    let header_chunks = Layout::horizontal([Constraint::Min(0), Constraint::Min(0)])
        .flex(Flex::SpaceBetween)
        .split(header_chunk);

    let left = header_chunks[0];
    let right = header_chunks[1];

    let logo =
        Paragraph::new(ratatui::text::Line::from("RSMC Pre Alpha")).block(border_block.clone());
    let exit_text =
        Paragraph::new(ratatui::text::Line::from("Press 'q' to quit.")).block(border_block.clone());

    f.render_widget(logo, left);
    f.render_widget(exit_text, right);

    let player_block = ratatui::widgets::Block::default()
        .borders(Borders::ALL)
        .title("Players");

    let player_chunks = Layout::vertical([Constraint::Length(10), Constraint::Min(0)]);

    for (player_id, player_state) in player_states.players.iter() {
        let player_chunk = player_chunks.split(chunks[1]);
        let player_chunks = Layout::horizontal([Constraint::Length(10), Constraint::Min(0)]);
        let left = player_chunks.split(player_chunk[0])[0];
        let right = player_chunks.split(player_chunk[1])[1];

        let player_name = Span::styled(
            format!("{}", player_id),
            Style::default().fg(ratatui::style::Color::Yellow),
        );
        let player_status = Span::styled(
            format!("{}", player_state.position),
            Style::default().fg(ratatui::style::Color::Green),
        );

        let player_text = Paragraph::new(Line::from(player_name))
            .style(Style::default().fg(ratatui::style::Color::Yellow))
            .block(border_block.clone())
            .alignment(Alignment::Left);
        let player_status_text = Paragraph::new(Line::from(player_status))
            .style(Style::default().fg(ratatui::style::Color::Green))
            .block(border_block.clone())
            .alignment(Alignment::Left);

        f.render_widget(player_text, left);
        f.render_widget(player_status_text, right);
    }
}

pub fn run_basic_ui(mut terminal: ResMut<bevy_tui::BevyTerminal>, player_states: Res<player_resources::PlayerStates>) {
    terminal
        .0
        .draw(|f| render_ui(f, player_states))
        .expect("failed to draw to terminal");
}

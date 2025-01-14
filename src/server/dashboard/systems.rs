use crate::prelude::*;
use bevy::core_pipeline::tonemapping::get_lut_bind_group_layout_entries;
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

    let player_chunks = Layout::vertical([Constraint::Length(10), Constraint::Min(0)]);

    let paragraphs = get_formatted_player_text(player_states);
    paragraphs.into_iter().for_each(|p| {
        let p = p.clone().block(
            ratatui::widgets::Block::default()
            .borders(Borders::ALL)
            .title("Players")

        );
        f.render_widget(p, player_chunks.split(chunks[1])[0]);
    });
}

fn get_formatted_player_text(player_states: Res<player_resources::PlayerStates>) -> Vec<Paragraph> {
    let text = get_player_text(player_states);

    text.into_iter().map(|line_content| {
        let line = Line::from(line_content);
        Paragraph::new(line)
    }).collect()
}

fn get_player_text(player_states: Res<player_resources::PlayerStates>) -> Vec<String> {
    if player_states.players.len() == 0 {
        vec![String::from("Waiting for players...")]
    } else {
        player_states.players.iter().map({|player| {
            let (client_id, state) = player;

            let position = state.position;
            let rotation = state.rotation;

            let val: String = format!("{}: {}/{}", client_id, position, rotation);

            val
        }}).collect()
    }
}

pub fn run_basic_ui(mut terminal: ResMut<bevy_tui::BevyTerminal>, player_states: Res<player_resources::PlayerStates>) {
    terminal
        .0
        .draw(|f| render_ui(f, player_states))
        .expect("failed to draw to terminal");
}

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

pub fn run_basic_ui(mut terminal: ResMut<bevy_tui::BevyTerminal>, chunk_manager: Res<terrain_resources::ChunkManager>, player_states: Res<player_resources::PlayerStates>) {
    terminal
        .0
        .draw(|f| render_ui(f, chunk_manager, player_states))
        .expect("failed to draw to terminal");
}

fn render_ui(frame: &mut ratatui::Frame, chunk_manager: Res<terrain_resources::ChunkManager>, player_states: Res<player_resources::PlayerStates>) {
    let chunks = Layout::vertical([Constraint::Length(2), Constraint::Length(5), Constraint::Min(0)]).split(frame.size());

    render_header(frame, chunks[0]);
    render_world_stats(frame, chunks[1], chunk_manager);
    render_players(frame, chunks[2], player_states);
}

fn render_header(frame: &mut Frame, header_chunk: ratatui::prelude::Rect) {
    let header_chunk = header_chunk;

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

    frame.render_widget(logo, left);
    frame.render_widget(exit_text, right);

}

fn render_world_stats(frame: &mut ratatui::Frame, slot: ratatui::prelude::Rect, chunk_manager: Res<terrain_resources::ChunkManager>) {
    let block = ratatui::widgets::Block::default().borders(Borders::ALL).title("World");

    let chunk_count = chunk_manager.chunks.keys().len();

    let paragraph = Paragraph::new(Line::from(format!("Chunk count: {}", chunk_count))).block(block);

    frame.render_widget(paragraph, slot);
}

fn render_players(frame: &mut Frame, chunk: ratatui::prelude::Rect, player_states: Res<player_resources::PlayerStates>) {
    let player_chunks = Layout::vertical([Constraint::Length(10), Constraint::Min(0)]);

    let paragraphs = get_formatted_player_text(player_states);
    paragraphs.into_iter().for_each(|paragraph| {
        let paragraph = paragraph.clone().block(
            ratatui::widgets::Block::default()
            .borders(Borders::ALL)
            .title("Players")

        );
        frame.render_widget(paragraph, player_chunks.split(chunk)[0]);
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

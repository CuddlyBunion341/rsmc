use bevy_flair::style::components::NodeStyleSheet;

use crate::prelude::*;

pub fn setup_gui_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Camera2d,
        Camera {
            order: 2,
            ..default()
        },
    ));
    commands
        .spawn((
            Node::default(),
            Name::new("menu_title_wrapper"),
            NodeStyleSheet::new(asset_server.load("gui.css")),
        ))
        .with_children(|parent| {
            parent.spawn((Text::new("RSMC - Pre Alpha"), Name::new("menu_title")));
        });
}

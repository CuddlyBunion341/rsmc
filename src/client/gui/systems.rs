use crate::prelude::*;

use bevy::{prelude::*, winit::WinitSettings};

pub fn setup_gui_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands
        .spawn(Node {
            // width: Val::Percent(100.0),
            // height: Val::Percent(100.0),
            // align_items: AlignItems::Center,
            // justify_content: JustifyContent::Center,
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn((
                    Button,
                    Node {
                        // width: Val::Px(150.0),
                        // height: Val::Px(65.0),
                        // border: UiRect::all(Val::Px(5.0)),
                        // // horizontally center child text
                        // justify_content: JustifyContent::Center,
                        // // vertically center child text
                        // align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    BorderColor(Color::BLACK),
                    // BorderRadius::MAX,
                    // BackgroundColor(NORMAL_BUTTON),
                ))
                .with_children(|parent| {
                    parent.spawn(Text::from_section(
                        "Click me!",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 30.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    ));
                    // Text::from_section("Hello World!", TextStyle { ..default() })
                    // TextFont {
                    //     font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    //     font_size: 33.0,
                    //     ..default()
                    // },
                    // TextColor(Color::srgb(0.9, 0.9, 0.9)),
                });
        });
}

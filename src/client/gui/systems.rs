use crate::prelude::*;

pub fn setup_gui_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle {
        camera: Camera {
            order: 2,
            ..default()
        },
        ..default()
    });
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::FlexStart,
                justify_content: JustifyContent::FlexStart,
                padding: UiRect {
                    left: Val::Px(5.0),
                    right: Val::Px(5.0),
                    top: Val::Px(5.0),
                    bottom: Val::Px(5.0),
                },
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "RSMC - Pre Alpha",
                    TextStyle {
                        font: asset_server.load("fonts/Terminus500.ttf"),
                        font_size: 60.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                ),
                ..Default::default()
            });
        });
}

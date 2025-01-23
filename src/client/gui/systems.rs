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
        .spawn(Node {
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
        })
        .with_children(|parent| {
            parent.spawn((
                Text::new("RSMC - Pre Alpha"),
                TextFont {
                    font: asset_server.load("fonts/Terminus500.ttf"),
                    font_size: 60.0,
                    ..Default::default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
            ));
        });
}

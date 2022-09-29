use bevy::prelude::*;

use super::text_effects;

#[allow(clippy::needless_pass_by_value)] // bevy requires Res to be passed by value
pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(
            TextBundle::from_section(
                // Accepts a `String` or any type that converts into a `String`, such as `&str`
                "press any key to start",
                TextStyle {
                    font_size: 100.0,
                    color: Color::WHITE,
                    font: asset_server.load("fonts/InterVariable.ttf"),
                },
            ) // Set the alignment of the Text
            .with_text_alignment(TextAlignment::TOP_CENTER)
            // Set the style of the TextBundle itself.
            .with_style(Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: UiRect {
                    bottom: Val::Px(5.0),
                    right: Val::Px(15.0),
                    ..default()
                },
                ..default()
            }),
        )
        .insert(text_effects::ChangingColors);
}

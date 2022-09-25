use bevy::prelude::*;

use super::text_effects;

#[derive(Component, Debug)]
pub struct StartScreen;

pub fn setup(mut commands: Commands) {
    commands
        .spawn_bundle(
            TextBundle::from_section(
                // Accepts a `String` or any type that converts into a `String`, such as `&str`
                "press any key to start",
                TextStyle {
                    font_size: 100.0,
                    color: Color::WHITE,
                    ..Default::default()
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

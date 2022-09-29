use bevy::prelude::*;

#[derive(Component)]
pub struct Score {
    value: i32,
}

impl Score {
    pub fn new(value: i32) -> Self {
        Score { value }
    }
}

#[allow(clippy::needless_pass_by_value)] // bevy requires Res to be passed by value
pub fn setup(mut score: ResMut<Score>) {
    score.as_mut().value = 0;
}

#[derive(Component)]
/// Event used to indicate that the score should be increased
pub struct Change {
    value: i32,
}

#[allow(clippy::needless_pass_by_value)] // bevy requires Res to be passed by value
pub fn update(mut events: EventReader<Change>, mut score: ResMut<Score>) {
    for event in events.iter() {
        score.value += event.value;
    }
}

#[allow(clippy::needless_pass_by_value)] // bevy requires Res to be passed by value
pub fn spawn_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(
        TextBundle::from_section(
            // Accepts a `String` or any type that converts into a `String`, such as `&str`
            "SCORE",
            TextStyle {
                font_size: 50.0,
                color: Color::WHITE,
                font: asset_server.load("fonts/InterVariable.ttf"),
            },
        ) // Set the alignment of the Text
        .with_text_alignment(TextAlignment::TOP_LEFT)
        // Set the style of the TextBundle itself.
        .with_style(Style {
            align_self: AlignSelf::FlexStart,
            ..default()
        }),
    );
}

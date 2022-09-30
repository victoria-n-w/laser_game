use bevy::prelude::*;

use crate::states;

#[derive(Component)]
pub struct Score {
    value: i32,
}

impl Score {
    pub const fn new(value: i32) -> Self {
        Self { value }
    }
}

#[derive(Component)]
struct ScoreText;

#[allow(clippy::needless_pass_by_value)] // bevy requires Res to be passed by value
pub fn setup(mut score: ResMut<Score>) {
    score.as_mut().value = 0;
}

#[derive(Component)]
/// Event used to indicate that the score should be increased
pub struct Change {
    pub value: i32,
}

#[allow(clippy::needless_pass_by_value)] // bevy requires Res to be passed by value
fn update(
    mut events: EventReader<Change>,
    mut score: ResMut<Score>,
    mut query: Query<&mut Text, With<ScoreText>>,
) {
    for event in events.iter() {
        score.value += event.value;
    }
    for mut text in &mut query {
        text.sections[0].value = score.value.to_string();
    }
}

#[allow(clippy::needless_pass_by_value)] // bevy requires Res to be passed by value
fn spawn_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(
            TextBundle::from_section(
                // Accepts a `String` or any type that converts into a `String`, such as `&str`
                "0",
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
        )
        .insert(ScoreText);
}

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Score::new(0))
            .add_event::<Change>()
            .add_system_set(SystemSet::on_enter(states::AppState::Title).with_system(setup))
            .add_system_set(SystemSet::on_enter(states::AppState::Game).with_system(spawn_ui))
            .add_system_set(SystemSet::on_update(states::AppState::Game).with_system(update));
    }
}

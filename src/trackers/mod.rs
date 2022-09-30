use bevy::prelude::*;

mod health;
pub mod score;

pub struct TrackersPlugin;

impl Plugin for TrackersPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(score::ScorePlugin)
            .add_plugin(health::HealthPlugin);
    }
}

use bevy::prelude::*;

use crate::states;

mod health;
pub mod score;

pub struct TrackersPlugin;

impl Plugin for TrackersPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(score::Score::new(0))
            .add_event::<score::Change>()
            .add_system_set(SystemSet::on_enter(states::AppState::Title).with_system(score::setup))
            .add_system_set(
                SystemSet::on_enter(states::AppState::Game).with_system(score::spawn_ui),
            )
            .add_system_set(
                SystemSet::on_update(states::AppState::Game).with_system(score::update),
            );
    }
}

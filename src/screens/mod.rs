use bevy::prelude::SystemSet;

use crate::{states, util};

mod common;
mod game_over;
mod start;
mod text_effects;

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system_set(SystemSet::on_enter(states::AppState::Title).with_system(start::setup))
            .add_system_set(
                SystemSet::on_update(states::AppState::Title).with_system(text_effects::color),
            )
            .add_system_set(
                SystemSet::on_exit(states::AppState::Title)
                    .with_system(util::commands::despawn_all),
            )
            .add_system_set(
                SystemSet::on_enter(states::AppState::GameOver).with_system(game_over::setup),
            )
            .add_system_set(
                SystemSet::on_update(states::AppState::GameOver).with_system(text_effects::color),
            )
            .add_system_set(
                SystemSet::on_exit(states::AppState::GameOver)
                    .with_system(util::commands::despawn_all),
            );
    }
}

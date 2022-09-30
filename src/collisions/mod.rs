use bevy::prelude::*;

mod with_enemies;
mod with_etities;
mod with_pickups;
mod with_walls;

use crate::{enemies, math, pickups, states};

#[allow(clippy::module_name_repetitions)] // it's used only once and other name would be ambigous
pub struct CollisionsPlugin;

#[derive(Component, Default)]
pub struct Collideable;

impl bevy::prelude::Plugin for CollisionsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<with_enemies::Collision>()
            .add_event::<with_pickups::PickedUp>()
            .add_system_set(
                SystemSet::on_update(states::AppState::Game)
                    .with_system(with_walls::system)
                    .with_system(
                        with_etities::collisions::<
                            math::distance::Manhatann,
                            enemies::entity::Enemy,
                            with_enemies::Collision,
                        >,
                    )
                    .with_system(with_enemies::on_collision.after(
                        with_etities::collisions::<
                            math::distance::Manhatann,
                            enemies::entity::Enemy,
                            with_enemies::Collision,
                        >,
                    ))
                    .with_system(
                        with_etities::collisions::<
                            math::distance::Manhatann,
                            pickups::ScorePickup,
                            with_pickups::PickedUp,
                        >,
                    )
                    .with_system(with_pickups::picked_up.after(
                        with_etities::collisions::<
                            math::distance::Manhatann,
                            pickups::ScorePickup,
                            with_pickups::PickedUp,
                        >,
                    )),
            );
    }
}

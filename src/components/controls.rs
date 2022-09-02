use bevy::prelude::Component;

pub enum Turn {
    Anticlockwise,
    Clockwise,
}

#[derive(Component)]
pub struct Controls {
    turn: Turn,
}

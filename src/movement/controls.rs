use bevy::prelude::Component;

pub enum Turn {
    Anticlockwise,
    Idle,
    Clockwise,
}

pub enum Drive {
    Forwards,
    Backward,
    Idle,
}
#[derive(Component)]
pub struct Controls {
    pub turn: Turn,
    pub drive: Drive,
}

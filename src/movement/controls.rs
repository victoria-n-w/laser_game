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
#[derive(Component, Default)]
pub struct Controls {
    pub turn: Turn,
    pub drive: Drive,
}

impl Default for Turn {
    fn default() -> Self {
        Turn::Idle
    }
}

impl Default for Drive {
    fn default() -> Self {
        Drive::Idle
    }
}

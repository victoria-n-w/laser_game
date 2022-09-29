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

pub fn setup(mut score: ResMut<Score>) {
    score.as_mut().value = 0;
}

#[derive(Component)]
/// Event used to indicate that the score should be increased
pub struct Change {
    value: i32,
}

pub fn update(mut events: EventReader<Change>, mut score: ResMut<Score>) {
    for event in events.iter() {
        score.value += event.value;
    }
}

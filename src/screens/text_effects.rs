use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct ChangingColors;

#[allow(clippy::needless_pass_by_value)] // bevy requires Res to be passed by value
pub fn color(time: Res<Time>, mut query: Query<&mut Text, With<ChangingColors>>) {
    for mut text in &mut query {
        #[allow(clippy::cast_possible_truncation)] // casting floats is more or less fine in rust
        let seconds = time.seconds_since_startup() as f32;

        // Update the color of the first and only section.
        text.sections[0].style.color = Color::Rgba {
            red: (1.25 * seconds).sin() / 2.0 + 0.5,
            green: (0.75 * seconds).sin() / 2.0 + 0.5,
            blue: (0.50 * seconds).sin() / 2.0 + 0.5,
            alpha: 1.0,
        };
    }
}

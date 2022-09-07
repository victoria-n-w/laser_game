#[derive(bevy::prelude::Component)]
pub struct ArenaSize {
    pub min_x: f32,
    pub min_y: f32,
    pub max_x: f32,
    pub max_y: f32,
    pub width: f32,
    pub height: f32,
}

impl ArenaSize {
    pub fn new(width: f32, height: f32) -> Self {
        ArenaSize {
            min_x: -height / 2_f32,
            min_y: -width / 2_f32,
            max_x: height / 2_f32,
            max_y: width / 2_f32,
            width,
            height,
        }
    }
}

pub struct Plugin {
    pub x: f32,
    pub y: f32,
}

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(ArenaSize::new(self.y, self.x));
    }
}

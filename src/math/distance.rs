use bevy::prelude::Vec3;

pub trait Metric {
    fn measure(x: &Vec3, y: &Vec3) -> f32;
}

pub struct Manhatann;

impl Metric for Manhatann {
    fn measure(x: &Vec3, y: &Vec3) -> f32 {
        let x_d = (x.x - y.x).abs();
        let y_d = (x.y - y.y).abs();
        let z_d = (x.z - y.z).abs();

        x_d + y_d + z_d
    }
}

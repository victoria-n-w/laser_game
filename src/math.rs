use bevy::prelude::Transform;

pub trait FrameChanger {
    fn local(&self, other: Self) -> Self;
}

impl FrameChanger for Transform {
    fn local(&self, mut other: Self) -> Self {
        let inverse_rotation = self.rotation.inverse();
        other.translation -= self.translation;
        other.rotation *= inverse_rotation;
        other.translation = inverse_rotation * other.translation;
        other
    }
}

#[cfg(test)]
mod tests {

    use bevy::prelude::{Quat, Transform, Vec3};

    use super::FrameChanger;

    #[test]
    fn basic_frame_change() {
        let entity_in_world = Transform::from_translation(Vec3::new(0.0, -10.0, 0.0))
            .with_rotation(Quat::from_rotation_z(0.0));

        let frame = Transform::from_translation(Vec3::new(0.0, -5.0, 0.0))
            .with_rotation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_2));

        let entity_in_frame = frame.local(entity_in_world);

        assert!(entity_in_frame
            .translation
            .abs_diff_eq(Vec3::new(-5.0, 0.0, 0.0), 1e-3));

        assert!(entity_in_frame
            .rotation
            .abs_diff_eq(Quat::from_rotation_z(-std::f32::consts::FRAC_PI_2), 1e-3));

        let entity_2_in_world = Transform::from_translation(Vec3::new(10.0, -5.0, 0.0))
            .with_rotation(Quat::from_rotation_z(-std::f32::consts::FRAC_PI_2));

        let entity_2_in_frame = frame.local(entity_2_in_world);

        assert!(entity_2_in_frame
            .translation
            .abs_diff_eq(Vec3::new(0.0, -10.0, 0.0), 1e-3));

        assert!(entity_2_in_frame
            .rotation
            .abs_diff_eq(Quat::from_rotation_z(-std::f32::consts::PI), 1e-3));
    }
}

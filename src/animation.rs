use bevy::prelude::*;

#[derive(Component, Deref, DerefMut)]
pub struct Repeating(pub Timer);

pub fn animate_sprites(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &mut Repeating,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
    )>,
) {
    query.for_each_mut(|(mut timer, mut sprite, texture_atlas_handle)| {
        timer.tick(time.delta());
        if timer.just_finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
        }
    });
}

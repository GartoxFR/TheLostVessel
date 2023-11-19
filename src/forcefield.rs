use bevy::prelude::*;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};
use bevy::sprite::Material2d;

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone, Default)]
pub struct ForceFieldMaterial {
    #[uniform(0)]
    pub color: Color,
    #[uniform(1)]
    pub time: f32,
}

impl Material2d for ForceFieldMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/forcefield.wgsl".into()
    }
}

pub fn forcefield_update_time(time: Res<Time>, mut materials: ResMut<Assets<ForceFieldMaterial>>) {
    for material in materials.iter_mut() {
        material.1.time = time.elapsed_seconds();
    }
}

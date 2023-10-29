use bevy::ecs::query::QuerySingleError;
use bevy::prelude::*;

#[derive(Component)]
pub struct ParalaxTarget;

#[derive(Component, Reflect)]
pub struct ParalaxBackground {
    pub paralax_factor: f32,
}

pub fn paralax_movement(
    target: Query<&Transform, (With<ParalaxTarget>, Changed<Transform>)>,
    mut background: Query<(&mut Transform, &ParalaxBackground), Without<ParalaxTarget>>,
) {
    match target.get_single() {
        Ok(target_transform) => {
            for (mut transform, ParalaxBackground { paralax_factor }) in background.iter_mut()  {
                transform.translation = target_transform.translation * *paralax_factor
            }
        }
        Err(QuerySingleError::MultipleEntities(..)) => {
            error!("Multiple ParalaxTarget were found");
        }
        _ => {}
    }
}

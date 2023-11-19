use bevy::prelude::*;
use bevy_hanabi::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::player::PlayerTag;
use crate::player_movement;

pub struct ParticlePlugin;

#[derive(Component)]
struct LeftRCS;

#[derive(Component)]
struct RightRCS;

impl Plugin for ParticlePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(HanabiPlugin)
            .add_systems(PostStartup, setup_particle_effect)
            .add_systems(Update, rcs_particles.after(player_movement));

    }
}

fn setup_particle_effect(
    mut commands: Commands<'_, '_>,
    player: Query<Entity, With<PlayerTag>>,
    asset_server: Res<AssetServer>,
    mut effects: ResMut<Assets<EffectAsset>>,
) {
    let particle_texture = asset_server.load("texture/cloud.png");
    let spawner = Spawner::rate(400.0.into());
    let mut gradient = Gradient::default();
    gradient.add_key(0.0, Vec4::splat(0.6));
    gradient.add_key(0.2, Vec4::splat(0.2));
    gradient.add_key(1.0, Vec4::splat(0.0));

    let writer = ExprWriter::new();
    let age = writer.lit(0.0).expr();
    let direction = writer.prop("direction");
    let parent_velocity = writer.prop("parent_velocity");

    let init_age = SetAttributeModifier::new(Attribute::AGE, age);

    let lifetime = writer.lit(0.2).expr();
    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

    let init_pos = SetPositionCircleModifier {
        // center: (writer.lit(-1.0) * parent_velocity.clone()).expr(),
        center: writer.lit(Vec3::ZERO).expr(),
        axis: writer.lit(Vec3::Z).expr(),
        radius: writer.lit(0.1).expr(),
        dimension: ShapeDimension::Volume,
    };

    let drag = writer.lit(2.).expr();
    let update_drag = LinearDragModifier::new(drag);

    let ortho = writer.lit(Vec3::Z).cross(direction.clone());
    let spread = writer.rand(ScalarType::Float) * writer.lit(2.) - writer.lit(1.);
    let speed = writer.lit(3.0);
    let velocity =
        parent_velocity + (direction + ortho * spread * writer.lit(0.3)).normalized() * speed;
    let init_vel = SetAttributeModifier::new(Attribute::VELOCITY, (velocity).expr());
    let effect = effects.add(
        EffectAsset::new(32768, spawner, writer.finish())
            .with_name("rcs_gas")
            .with_property("parent_velocity", Vec3::ZERO.into())
            .with_property("direction", Vec3::ZERO.into())
            .init(init_pos)
            .init(init_vel)
            .init(init_age)
            .init(init_lifetime)
            .update(update_drag)
            .render(ParticleTextureModifier {
                texture: particle_texture,
                sample_mapping: ImageSampleMapping::ModulateOpacityFromR,
            })
            .render(SetSizeModifier {
                size: Vec2::splat(15.0).into(),
                screen_space_size: true,
            })
            .render(ColorOverLifetimeModifier { gradient }),
    );

    commands.entity(player.single()).with_children(|player| {
        player
            .spawn(ParticleEffectBundle::new(effect.clone()).with_spawner(spawner))
            .insert(TransformBundle::from_transform(Transform::from_xyz(
                -310., -250., 0.,
            )))
            .insert(RightRCS);
        player
            .spawn(ParticleEffectBundle::new(effect).with_spawner(spawner))
            .insert(TransformBundle::from_transform(Transform::from_xyz(
                310., -250., 0.,
            )))
            .insert(LeftRCS);
    });
}

fn rcs_particles(
    player: Query<(&Velocity, &ExternalImpulse, &GlobalTransform), With<PlayerTag>>,
    mut right_rcs: Query<(&mut CompiledParticleEffect, &mut EffectSpawner), With<RightRCS>>,
    mut left_rcs: Query<(&mut CompiledParticleEffect, &mut EffectSpawner), Without<RightRCS>>,
) {
    let Ok((velocity, impulse, transform)) = player.get_single() else {
        return;
    };

    let Ok((mut right_effect, mut right_spawner)) = right_rcs.get_single_mut() else {
        return;
    };

    let Ok((mut left_effect, mut left_spawner)) = left_rcs.get_single_mut() else {
        return;
    };

    if impulse.impulse.length_squared() > 0.0 {
        let dir = -impulse.impulse.normalize();
        set_effect(&mut right_effect, &mut right_spawner, velocity, dir);
        set_effect(&mut left_effect, &mut left_spawner, velocity, dir);
    } else if impulse.torque_impulse.abs() > 0.0 {
        let dir = if impulse.torque_impulse > 0.0 {
            transform.up().truncate()
        } else {
            -transform.up().truncate()
        };

        set_effect(&mut right_effect, &mut right_spawner, velocity, dir);
        set_effect(&mut left_effect, &mut left_spawner, velocity, -dir);
    } else {
        right_spawner.set_active(false);
        left_spawner.set_active(false);
    }
}

fn set_effect(
    effect: &mut CompiledParticleEffect,
    spawner: &mut EffectSpawner,
    velocity: &Velocity,
    dir: Vec2,
) {
    spawner.set_active(true);
    effect.set_property("parent_velocity", velocity.linvel.extend(0.).into());
    effect.set_property("direction", Vec3::new(0., dir.x, dir.y).into());
}

mod paralax;
mod player;

use bevy::prelude::*;
use bevy::render::settings::{WgpuFeatures, WgpuSettings};
use bevy::render::RenderPlugin;
use bevy::window::WindowResolution;
use bevy_hanabi::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;

use self::paralax::{paralax_movement, ParalaxBackground, ParalaxTarget};
use self::player::{PlayerBundle, PlayerTag};

#[derive(Component)]
struct LeftRCS;

#[derive(Component)]
struct RightRCS;

fn main() {
    let mut wgpu_settings = WgpuSettings::default();
    wgpu_settings
        .features
        .set(WgpuFeatures::VERTEX_WRITABLE_STORAGE, true);
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::new(1280.0, 720.0),
                        resizable: false,
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(RenderPlugin { wgpu_settings }),
        )
        .register_type::<ParalaxBackground>()
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(WorldInspectorPlugin::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(HanabiPlugin)
        .insert_resource(RapierConfiguration {
            gravity: Vec2::ZERO,
            ..Default::default()
        })
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (player_movement, rcs_particles).chain(),
        )
        .add_systems(PostUpdate, paralax_movement.after(bevy::transform::TransformSystem::TransformPropagate))
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut effects: ResMut<Assets<EffectAsset>>,
) {
    let image = asset_server.load("background.png");
    commands
        .spawn(SpatialBundle::default())
        .with_children(|background| {
            for (i, j) in [(-1.0, -1.0), (-1.0, 1.0), (1.0, -1.0), (1.0, 1.0)] {
                background.spawn(SpriteBundle {
                    texture: image.clone(),
                    transform: Transform::from_xyz(512.0 * i, 512.0 * j, -10.0),
                    ..Default::default()
                });
            }
        })
        .insert(ParalaxBackground {
            paralax_factor: 0.95,
        })
        .insert(Name::new("Background"));
    commands
        .spawn(Camera2dBundle::default())
        .insert(ParalaxBackground {
            paralax_factor: 1.0,
        });

    commands
        .spawn(TransformBundle::from_transform(Transform::from_xyz(
            -300.0, 0.0, 0.0,
        )))
        .insert(Collider::segment(
            Vec2::new(0.0, -1000.0),
            Vec2::new(0.0, 1000.0),
        ));

    let particle_texture = asset_server.load("cloud.png");
    let spawner = Spawner::rate(400.0.into());
    let mut gradient = Gradient::default();
    gradient.add_key(0.0, Vec4::splat(1.));
    gradient.add_key(1.0, Vec4::splat(0.0));

    let writer = ExprWriter::new();
    let age = writer.lit(0.0).expr();
    let init_age = SetAttributeModifier::new(Attribute::AGE, age);

    let lifetime = writer.lit(0.2).expr();
    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

    let init_pos = SetPositionCircleModifier {
        center: writer.lit(Vec3::ZERO).expr(),
        axis: writer.lit(Vec3::Z).expr(),
        radius: writer.lit(10.0).expr(),
        dimension: ShapeDimension::Volume,
    };

    let drag = writer.lit(2.).expr();
    let update_drag = LinearDragModifier::new(drag);

    let direction = writer.prop("direction");
    let parent_velocity = writer.prop("parent_velocity");
    let ortho = writer.lit(Vec3::Z).cross(direction.clone());
    let spread = writer.rand(ScalarType::Float) * writer.lit(2.) - writer.lit(1.);
    let speed = writer.lit(200.0);
    let velocity =
        parent_velocity + (direction + ortho * spread * writer.lit(0.7)).normalized() * speed;
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
            })
            .render(SetSizeModifier {
                size: Vec2::splat(15.0).into(),
                screen_space_size: true,
            })
            .render(ColorOverLifetimeModifier { gradient }),
    );

    commands
        .spawn(PlayerBundle::new(asset_server.load("player.png")))
        .insert(ParalaxTarget)
        .with_children(|player| {
            player
                .spawn(ParticleEffectBundle::new(effect.clone()).with_spawner(spawner))
                .insert(TransformBundle::from_transform(Transform::from_xyz(
                    -40., -30., 0.,
                )))
                .insert(RightRCS);
            player
                .spawn(ParticleEffectBundle::new(effect).with_spawner(spawner))
                .insert(TransformBundle::from_transform(Transform::from_xyz(
                    40., -30., 0.,
                )))
                .insert(LeftRCS);
        });
}

const RCS_FORCE: f32 = 20.0;
const RCS_TORQUE: f32 = 0.01;

fn player_movement(
    gamepads: Res<Gamepads>,
    axes: Res<Axis<GamepadAxis>>,
    mut player: Query<&mut ExternalImpulse, With<PlayerTag>>,
) {
    for gamepad in gamepads.iter().take(1) {
        let mut left_stick_x = axes
            .get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickX))
            .unwrap();
        if left_stick_x.abs() < 0.2 {
            left_stick_x = 0.0;
        }
        let mut left_stick_y = axes
            .get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickY))
            .unwrap();
        if left_stick_y.abs() < 0.2 {
            left_stick_y = 0.0;
        }

        let mut right_stick_x = axes
            .get(GamepadAxis::new(gamepad, GamepadAxisType::RightStickX))
            .unwrap();

        if right_stick_x.abs() < 0.2 {
            right_stick_x = 0.0;
        }

        let left_stick = Vec2::new(left_stick_x, left_stick_y);
        let force = left_stick * RCS_FORCE;
        let torque = -right_stick_x * RCS_TORQUE;
        if let Ok(mut ball_force) = player.get_single_mut() {
            ball_force.impulse = force;
            ball_force.torque_impulse = torque;
        }
    }
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

mod dialog;
mod forcefield;
mod paralax;
mod particles;
mod player;
mod asset_enum;

use std::f32::consts::PI;

use bevy::input::gamepad::{GamepadButtonChangedEvent, GamepadEvent};
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy::sprite::{Material2dPlugin, MaterialMesh2dBundle};
use bevy::transform::TransformSystem;
use bevy::window::WindowResolution;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;
use bevy_scene_hook::*;

use self::dialog::{CurrentDialog, DialogPlugin, Portrait, Dialog};
use self::forcefield::{forcefield_update_time, ForceFieldMaterial};
use self::paralax::{paralax_movement, ParalaxBackground, ParalaxTarget};
use self::particles::ParticlePlugin;
use self::player::{PlayerBundle, PlayerTag};

#[derive(Debug, States, Default, Clone, Copy, Eq, PartialEq, Hash)]
enum AppState {
    #[default]
    InGame,
    InDialog,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(1280.0, 720.0),
                resizable: false,
                // present_mode: PresentMode::Immediate,
                ..Default::default()
            }),
            ..Default::default()
        }))
        .register_type::<ParalaxBackground>()
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(1.0))
        .add_plugins(WorldInspectorPlugin::default())
        .add_plugins(Material2dPlugin::<ForceFieldMaterial>::default())
        // .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(ParticlePlugin)
        .add_plugins(HookPlugin)
        .add_plugins(DialogPlugin)
        .add_state::<AppState>()
        .insert_resource(RapierConfiguration {
            gravity: Vec2::ZERO,
            ..Default::default()
        })
        .add_systems(Startup, setup)
        .add_systems(Update, (player_movement, launch_dialog))
        .add_systems(Update, forcefield_update_time)
        .add_systems(
            PostUpdate,
            paralax_movement
                .after(PhysicsSet::Writeback)
                .before(TransformSystem::TransformPropagate),
        )
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut forcefield_materials: ResMut<Assets<ForceFieldMaterial>>,
    mut dialog: ResMut<CurrentDialog>,
) {
    let quad = meshes.add(shape::Quad::new(Vec2::splat(1.0)).into());
    let quad_collider = Collider::cuboid(0.5, 0.5);
    let wall_rigidbody = RigidBody::Fixed;
    let wall_material = forcefield_materials.add(ForceFieldMaterial {
        color: Color::PURPLE,
        ..Default::default()
    });
    let image = asset_server.load("texture/background.png");
    commands
        .spawn(SpatialBundle::from_transform(Transform::from_scale(
            Vec3::splat(0.01),
        )))
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
        .spawn(Camera2dBundle {
            projection: OrthographicProjection {
                scaling_mode: ScalingMode::WindowSize(100.0),
                near: 10000.0,
                far: -10000.0,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(ParalaxBackground {
            paralax_factor: 1.0,
        });

    commands
        .spawn(MaterialMesh2dBundle {
            transform: Transform {
                translation: Vec3 {
                    x: 0.0,
                    y: 50.0,
                    z: 0.0,
                },
                scale: Vec3 {
                    x: 102.0,
                    y: 1.0,
                    z: 1.0,
                },
                ..Default::default()
            },
            mesh: quad.clone().into(),
            material: wall_material.clone(),
            ..Default::default()
        })
        .insert((quad_collider.clone(), wall_rigidbody));

    commands
        .spawn(MaterialMesh2dBundle {
            transform: Transform {
                translation: Vec3 {
                    x: 0.0,
                    y: -50.0,
                    z: 0.0,
                },
                scale: Vec3 {
                    x: 102.0,
                    y: 1.0,
                    z: 1.0,
                },
                ..Default::default()
            },
            mesh: quad.clone().into(),
            material: wall_material.clone(),
            ..Default::default()
        })
        .insert((quad_collider.clone(), wall_rigidbody));

    commands
        .spawn(MaterialMesh2dBundle {
            transform: Transform {
                translation: Vec3 {
                    x: 50.0,
                    y: 0.0,
                    z: 0.0,
                },
                scale: Vec3 {
                    x: 102.0,
                    y: 1.0,
                    z: 1.0,
                },
                rotation: Quat::from_rotation_z(PI / 2.0),
            },
            mesh: quad.clone().into(),
            material: wall_material.clone(),
            ..Default::default()
        })
        .insert((quad_collider.clone(), wall_rigidbody));

    commands
        .spawn(MaterialMesh2dBundle {
            transform: Transform {
                translation: Vec3 {
                    x: -50.0,
                    y: 0.0,
                    z: 0.0,
                },
                scale: Vec3 {
                    x: 102.0,
                    y: 1.0,
                    z: 1.0,
                },
                rotation: Quat::from_rotation_z(PI / 2.0),
            },
            mesh: quad.clone().into(),
            material: wall_material.clone(),
            ..Default::default()
        })
        .insert((quad_collider.clone(), wall_rigidbody));

    commands
        .spawn(MaterialMesh2dBundle {
            transform: Transform {
                translation: Vec3 {
                    x: -5.0,
                    y: 0.0,
                    z: 0.0,
                },
                ..Default::default()
            },
            mesh: quad.clone().into(),
            material: wall_material.clone(),
            ..Default::default()
        })
        .insert((quad_collider.clone(), wall_rigidbody));
    commands
        .spawn(PlayerBundle::new(asset_server.load("texture/player.png")))
        .insert(ParalaxTarget);
}

const RCS_FORCE: f32 = 0.5;
const RCS_TORQUE: f32 = 0.08;

fn launch_dialog(
    mut events: EventReader<GamepadButtonChangedEvent>,
    mut current_dialog: ResMut<CurrentDialog>,
    mut state: ResMut<NextState<AppState>>,
) {
    for event in events.read() {
        match event {
            GamepadButtonChangedEvent {
                button_type: GamepadButtonType::North,
                value,
                ..
            } if *value > 0.5 => {
                current_dialog.set(Dialog::Test);
                state.set(AppState::InDialog);
            }
            _ => {}
        }
    }
}

fn player_movement(
    time: Res<Time>,
    gamepads: Res<Gamepads>,
    buttons: Res<Input<GamepadButton>>,
    axes: Res<Axis<GamepadAxis>>,
    mut player: Query<(&mut ExternalImpulse, &mut Velocity, &GlobalTransform), With<PlayerTag>>,
    state: Res<State<AppState>>,
) {
    let (mut impulse, mut current_velocity, transform) = player.single_mut();
    if *state == AppState::InDialog {
        stop_player(&mut current_velocity, &mut impulse, &time);
        return;
    }

    for gamepad in gamepads.iter().take(1) {
        let stop_button = GamepadButton {
            gamepad,
            button_type: GamepadButtonType::South,
        };
        let mut right_stick_x = axes
            .get(GamepadAxis::new(gamepad, GamepadAxisType::RightStickX))
            .unwrap();
        if right_stick_x.abs() < 0.2 {
            right_stick_x = 0.0;
        }
        let mut left_stick_y = axes
            .get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickY))
            .unwrap();
        if left_stick_y.abs() < 0.2 {
            left_stick_y = 0.0;
        }

        if buttons.pressed(stop_button) {
            stop_player(&mut current_velocity, &mut impulse, &time)
        } else {
            let force = transform.up().truncate() * left_stick_y * RCS_FORCE;
            let torque = -right_stick_x * RCS_TORQUE;

            impulse.impulse += force * time.delta_seconds();
            impulse.torque_impulse += torque * time.delta_seconds();
        }
    }
}

fn stop_player(current_velocity: &mut Velocity, impulse: &mut ExternalImpulse, time: &Time) {
    if current_velocity.linvel.length() > 0.5 {
        impulse.impulse =
            -current_velocity.linvel.normalize() * RCS_FORCE * 2.0 * time.delta_seconds();
    } else {
        current_velocity.linvel = Vec2::splat(0.0);
    }

    if current_velocity.angvel.abs() > 0.1 {
        impulse.torque_impulse =
            -current_velocity.angvel.signum() * RCS_TORQUE * 2.0 * time.delta_seconds();
    } else {
        current_velocity.angvel = 0.0;
    }
}

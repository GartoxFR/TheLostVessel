mod asset_enum;
mod dialog;
mod forcefield;
pub mod objects;
mod old;
mod paralax;
mod particles;
mod player;
mod tilemap;

use bevy::input::keyboard::KeyboardInput;
use bevy::input::ButtonState;
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy::transform::TransformSystem;
use bevy::window::WindowResolution;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;

use self::dialog::{CurrentDialog, Dialog, DialogPlugin};
use self::objects::ObjectsPlugin;
use self::paralax::{paralax_movement, ParalaxBackground, ParalaxTarget};
use self::player::{PlayerBundle, PlayerTag};
use self::tilemap::spawn_map;

#[derive(Debug, States, Default, Clone, Copy, Eq, PartialEq, Hash)]
pub enum AppState {
    #[default]
    InGame,
    InDialog,
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::new(1280.0, 720.0),
                        // present_mode: PresentMode::Immediate,
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(16.0))
        // .add_plugins(WorldInspectorPlugin::default())
        .add_plugins(RapierDebugRenderPlugin::default().disabled())
        .add_plugins(DialogPlugin)
        .add_plugins(ObjectsPlugin)
        .add_state::<AppState>()
        .add_event::<ResetEvent>()
        .add_systems(Startup, (setup, spawn_map))
        .add_systems(
            Update,
            (
                (movement, reset).run_if(in_state(AppState::InGame)),
                control_debug_renderer,
            ),
        )
        .add_systems(
            PostUpdate,
            paralax_movement
                .after(PhysicsSet::Writeback)
                .before(TransformSystem::TransformPropagate),
        )
        .insert_resource(RapierConfiguration {
            gravity: Vec2::ZERO,
            ..Default::default()
        })
        .run();
}

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut atlases: ResMut<Assets<TextureAtlas>>,
    mut current_dialog: ResMut<CurrentDialog>,
    mut state: ResMut<NextState<AppState>>,
) {
    commands
        .spawn(Camera2dBundle {
            transform: Transform::from_translation(Vec3::new(200.0, -300.0, 10.0)),
            projection: OrthographicProjection {
                scaling_mode: ScalingMode::WindowSize(3.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(ParalaxBackground {
            paralax_factor: 1.0,
        });
    let player_atlas = TextureAtlas::from_grid(
        asset_server.load("texture/player.png"),
        Vec2::splat(32.0),
        4,
        1,
        None,
        None,
    );
    let player_atlas = atlases.add(player_atlas);
    commands
        .spawn(PlayerBundle::new(
            Transform::from_translation(Vec3::new(200.0, -300.0, 1.0)),
            player_atlas,
        ))
        .insert(ParalaxTarget)
        .with_children(|commands| {
            commands.spawn((
                SpatialBundle::from_transform(Transform::from_xyz(0.0, -9.5, 0.0)),
                Collider::cuboid(6.5, 2.5),
            ));
        });

    current_dialog.set(Dialog::Intro);
    state.set(AppState::InDialog);
}

const RCS_FORCE: f32 = 100.0;

pub fn movement(
    time: Res<Time>,
    mut player_impulse: Query<(&mut ExternalImpulse, &mut TextureAtlasSprite), With<PlayerTag>>,
    inputs: Res<Input<KeyCode>>,
) {
    let (mut impulse, mut sprite) = player_impulse.single_mut();

    let mut add_impulse = Vec2::default();

    if inputs.pressed(KeyCode::S) {
        add_impulse.y -= 1.0;
        sprite.index = 0;
    }

    if inputs.pressed(KeyCode::W) {
        add_impulse.y += 1.0;
        sprite.index = 2;
    }

    if inputs.pressed(KeyCode::A) {
        add_impulse.x -= 1.0;
        sprite.index = 3;
    }

    if inputs.pressed(KeyCode::D) {
        add_impulse.x += 1.0;
        sprite.index = 1;
    }

    if add_impulse != Vec2::default() {
        impulse.impulse += add_impulse.normalize() * RCS_FORCE * time.delta_seconds();
    }
}

pub fn control_debug_renderer(
    mut events: EventReader<KeyboardInput>,
    mut debug_render_context: ResMut<DebugRenderContext>,
) {
    for _ in events.read().filter(|input| {
        matches!(
            input,
            KeyboardInput {
                state: ButtonState::Pressed,
                key_code: Some(KeyCode::F1),
                ..
            }
        )
    }) {
        debug_render_context.enabled = !debug_render_context.enabled;
    }
}

#[derive(Debug, Default, Event)]
pub struct ResetEvent;

pub fn reset(
    mut events: EventReader<KeyboardInput>,
    mut player: Query<(&mut Transform, &mut Velocity), With<PlayerTag>>,
    mut current_dialog: ResMut<CurrentDialog>,
    mut state: ResMut<NextState<AppState>>,
    mut event: EventWriter<ResetEvent>,
) {
    for _ in events.read().filter(|input| {
        matches!(
            input,
            KeyboardInput {
                state: ButtonState::Pressed,
                key_code: Some(KeyCode::R),
                ..
            }
        )
    }) {
        let (mut transform, mut velocity) = player.single_mut();
        *transform = Transform::from_translation(Vec3::new(200.0, -300.0, 1.0));
        *velocity = Default::default();

        current_dialog.set(Dialog::Intro);
        state.set(AppState::InDialog);
        event.send_default();
    }
}

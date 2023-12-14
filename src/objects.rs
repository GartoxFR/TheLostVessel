use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::dialog::{CurrentDialog, Dialog};
use crate::{AppState, ResetEvent};

pub struct ObjectsPlugin;

#[derive(Component)]
struct DialogTrigger(Dialog, bool);

impl DialogTrigger {
    fn new(dialog: Dialog) -> Self {
        Self(dialog, false)
    }
}

impl Plugin for ObjectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_objects)
            .add_systems(Update, (trigger_check.run_if(in_state(AppState::InGame)), reset_check));
    }
}

fn spawn_objects(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            Name::new("Teddy bear"),
            SpriteBundle {
                texture: asset_server.load("texture/teddy_bear.png"),
                transform: Transform::from_xyz(113.0, -206.0, 0.1),
                ..Default::default()
            },
            RigidBody::Fixed,
        ))
        .with_children(|commands| {
            commands.spawn((
                Name::new("Solid collider"),
                SpatialBundle::default(),
                Collider::ball(6.0),
            ));
            commands.spawn((
                Name::new("Dialogs sensor collider"),
                SpatialBundle::default(),
                Collider::ball(24.0),
                Sensor,
                ActiveEvents::COLLISION_EVENTS,
                DialogTrigger::new(Dialog::Bear),
            ));
        });

    commands
        .spawn((
            Name::new("Plant"),
            SpriteBundle {
                texture: asset_server.load("texture/plant.png"),
                transform: Transform::from_xyz(399.0, -200.0, 0.1),
                ..Default::default()
            },
            RigidBody::Fixed,
        ))
        .with_children(|commands| {
            commands.spawn((
                Name::new("Solid collider"),
                SpatialBundle::default(),
                Collider::cuboid(8.0, 14.0),
            ));
            commands.spawn((
                Name::new("Dialogs sensor collider"),
                SpatialBundle::default(),
                Collider::ball(32.0),
                Sensor,
                ActiveEvents::COLLISION_EVENTS,
                DialogTrigger::new(Dialog::Plant),
            ));
        });
}

fn trigger_check(
    mut collision_events: EventReader<CollisionEvent>,
    mut dialog_trigger: Query<&mut DialogTrigger>,
    mut current_dialog: ResMut<CurrentDialog>,
    mut state: ResMut<NextState<AppState>>,
) {
    for collision_event in collision_events.read() {
        if let CollisionEvent::Started(entity1, entity2, _) = *collision_event {
            if let Ok(mut dialog_trigger) = dialog_trigger.get_mut(entity1) {
                if !dialog_trigger.1 {
                    current_dialog.set(dialog_trigger.0);
                    state.set(AppState::InDialog);
                    dialog_trigger.1 = true;
                }
            } else if let Ok(mut dialog_trigger) = dialog_trigger.get_mut(entity2) {
                if !dialog_trigger.1 {
                    current_dialog.set(dialog_trigger.0);
                    state.set(AppState::InDialog);
                    dialog_trigger.1 = true;
                }
            }
        }
    }
}

fn reset_check(mut events: EventReader<ResetEvent>, mut triggers: Query<&mut DialogTrigger>) {
    for _ in events.read() {
        for mut dialog in triggers.iter_mut() {
            dialog.1 = false;
        }
    }
}

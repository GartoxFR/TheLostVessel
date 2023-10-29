use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct PlayerTag;

#[derive(Bundle)]
pub struct PlayerBundle {
    tag: PlayerTag,
    name: Name,
    sprite: SpriteBundle,
    rigidbody: RigidBody,
    friction: Friction,
    velocity: Velocity,
    force: ExternalImpulse,
    locked_axes: LockedAxes,
    damping: Damping,
    collider: Collider,
    mass: ColliderMassProperties,
}

impl PlayerBundle {
    pub fn new(image: Handle<Image>) -> Self {
        let sprite = SpriteBundle {
            texture: image,
            ..Default::default()
        };

        let damping = Damping {
            linear_damping: 1.0,
            angular_damping: 1.0,
        };

        Self {
            tag: PlayerTag,
            name: Name::new("Player"),
            sprite,
            rigidbody: RigidBody::Dynamic,
            friction: Friction::coefficient(0.0),
            force: ExternalImpulse::default(),
            locked_axes: LockedAxes::empty(),
            damping,
            collider: Collider::ball(45.0),
            velocity: Velocity::default(),
            mass: ColliderMassProperties::Density(1.0),
        }
    }
}

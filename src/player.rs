use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct PlayerTag;

#[derive(Bundle)]
pub struct PlayerBundle {
    tag: PlayerTag,
    name: Name,
    sprite: SpriteSheetBundle,
    rigidbody: RigidBody,
    friction: Friction,
    velocity: Velocity,
    force: ExternalImpulse,
    locked_axes: LockedAxes,
    damping: Damping,
    mass: ColliderMassProperties,
}

impl PlayerBundle {
    pub fn new(transform: Transform, image: Handle<TextureAtlas>) -> Self {
        let sprite = SpriteSheetBundle {
            transform,
            texture_atlas: image,
            sprite: TextureAtlasSprite::new(0),
            ..Default::default()
        };

        let damping = Damping {
            linear_damping: 3.0,
            angular_damping: 0.0,
        };

        Self {
            tag: PlayerTag,
            name: Name::new("Player"),
            sprite,
            rigidbody: RigidBody::Dynamic,
            friction: Friction::coefficient(0.0),
            force: ExternalImpulse::default(),
            locked_axes: LockedAxes::ROTATION_LOCKED,
            damping,
            velocity: Velocity::default(),
            mass: ColliderMassProperties::Density(1.0),
        }
    }
}

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

const MAP: &str = r#"
999999999
999999999
992131099
994555899
994555899
996AAA799
999999999
999999999
"#;

const TILE_SIZE: f32 = 64.0;

pub fn spawn_map(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture = asset_server.load("texture/tilemap.png");
    let atlas = atlases.add(TextureAtlas::from_grid(
        texture,
        Vec2::splat(64.0),
        4,
        4,
        None,
        None,
    ));

    commands
        .spawn((SpatialBundle::default(), Name::new("Map")))
        .with_children(|commands| {
            for (row, (col, tile)) in MAP
                .lines()
                .enumerate()
                .flat_map(|(row, line)| std::iter::repeat(row).zip(line.bytes().enumerate()))
            {
                let tile_byte = &[tile];
                let s = std::str::from_utf8(tile_byte).unwrap();
                let index = i64::from_str_radix(s, 16).unwrap();

                let mut entity = commands.spawn(SpriteSheetBundle {
                    texture_atlas: atlas.clone(),
                    sprite: TextureAtlasSprite::new(index as usize),
                    transform: Transform {
                        translation: Vec3 {
                            x: col as f32 * TILE_SIZE,
                            y: -(row as f32 * TILE_SIZE),
                            z: 0.0,
                        },
                        ..Default::default()
                    },
                    ..Default::default()
                });
                let collider_spec = get_collider(index);
                if !collider_spec.is_empty() {
                    entity.insert(RigidBody::Fixed).with_children(|commands| {
                        for (pos, collider) in collider_spec {
                            commands.spawn((
                                SpatialBundle::from_transform(Transform::from_translation(
                                    pos.extend(0.0),
                                )),
                                collider,
                            ));
                        }
                    });
                }
            }
        });
}

pub fn get_collider(index: i64) -> Vec<(Vec2, Collider)> {
    match index {
        0 => vec![
            (Vec2::new(0.0, 13.0), Collider::cuboid(32.0, 19.0)),
            (Vec2::new(28.5, 0.0), Collider::cuboid(3.5, 32.0)),
        ],
        1 | 3 => vec![
            (Vec2::new(0.0, 13.0), Collider::cuboid(32.0, 19.0)),
        ],
        2  => vec![
            (Vec2::new(0.0, 13.0), Collider::cuboid(32.0, 19.0)),
            (Vec2::new(-28.5, 0.0), Collider::cuboid(3.5, 32.0)),
        ],
        4 => vec![
            (Vec2::new(-28.5, 0.0), Collider::cuboid(3.5, 32.0)),
        ],
        8 => vec![
            (Vec2::new(28.5, 0.0), Collider::cuboid(3.5, 32.0)),
        ],
        7 => vec![
            (Vec2::new(0.0, -28.5), Collider::cuboid(32.0, 3.5)),
            (Vec2::new(28.5, 0.0), Collider::cuboid(3.5, 32.0)),
        ],
        6 => vec![
            (Vec2::new(0.0, -28.5), Collider::cuboid(32.0, 3.5)),
            (Vec2::new(-28.5, 0.0), Collider::cuboid(3.5, 32.0)),
        ],
        
        10 => vec![
            (Vec2::new(0.0, -28.5), Collider::cuboid(32.0, 3.5)),
        ],
        _ => vec![],
    }
}

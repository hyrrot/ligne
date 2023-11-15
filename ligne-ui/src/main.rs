use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_ecs_tilemap::prelude::*;

const QUADRANT_SIDE_LENGTH: u32 = 80;

pub fn fill_tilemap(
    texture_index: TileTextureIndex,
    size: TilemapSize,
    tilemap_id: TilemapId,
    commands: &mut Commands,
    tile_storage: &mut TileStorage,
) {
    commands.entity(tilemap_id.0).with_children(|parent| {
        for x in 0..size.x {
            for y in 0..size.y {
                let tile_pos = TilePos { x, y };
                let tile_entity = parent
                    .spawn(TileBundle {
                        position: tile_pos,
                        tilemap_id,
                        texture_index,
                        ..Default::default()
                    })
                    .id();
                tile_storage.set(&tile_pos, tile_entity);
            }
        }
    });
}

pub fn fill_tilemap_rect(
    texture_index: TileTextureIndex,
    origin: TilePos,
    size: TilemapSize,
    tilemap_id: TilemapId,
    commands: &mut Commands,
    tile_storage: &mut TileStorage,
) {
    commands.entity(tilemap_id.0).with_children(|parent| {
        for x in 0..size.x {
            for y in 0..size.y {
                if x % 3 == 0 && y % 3 == 0 {
                    continue;
                }
                let tile_pos = TilePos {
                    x: origin.x + x,
                    y: origin.y + y,
                };

                let tile_entity = parent
                    .spawn(TileBundle {
                        position: tile_pos,
                        tilemap_id,
                        texture_index,
                        ..Default::default()
                    })
                    .id();
                tile_storage.set(&tile_pos, tile_entity);
            }
        }
    });
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("Layers Example"),
                        ..Default::default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(TilemapPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    assets_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    commands.spawn(Camera2dBundle::default());

    let texture_handle: Handle<Image> = assets_server.load("EmptyChip.png");
    let map_size = TilemapSize {
        x: QUADRANT_SIDE_LENGTH,
        y: QUADRANT_SIDE_LENGTH,
    };
    let quadrant_size = TilemapSize {
        x: QUADRANT_SIDE_LENGTH,
        y: QUADRANT_SIDE_LENGTH,
    };
    let mut tile_storage = TileStorage::empty(map_size);
    let tilemap_entity = commands.spawn_empty().id();
    let tilemap_id = TilemapId(tilemap_entity);

    fill_tilemap_rect(
        TileTextureIndex(0),
        TilePos { x: 0, y: 0 },
        quadrant_size,
        tilemap_id,
        &mut commands,
        &mut tile_storage,
    );

    let tile_size = TilemapTileSize { x: 32.0, y: 16.0 };
    let grid_size = tile_size.into();
    let map_type = TilemapType::Isometric(IsoCoordSystem::Diamond);

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(texture_handle),
        tile_size,
        map_type,
        transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.0),
        ..Default::default()
    });

    let texture_handle_tgv: Handle<Image> = assets_server.load("tgv_atr_a.png");
    let texture_atlas_tgv =
        TextureAtlas::from_grid(texture_handle_tgv, Vec2::new(32.0, 32.0), 24, 1, None, None);
    let texture_atlas_handle_tgv = texture_atlases.add(texture_atlas_tgv);

    commands.spawn(SpriteSheetBundle {
        texture_atlas: texture_atlas_handle_tgv,
        transform: Transform::from_xyz(16., 16., 2.),
        sprite: TextureAtlasSprite {
            index: 0,
            ..default()
        },
        ..default()
    });

    let texture_handle_railroads: Handle<Image> = assets_server.load("RailRoads.png");
    let texture_atlas_railroads = TextureAtlas::from_grid(
        texture_handle_railroads,
        Vec2::new(32.0, 16.0),
        48,
        1,
        None,
        None,
    );
    let texture_atlas_handle_railroads = texture_atlases.add(texture_atlas_railroads);

    commands.spawn(SpriteSheetBundle {
        texture_atlas: texture_atlas_handle_railroads,
        transform: Transform::from_xyz(16., 16., 1.),
        sprite: TextureAtlasSprite {
            index: 10,
            ..default()
        },
        ..default()
    });

    // commands.spawn(SpriteBundle {
    //     texture: texture_handle_tgv,
    //     transform: Transform::from_xyz(0., 0., 1.),
    //     ..default()
    // });
}

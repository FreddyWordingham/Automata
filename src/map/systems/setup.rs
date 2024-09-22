use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use rayon::prelude::*;

use super::super::constants::*;

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    array_texture_loader: Res<ArrayTextureLoader>,
) {
    commands.spawn(Camera2dBundle::default());

    let texture_handle: Handle<Image> = asset_server.load(TILE_SHEET_FILEPATH);

    let map_size = TilemapSize {
        x: MAP_WIDTH,
        y: MAP_HEIGHT,
    };
    let tilemap_entity = commands.spawn_empty().id();
    let mut tile_storage = TileStorage::empty(map_size);

    // Collect all positions and prepare their bundles in parallel
    let tile_bundles: Vec<(TilePos, TileBundle)> = (0..map_size.x)
        .flat_map(|x| (0..map_size.y).map(move |y| TilePos { x, y }))
        .collect::<Vec<_>>() // Collect positions first
        .par_iter()          // Parallelize the next part
        .map(|&tile_pos| {
            let tile_bundle = TileBundle {
                position: tile_pos,
                tilemap_id: TilemapId(tilemap_entity),
                ..Default::default()
            };
            (tile_pos, tile_bundle)
        })
        .collect(); // Collect into a Vec of (TilePos, TileBundle)

    // Now that we've built all the bundles, we can spawn entities serially
    for (tile_pos, tile_bundle) in tile_bundles {
        let tile_entity = commands.spawn(tile_bundle).id();
        tile_storage.set(&tile_pos, tile_entity);
    }

    let tile_size = TilemapTileSize {
        x: TILE_WIDTH,
        y: TILE_HEIGHT,
    };
    let grid_size = tile_size.into();
    let map_type = TilemapType::default();

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        map_type,
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(texture_handle.clone()), // Reuse the texture handle
        tile_size,
        transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.0),
        ..Default::default()
    });

    array_texture_loader.add(TilemapArrayTexture {
        texture: TilemapTexture::Single(texture_handle), // Reuse the texture handle
        tile_size,
        ..Default::default()
    });
}

use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

mod constants;
mod resources;
mod systems;

// pub use resources::*;
pub use systems::*;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TilemapPlugin).add_systems(Startup, setup);
    }
}

use crate::GameState;
use bevy::prelude::{Plugin, App, SystemSet, Commands, OrthographicCameraBundle};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
   commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
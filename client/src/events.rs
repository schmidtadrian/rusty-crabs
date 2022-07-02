
use bevy::{prelude::{App, Plugin, EventReader, warn, Commands, Res, Transform, SystemSet, AssetServer}, sprite::SpriteBundle, math::Vec3};
use shared::protocol::ServerMessage;

use crate::{loading::TextureAssets, GameState};

pub struct EventPlugin;

impl Plugin for EventPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ServerMessage>()
            //.add_system(handle_server_messages);
            .add_system_set(
                SystemSet::on_update(GameState::InGame)
                    .with_system(handle_server_messages)
            );
    }
}

fn handle_server_messages(
    mut ev_server_message: EventReader<ServerMessage>,
    mut commands: Commands,
    //textures: Res<TextureAssets>,
    asset_server: Res<AssetServer>
) {
    for ev in ev_server_message.iter() {
        match ev {
            ServerMessage::Position { x, y } => {
                warn!("Spawning player x: {}, y: {}", x, y);
                commands.spawn_bundle(SpriteBundle {
                    //texture: textures.texture_bevy.clone(),
                    texture: asset_server.load("textures/bevy.png"),
                    transform: Transform {
                        translation: Vec3::new(*x, *y, 0.),
                        scale: Vec3::new(0.1, 0.1, 0.1),
                        ..Default::default()
                    },
                    ..Default::default()
                });
            }
        }
    }
}
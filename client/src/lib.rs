mod game;
mod network;
mod events;
mod loading;

use crate::loading::LoadingPlugin;
use crate::events::EventPlugin;
use crate::network::NetworkPlugin;
use crate::game::GamePlugin;

use bevy::prelude::{App, Plugin};

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    Loading,
    InGame,
    Menu
}
pub struct MainPlugin;

impl Plugin for MainPlugin {
    fn build(&self, app: &mut App) {
        app.add_state(GameState::InGame)
            .add_plugin(LoadingPlugin)
            .add_plugin(EventPlugin)
            .add_plugin(GamePlugin)
            .add_plugin(NetworkPlugin);
    }
}
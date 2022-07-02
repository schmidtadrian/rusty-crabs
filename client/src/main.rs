use bevy::{prelude::{App, Color}, DefaultPlugins, core_pipeline::ClearColor, window::WindowDescriptor};
use client::MainPlugin;
use shared::constants::{HEIGHT, WIDTH};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.4, 0.4, 0.4)))
        .insert_resource(WindowDescriptor {
            width: WIDTH,
            height: HEIGHT,
            title: "Rusty Crabs".to_string(),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(MainPlugin)
        .run();
}

use std::{time::Duration, net::SocketAddr};

use bevy::{app::ScheduleRunnerSettings, MinimalPlugins, log::LogPlugin, prelude::{App, ResMut, info, Component, Commands, Changed, Query}, math::{Vec2, vec2}};
use bevy_networking_turbulence::{NetworkingPlugin, NetworkResource, ConnectionChannelsBuilder};

use shared::{constants::{PORT, CLIENT_MESSAGE_SEETINGS}, protocol::{ClientMessage, ServerMessage}, channel::setup_channels};

#[derive(Component)]
struct Player(String);

#[derive(Component)]
struct Position(Vec2);

fn main() {
    App::new()
        .insert_resource(ScheduleRunnerSettings::run_loop(Duration::from_secs_f64(
            1.0 / 60.0,
        )))
        .add_plugins(MinimalPlugins)
        .add_plugin(LogPlugin)
        .add_plugin(NetworkingPlugin::default())
        .add_startup_system(setup_channels)
        .add_startup_system(setup_network)
        .add_system(handle_packets)
        .add_system(send_packets)
        .run();
}

fn setup_network(mut net: ResMut<NetworkResource>) {
    let ip_address = bevy_networking_turbulence::find_my_ip_address()
        .expect("Could't get IP");
    let server_address = SocketAddr::new(ip_address, PORT);
    
//    net.set_channels_builder(|builder: &mut ConnectionChannelsBuilder| {
//        builder.register::<ClientMessage>(CLIENT_MESSAGE_SEETINGS).unwrap();
//    });

    net.listen(server_address, None, None);
    info!("Started server: {}, {}", ip_address, PORT);
}

fn handle_packets(
    mut commands: Commands,
    mut net: ResMut<NetworkResource>
) {
    for (handle, connection) in net.connections.iter_mut() {
        let channels = connection.channels().unwrap();
        while let Some(client_message) = channels.recv::<ClientMessage>() {
            match client_message {
                ClientMessage::Join{ name} => {
                    info!("Client [{}] connected on [{}]", name, handle);
                    commands.spawn_bundle((
                        Player((*name).to_string()),
                        Position(vec2(300.0, 200.0))
                    ));
                    info!("Player: {} spawned x: {} y: {}", name.to_string(), 100.0, 100.0);
                }
                event => info!("{event:?} received!"),
            }
        }
    }
}

fn send_packets(
    mut net: ResMut<NetworkResource>,
    pos: Query<&Position, Changed<Position>>
) {
    for position in pos.iter() {
        info!("x: {}", position.0[0]);
        info!("y: {}", position.0[1]);
        
        let _ = net.broadcast_message(ServerMessage::Position { x: position.0[0], y: position.0[1] });
    }
}
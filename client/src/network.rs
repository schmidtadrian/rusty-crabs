use std::{time::Duration, net::{Ipv4Addr, IpAddr, SocketAddr}};

use bevy::{prelude::{Plugin, App, ResMut, warn, EventReader, EventWriter, SystemSet}, app::ScheduleRunnerSettings};
use bevy_networking_turbulence::{NetworkingPlugin, NetworkResource, NetworkEvent, ConnectionChannelsBuilder, Packet};
use shared::{constants::{PORT, CLIENT_MESSAGE_SEETINGS}, protocol::{ClientMessage, ServerMessage}, channel::setup_channels};

use crate::GameState;

pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ScheduleRunnerSettings::run_loop(Duration::from_secs_f64(
            1.0 / 60.0,
        )))
        .add_plugin(NetworkingPlugin::default())
        .add_startup_system(setup_channels)
        .add_startup_system(setup_network)
        .add_system_set(
            SystemSet::on_update(GameState::InGame).with_system(handle_packets)
        );
        //.add_system(handle_packets)
    }
}

fn setup_network(mut net: ResMut<NetworkResource>) {
    let ip_address = IpAddr::V4(Ipv4Addr::new(192, 168, 178, 29));
    let socket_address = SocketAddr::new(ip_address, PORT);

//    net.set_channels_builder(|builder: &mut ConnectionChannelsBuilder| {
//        builder.register::<ClientMessage>(CLIENT_MESSAGE_SEETINGS).unwrap();
//    });

    warn!("Connecting to {}...", socket_address);
    net.connect(socket_address);
}

fn handle_packets(
    mut net: ResMut<NetworkResource>,
    mut reader: EventReader<NetworkEvent>,
    mut event_writer: EventWriter<ServerMessage>
) {
    for event in reader.iter() {
        match event {
            NetworkEvent::Connected(handle) => match net.connections.get_mut(handle) {
                Some (_connection) => {
                    warn!("Connected successful!");
                    println!("{}", handle.to_string());
                    let _ = net.send_message(*handle, ClientMessage::Join { name: "HI".to_string() });
                }
                None => panic!("Got packet for non-existing connection [{}]", handle),
            },
            event => warn!("{event:?} received!"),
        }
    }
    for (handle, connection) in net.connections.iter_mut() {
        let channels = connection.channels().unwrap();
        while let Some(mut server_message) = channels.recv::<ServerMessage>() {
            warn!("ServerMessage received on [{}]: {:?}", handle, server_message);
            event_writer.send(server_message);
        }
    }
}
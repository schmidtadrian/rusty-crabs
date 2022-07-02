use bevy::{app::ScheduleRunnerSettings, DefaultPlugins, prelude::{App, ResMut, info, warn, EventReader}};
use bevy_networking_turbulence::{NetworkingPlugin, NetworkResource, NetworkEvent, Packet};
use serde::{Serialize, Deserialize};

use std::{time::Duration, net::{IpAddr, Ipv4Addr, SocketAddr}};

use shared::protocol;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ClientMessage {
    Hello,
}

fn main() {
    App::new()
        .insert_resource(ScheduleRunnerSettings::run_loop(Duration::from_secs_f64(
            1.0 / 60.0,
        )))
        .add_plugins(DefaultPlugins)
        //.add_plugin(LogPlugin)
        .add_plugin(NetworkingPlugin::default())
        .add_startup_system(setup_network)
        .add_system(handle_packets)
        .run();
}

fn setup_network(mut net: ResMut<NetworkResource>) {
    let ip_address = IpAddr::V4(Ipv4Addr::new(192, 168, 178, 29));
    let socket_address = SocketAddr::new(ip_address, 9001);
    warn!("Connecting to {}...", socket_address);
    net.connect(socket_address);
}

fn handle_packets(
    mut net: ResMut<NetworkResource>,
    mut reader: EventReader<NetworkEvent>
) {
    for event in reader.iter() {
        match event {
            NetworkEvent::Connected(handle) => match net.connections.get_mut(handle) {
                Some(_connection) => {
                    info!("Connection successful");
                    let _ = net.broadcast(Packet::from("Hello from client!"));
                }
                None => panic!("Got packet for non-existing connection [{}]", handle),
            },
            NetworkEvent::Packet(handle, packet) => {
                let message = String::from_utf8_lossy(packet);
                info!("Got packet on [{}]: {}", handle, message);
            }
            event => info!("{event:?} received!")
        }
    }
}
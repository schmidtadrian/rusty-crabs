use bevy::app::{App, ScheduleRunnerSettings};
use bevy::MinimalPlugins;
use bevy::log::{LogPlugin, info};
use bevy::prelude::{ResMut, EventReader, Res};
use bevy_networking_turbulence::{NetworkingPlugin, NetworkResource, NetworkEvent, Packet};

use std::time::Duration;
use std::net::SocketAddr;

fn main() {
    App::new()
        .insert_resource(ScheduleRunnerSettings::run_loop(Duration::from_secs_f64(
            1.0 / 60.0,
        )))
        .add_plugins(MinimalPlugins)
        .add_plugin(LogPlugin)
        .add_plugin(NetworkingPlugin::default())
        .add_startup_system(setup_network)
        .add_system(handle_packets)
        .add_system(send_packets)
        .run();
}

fn setup_network(mut net: ResMut<NetworkResource>) {
    let ip_address = 
        bevy_networking_turbulence::find_my_ip_address().expect("Could't find IP");
    let server_address = SocketAddr::new(ip_address, 9001);
    net.listen(server_address, None, None);
    info!("Started server: {}, {}", ip_address, 9001);
}

fn handle_packets(
    mut reader: EventReader<NetworkEvent>
) {
    for event in reader.iter() {
        match event {
            NetworkEvent::Packet(handle, packet) => {
                let message = String::from_utf8_lossy(packet);
                info!("Got packet on [{}]: {}", handle, message);
            }
            event => info!("{event:?} received!")
        }
    }
}

fn send_packets(
    mut net: ResMut<NetworkResource>
) {
    let _ = net.broadcast(Packet::from("Server data"));
}
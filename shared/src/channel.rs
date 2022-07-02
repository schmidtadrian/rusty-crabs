use bevy::prelude::ResMut;
use bevy_networking_turbulence::{NetworkResource, ConnectionChannelsBuilder};

use crate::protocol::{ClientMessage, ServerMessage};
use crate::constants::{CLIENT_MESSAGE_SEETINGS, SERVER_MESSAGE_SEETINGS};


pub fn setup_channels(mut net: ResMut<NetworkResource>) {
    net.set_channels_builder(|builder: &mut ConnectionChannelsBuilder| {
        builder
            .register::<ClientMessage>(CLIENT_MESSAGE_SEETINGS)
            .unwrap();
        builder
            .register::<ServerMessage>(SERVER_MESSAGE_SEETINGS)
            .unwrap();
    });
}
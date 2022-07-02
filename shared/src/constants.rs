use bevy_networking_turbulence::{MessageChannelSettings, MessageChannelMode};

pub const PORT: u16 = 9001;
pub const WIDTH: f32 = 800.;
pub const HEIGHT: f32 = 600.;

pub const CLIENT_MESSAGE_SEETINGS: MessageChannelSettings = MessageChannelSettings {
    channel: 0,
    channel_mode: MessageChannelMode::Unreliable,
    message_buffer_size: 8,
    packet_buffer_size: 8,
};

pub const SERVER_MESSAGE_SEETINGS: MessageChannelSettings = MessageChannelSettings {
    channel: 1,
    channel_mode: MessageChannelMode::Unreliable,
    message_buffer_size: 8,
    packet_buffer_size: 8,
};
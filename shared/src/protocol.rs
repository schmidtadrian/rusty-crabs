use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ClientMessage {
    Join {
        name: String
    },
    ChatMessage {
        from: String,
        message: String
    },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ServerMessage {
    Position {
        x: f32,
        y: f32
    }
}
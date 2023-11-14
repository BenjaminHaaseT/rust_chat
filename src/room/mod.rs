//! Contains all of useful items for modeling the behavior of a chatroom
//!
// pub mod message;
use std::collections::HashMap;
use uuid::Uuid;
use tokio::sync::broadcast::{Sender as TokioBroadcastSender, Receiver as TokioBroadcastReceiver};
use async_std::channel::{Receiver as AsyncStdReceiver, Sender as AsyncStdSender};
use crate::*;

// Rough outline of needed ingredients for chatroom task
pub struct Chatroom {
    name: String,
    id: Uuid,
    peers: HashMap<Uuid, String>,
    // message_sender: TokioBroadcastSender<Message>,
    // message_receiver: TokioBroadcastReceiver<Message>,
    // client_receiver: AsyncStdReceiver<Client>,
    // client_disconnect_receiver: AsyncStdReceiver<(Client, AsyncStdSender<Empty>)>

}

pub struct Message {}

enum Empty {}




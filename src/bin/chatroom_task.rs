//! Module that encapsulates everything needed to create a new chatroom task by the main broker task
//!
//!

use std::collections::HashMap;
use uuid::Uuid;
use async_std::channel::{bounded, unbounded};
use tokio::sync::broadcast;

use chat_room::prelude::*;
use crate::server_error::prelude::*;

pub async fn chatroom_broker(client_receiver: AsyncStdReceiver<Client>, chatroom_shutdown: AsyncStdSender<NullEnum>, channel_buf_size: usize) -> Result<(), ServerError> {
    // Channel for receiving new messages from clients
    let (client_msg_sender, client_msg_receiver) = bounded::<Message>(channel_buf_size);

    // Channel for sending received messages from clients to clients,
    // `broadcast_msg_receiver` will be cloned and passed to the writing task for each client
    let (broadcast_msg_sender, broadcast_msg_receiver) = broadcast::channel::<Message>(channel_buf_size);

    // Channel for receiving harvesting disconnected clients
    let (client_disconnect_sender, client_disconnect_receiver) = unbounded::<(Client, TokioBroadcastReceiver<Message>)>();

    // Hashmap for keeping track of all clients
    let mut clients: HashMap<Uuid, Client> = HashMap::new();

    loop {
        todo!()
    }

    todo!()

}
use crate::messages::Message;
use std::sync::mpsc::Sender;
use crate::lapin::{
    BasicProperties,
    options::*,
};
use futures::Future;
use crate::messages::message_handler::MessageHandler;
use actix::fut::err;
use crate::server::ApiResult;

#[derive(Clone)]
pub struct MessageSender {
    channel: Sender<Message>
}

impl MessageSender {
    pub fn new(channel: Sender<Message>) -> Self {
        MessageSender { channel }
    }

    pub fn send(&self, message: Message) {
        let res = self.channel.send(message);
        if let Err(err) = res {
            error!("Failed to send message: {}", err);
        }
    }
    /*
        fn send_future(&mut self, message: Message) -> impl Future<Item=(), Error=()> {
            let key = message.get_routing_key();
            let payload = serde_json::to_string_pretty(&message).expect("Serialization error").into_bytes();

            self.handler.channel.basic_publish(EXCHANGE, key, payload, BasicPublishOptions::default(), BasicProperties::default())
                .and_then(move |_| {
                    info!("Message sent to {}", key);
                    futures::future::ok(())
                }).map_err(|e| {
                match e {
                    //lapin::Error::NotConnected => self.handler.reconnect().unwrap(),
                    _ => error!("Failed to send message {}", e)
                }
            })
        }
        */
}
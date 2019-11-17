use std::sync::mpsc::Receiver;
use std::thread;
use crate::messages::Message;
use crate::messages::message_handler::MessageHandler;
use crate::lapin::{
    BasicProperties,
    options::*,
};
use futures::Future;
use std::borrow::BorrowMut;

static EXCHANGE: &str = "amq.topic";

pub struct RabbitSender {
    channel: Receiver<Message>,
    handler: MessageHandler,
}

impl RabbitSender {
    pub fn new(url: &str, channel: Receiver<Message>) -> Result<Self, lapin::Error> {
        let handler = MessageHandler::new(url, "Sender")?;
        Ok(RabbitSender { channel, handler })
    }

    pub fn start(&mut self) {
        loop {
            let message = self.channel.recv().unwrap();
            let key = message.get_routing_key();
            let payload = serde_json::to_string_pretty(&message).expect("Serialization error").into_bytes();
            let mut res = self.try_to_send(key, payload.clone());
            while res.is_err() {
                res = self.try_to_send(key, payload.clone());
            }
        }
    }

    fn try_to_send(&mut self, key: &str, payload: Vec<u8>) -> Result<(), lapin::Error> {
        let res = self.handler.channel.basic_publish(EXCHANGE, key, payload, BasicPublishOptions::default(), BasicProperties::default()).wait();
        match res {
            Ok(()) => {
                info!("Message sent to {}", key);
                Ok(())
            }
            Err(lapin::Error::NotConnected) => {
                error!("Failed to send message to {} retrying", key);
                self.handler.reconnect()
            }
            Err(err) => {
                error!("Error sending message {} unable to continue", err);
                Ok(())
            }
        }
    }
}
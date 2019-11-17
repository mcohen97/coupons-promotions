use crate::messages::Message;
use crate::lapin::{
    BasicProperties,
    options::*,
};
use futures::Future;
use crate::messages::message_handler::MessageHandler;

static EXCHANGE: &str = "amq.topic";

#[derive(Clone)]
pub struct MessageSender {
    handler: MessageHandler,
    url: String
}

impl MessageSender {
    pub fn new(url: &str) -> Result<Self, lapin::Error> {
        let handler = MessageHandler::new(url, "Message sender")?;
        Ok(MessageSender { handler, url: url.to_string() })
    }

    pub fn send(&self, message: Message) {
        actix::spawn(self.send_future(message));
    }

    fn send_future(&self, message: Message) -> impl Future<Item=(), Error=()> {
        let key = message.get_routing_key();
        let payload = serde_json::to_string_pretty(&message).expect("Serialization error").into_bytes();

        self.handler.channel.basic_publish(EXCHANGE, key, payload, BasicPublishOptions::default(), BasicProperties::default())
            .and_then(move |_| {
                info!("Message sent to {}", key);
                futures::future::ok(())
            }).map_err(|e| error!("Failed to send message {}", e))
    }
}
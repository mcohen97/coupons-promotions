use crate::messages::Message;
use crate::server::ApiResult;
use lapin_futures::{Client, ConnectionProperties, Channel};
use crate::lapin::{
    BasicProperties, Connection, ConsumerDelegate,
    message::DeliveryResult,
    options::*,
    types::FieldTable,
};
use futures::Future;

static EXCHANGE: &str = "amq.topic";

#[derive(Clone)]
pub struct MessageSender {
    client: Client,
    channel: Channel,
}

impl MessageSender {
    pub fn new(url: &str) -> Result<Self, lapin::Error> {
        info!("Connecting to Rabbit MQ with url {}", url);
        let client = Client::connect(&url, ConnectionProperties::default()).wait()?;
        info!("Connected to Rabbit MQ");
        let channel = client.create_channel().wait()?;
        info!("Created channel with id {}", channel.id());
        Ok(MessageSender { client, channel })
    }

    pub fn send(&self, message: Message) {
        actix::spawn(self.send_future(message));
    }

    fn send_future(&self, message: Message) -> impl Future<Item=(), Error=()> {
        let key = message.get_routing_key();
        let payload = serde_json::to_string_pretty(&message).expect("Serialization error").into_bytes();

        self.channel.basic_publish(EXCHANGE, key, payload, BasicPublishOptions::default(), BasicProperties::default())
            .and_then(move |_| {
                info!("Message sent to {}", key);
                futures::future::ok(())
            }).map_err(|e| error!("Failed to send message {}", e))
    }
}
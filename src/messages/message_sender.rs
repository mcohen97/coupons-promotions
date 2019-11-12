use crate::messages::Message;
use crate::server::ApiResult;
use lapin_futures::{Client, ConnectionProperties};
use crate::lapin::{
    BasicProperties, Channel, Connection, ConsumerDelegate,
    message::DeliveryResult,
    options::*,
    types::FieldTable,
};
use futures::Future;

lazy_static! {
    static ref URL: String = std::env::var("RABBIT_URL").expect("Rabbit URL not set");
}
static EXCHANGE: &str = "";

pub struct MessageSender {}

impl MessageSender{
    pub fn send(message: Message) {
        actix::spawn(Self::send_future(message));
    }

    fn send_future(message: Message) -> impl Future<Item=(), Error=()>{
        let url = std::env::var("RABBIT_URL").expect("Rabbit URL not set");
        let key = message.get_routing_key();
        let payload = serde_json::to_string_pretty(&message).expect("Serialization error").into_bytes();
        Client::connect(&url, ConnectionProperties::default()).and_then(|client| {
            client.create_channel()
        }).and_then(move |channel| {
            info!("created channel with id: {}", channel.id());
            channel.basic_publish(EXCHANGE, key, payload, BasicPublishOptions::default(), BasicProperties::default())
        }).and_then(move |_| {
            info!("Message sent to {}", key);
            futures::future::ok(())
        }).map_err(|e| error!("Failed to send message {}", e))
    }
}
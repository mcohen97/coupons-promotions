use lapin_futures::{Client, ConnectionProperties, Channel};
use futures::Future;

static EXCHANGE: &str = "amq.topic";

#[derive(Clone)]
pub struct MessageHandler {
    pub name: &'static str,
    pub client: Client,
    pub channel: Channel,
}

impl MessageHandler {
    pub fn new(url: &str, name: &'static str) -> Result<Self, lapin::Error> {
        info!("{} connecting to Rabbit MQ", name);
        let client = Client::connect(&url, ConnectionProperties::default()).wait()?;
        info!("{} connected to Rabbit MQ", name);
        let channel = client.create_channel().wait()?;
        info!("{} created channel with id {}", name, channel.id());
        Ok(MessageHandler { client, channel, name })
    }
}
use lapin_futures::{Client, ConnectionProperties, Channel};
use futures::Future;

static EXCHANGE: &str = "amq.topic";

#[derive(Clone)]
pub struct MessageHandler {
    pub name: &'static str,
    pub url: String,
    pub client: Client,
    pub channel: Channel,
}

impl MessageHandler {
    pub fn new(url: &str, name: &'static str) -> Result<Self, lapin::Error> {
        let (client, channel) = Self::connect(url, name)?;
        let url = url.to_string();
        Ok(MessageHandler { client, channel, name, url })
    }

    pub fn reconnect(&mut self) -> Result<(), lapin::Error> {
        let (client, channel) = Self::connect(&self.url, self.name)?;
        self.client = client;
        self.channel = channel;

        Ok(())
    }

    fn connect(url: &str, name: &'static str) -> Result<(Client, Channel), lapin::Error> {
        info!("{} connecting to Rabbit MQ", name);
        let client = Client::connect(&url, ConnectionProperties::default()).wait()?;
        info!("{} connected to Rabbit MQ", name);
        let channel = client.create_channel().wait()?;
        info!("{} created channel with id {}", name, channel.id());

        Ok((client, channel))
    }
}
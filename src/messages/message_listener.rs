use futures::future::Future;
use lapin_futures::{Client, ConnectionProperties};
use crate::lapin::{
    BasicProperties, Channel, Connection, ConsumerDelegate,
    message::DeliveryResult,
    options::*,
    types::FieldTable,
};
use futures::Stream;
use crate::server::ApiError;

pub struct MessageListener {
    url: String
}

impl MessageListener {
    pub fn new<T>(url: T) -> Self where T: Into<String> {
        MessageListener { url: url.into() }
    }

    pub fn start(&self) -> impl Future<Item=(), Error=()> {
        Client::connect(&self.url, ConnectionProperties::default()).and_then(|client| {
            // create_channel returns a future that is resolved
            // once the channel is successfully created
            client.create_channel()
        }).and_then(|channel| {
            let id = channel.id();
            info!("created channel with id: {}", id);
            let ch = channel.clone();

            channel.basic_publish("", "hello", Vec::from("Send the load bitch".as_bytes()), BasicPublishOptions::default(), BasicProperties::default())
                .and_then(move |_| {
                    info!("published message");
                    channel.queue_declare("hello", QueueDeclareOptions::default(), FieldTable::default()).and_then(move |queue| {
                        info!("channel {} declared queue {}", id, "hello");

                        // basic_consume returns a future of a message
                        // stream. Any time a message arrives for this consumer,
                        // the for_each method would be called
                        channel.basic_consume(&queue, "my_consumer", BasicConsumeOptions::default(), FieldTable::default())
                    }).and_then(|stream| {
                        info!("got consumer stream");

                        stream.for_each(move |message| {
                            debug!("got message: {:?}", message);
                            info!("decoded message: {:?}", std::str::from_utf8(&message.data).unwrap());
                            ch.basic_ack(message.delivery_tag, false)
                        })
                    })
                })
        }).map_err(|e| ( error!("{}", e)))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        env_logger::init();
        let m = MessageListener::new("amqp://lyepjabq:DDt-OwA5B7XOCswfKgthGwA59yA1P73w@prawn.rmq.cloudamqp.com/lyepjabq");
        m.start();
    }
}
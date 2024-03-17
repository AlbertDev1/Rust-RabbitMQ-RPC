use futures_util::stream::StreamExt;
use lapin::{options::*, types::FieldTable, Connection, ConnectionProperties, Channel, BasicProperties};
use tokio::runtime::Runtime;

async fn handle_message(channel: Channel, delivery: lapin::message::Delivery) {
    println!("Received request: {:?}", delivery);
    let response = "Hello from Server ;-)".as_bytes().to_vec();

    if let Some(reply_to) = delivery.properties.reply_to().as_ref() {
        let reply_to_str = reply_to.as_str(); // Convert ShortString to &str

        if let Some(correlation_id) = delivery.properties.correlation_id().as_ref() {
            channel.basic_publish(
                "",
                reply_to_str, // Use the &str here
                BasicPublishOptions::default(),
                response,
                BasicProperties::default().with_correlation_id(correlation_id.clone()),
            )
                .await
                .expect("Failed to publish a message");
        }
    }

    channel.basic_ack(delivery.delivery_tag, BasicAckOptions::default())
        .await
        .expect("Failed to ack message");
}


fn main() {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        // Add your RabbitMQ Instance HostName, IP and Port (replace localhost:port)
        let addr = "amqp://guest:guest@localhost:32779/%2f"; // Update as needed
        let conn = Connection::connect(&addr, ConnectionProperties::default())
            .await
            .expect("Failed to connect to RabbitMQ");

        let channel = conn.create_channel().await.expect("Failed to create a channel");

        let _queue = channel.queue_declare(
            "rpc_queue",
            QueueDeclareOptions::default(),
            FieldTable::default(),
        ).await.expect("Failed to declare a queue");

        println!("Awaiting RPC requests");

        let mut consumer = channel.basic_consume(
            "rpc_queue",
            "my_consumer",
            BasicConsumeOptions::default(),
            FieldTable::default(),
        ).await.expect("Failed to consume");

        while let Some(delivery) = consumer.next().await {
            if let Ok((channel, delivery)) = delivery {
                handle_message(channel, delivery).await;
            }
        }
    });
}

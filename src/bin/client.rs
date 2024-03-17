use lapin::{
    options::*, types::FieldTable, BasicProperties, Connection, ConnectionProperties,
};
use tokio::runtime::Runtime;
use uuid::Uuid;
use futures_util::stream::StreamExt;

fn main() {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        // Add your RabbitMQ Instance HostName, IP and Port (replace localhost:port)
        let addr = "amqp://guest:guest@localhost:port/%2f"; // Update as needed
        let conn = Connection::connect(&addr, ConnectionProperties::default())
            .await
            .expect("Failed to connect to RabbitMQ");

        let channel = conn.create_channel().await.expect("Failed to create a channel");

        let payload = "Hello, RabbitMQ!".as_bytes().to_vec();
        let correlation_id = Uuid::new_v4().to_string();

        // Declare a queue for replies
        let queue = channel.queue_declare(
            "",
            QueueDeclareOptions {
                exclusive: true,
                ..QueueDeclareOptions::default()
            },
            FieldTable::default(),
        ).await.expect("Failed to declare a queue");

        let mut consumer = channel.basic_consume(
            &queue.name().as_str(),
            "my_consumer",
            BasicConsumeOptions::default(),
            FieldTable::default(),
        ).await.expect("Failed to create consumer");

        println!("Sent RPC request");
        channel.basic_publish(
            "",
            "rpc_queue",
            BasicPublishOptions::default(),
            payload,
            BasicProperties::default()
                .with_reply_to(queue.name().clone())
                .with_correlation_id(correlation_id.clone().into()),
        )
            .await
            .expect("Failed to publish a message");


        consumer.for_each(|delivery| async {
            let delivery = delivery.expect("Error in consumer");
            let (_, delivery_data) = delivery; // Destructure the tuple

            // Now use delivery_data to access the properties
            if delivery_data.properties.correlation_id().as_ref().map(|s| s.as_str()) == Some(correlation_id.as_str()) {
                println!("Received response: {:?}", std::str::from_utf8(&delivery_data.data).unwrap());
            }
        }).await;

    });
}

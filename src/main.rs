mod client;
use client::{Client, Message, MessageBatch};
use std::thread;
use std::time::Duration;

#[derive(Clone, Copy)]
struct SimpleBroker;
impl client::MqqtBroker for SimpleBroker {
    fn mqtt_publish(message_batch: MessageBatch) -> Option<usize> {
        thread::sleep(Duration::new(1, 0)); //simulated round trip latency
        message_batch.print_messages();
        Some(message_batch.batch_id)
    }
}

fn main() {

    let mut a_client: Client<SimpleBroker> = Client::new("device_1".into());

    //This just shows the threads not waiting on each other
    for num in 0..100 {
        let new_message = Message::new(format!("message {}.", num));

        a_client.send_telemetry(new_message);
        println!("sent telemetry");
        thread::sleep(Duration::new(0, 50_000_000));
    }
}

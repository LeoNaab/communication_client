mod client;
use client::{Client, Message, MessageBatch};
use std::thread;
use std::time::Duration;
#[derive(Clone, Copy)]
struct SimpleBroker;
impl client::MqqtBroker for SimpleBroker {
    fn mqtt_publish(message_batch: MessageBatch) -> Option<usize> {
        thread::sleep(Duration::new(1, 0));
        // println!("\n broker out: {:?}", message_batch);
        message_batch.print_messages();
        Some(message_batch.batch_id)
    }
}

fn main() {
    // let broker = SimpleBroker;

    let mut a_client: Client<SimpleBroker> = Client::new("device_1".into());

    for num in 0..100 {
        let new_message = Message::new(format!("message {}.", num));

        a_client.send_telemetry(new_message);
        println!("sent telemetry");
        thread::sleep(Duration::new(0, 50_000_000));
    }
}
//some device getting sensor measurements from some edge device, want to stream
//telemetry from device to some server
//logs, measurements any helpul data
//streaming over MQTT
//message brokers: Subscribe to topic to listen to messages
//topics are strings or regular expressions

// enum Data {
//     error(Error),
//     log(String),
// }
// struct Message {
//     data: Data,
//     date: u64,
// }

// //store values until received
// //unstable connection

// struct MessageBatch {
//     device_id : String,
//     messages : Vec<Message>,
//     batch_id : usize,
//     batch_date: usize,
// }

// fn mqtt_publish(batch: MessageBatch) -> Result<usize, Error> {
//     Err(Error)
// }

// fn send_telemetry(message: Message) {
//     //this is the front end function
//     //have to be careful not to mutate batch once trying to send it
//     //but that should only be a worry outside of this function scope

//     //store messages in a vector

//     //create batch and queue it to send
//     //
//     send_batch();

//     let mut unsent = true;
//     while (unsent) {

//     }

// }
// }

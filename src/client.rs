use std::collections::VecDeque;
use std::marker::PhantomData;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::SystemTime;

static MAX_BATCH: usize = 100;
//Simple stand-in for real mqtt
pub trait MqqtBroker {
    fn mqtt_publish(message_batch: MessageBatch) -> Option<usize>;
}
#[derive(Debug, Clone)]
pub struct Message {
    data: String,
    timestamp: SystemTime,
}

impl Message {
    pub fn new(data_in: String) -> Message {
        Message {
            data: data_in,
            timestamp: SystemTime::now(),
        }
    }

    pub fn print_message(&self) {
        println!("data: {}, timestamp: {:#?}", self.data, self.timestamp)
    }
}
#[derive(Debug, Clone)]
pub struct MessageBatch {
    device_id: String,
    messages: Vec<Message>,
    pub batch_id: usize,
}

impl MessageBatch {
    fn new(message: Message, device_id: String, batch_id: usize) -> MessageBatch {
        MessageBatch {
            device_id,
            messages: vec![message],
            batch_id,
        }
    }

    fn append_message(&mut self, message: Message) {
        self.messages.push(message);
    }

    pub fn print_messages(&self) {
        println!("Device ID: {}, Batch {}", self.device_id, self.batch_id);
        for message in &self.messages {
            message.print_message();
        }
    }

    pub fn len(&self) -> usize{
        self.messages.len()
    }
}

pub struct Client<T: MqqtBroker> {
    device_id: String,
    backlog_mutex: Arc<Mutex<VecDeque<MessageBatch>>>,
    batch_count: usize,
    join_handle: Option<thread::JoinHandle<()>>,
    broker: PhantomData<T>,
}

impl<T: MqqtBroker> Client<T> {
    //Interface
    pub fn new(device_id: String) -> Client<T> {
        Client {
            device_id,
            backlog_mutex: Arc::new(Mutex::new(VecDeque::<MessageBatch>::new())),
            batch_count: 0,
            join_handle: None,
            broker: PhantomData,
        }
    }
    pub fn send_telemetry(&mut self, message: Message) {
        let batch_id = self.get_batch_id();
        let mut backlog = self.backlog_mutex.lock().unwrap();
        println!("main thread has backlog");
        if backlog.is_empty() {
            //create new MessageBatch and append it to the backlog
            println!("spinning up thread");
            backlog.push_back(MessageBatch::new(message, self.device_id.clone(), batch_id));
            let new_back = self.backlog_mutex.clone();
            self.join_handle = Some(thread::spawn(move || Self::send_batches(new_back)));
        } else if backlog.len() == 1 || backlog.back().unwrap().len() >= MAX_BATCH {
            println!("Case 1");
            backlog.push_back(MessageBatch::new(message, self.device_id.clone(), batch_id));
        } else {
            backlog.back_mut().unwrap().append_message(message);
        }
    }

    fn get_batch_id(&mut self) -> usize {
        self.batch_count += 1;
        self.batch_count
    }

    fn send_batches(backlog_mutex: Arc<Mutex<VecDeque<MessageBatch>>>) {
        //spin off thread to print each value of the vector, one by one, then terminate the thread.
        let mut has_some = true;

        while has_some {
            let message_batch = {
                let backlog = backlog_mutex.lock().unwrap();
                backlog[0].clone()
            };

            let result = T::mqtt_publish(message_batch); //this assumes a failed publish returns None, which is not how it would be 
            {
                let mut backlog = backlog_mutex.lock().unwrap();
                if result.is_some() {
                    backlog.pop_front();
                }
                if backlog.is_empty() {
                    has_some = false;
                }
            }
        }
    }
}

impl<T: MqqtBroker> Drop for Client<T> {
    fn drop(&mut self) {
        let the_join_handle = self.join_handle.take();
        if let Some(join_h) = the_join_handle {
            let _ = join_h.join();
        }
    }
}

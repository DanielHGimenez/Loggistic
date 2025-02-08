use std::cell::{Cell, RefCell};
use crate::provider::api::Provider;
use crate::reader::api::Reader;
use futures::stream::select_all;
use std::sync::{Arc, Mutex};
use futures::SinkExt;
use tokio::{
    sync::{
        broadcast::Receiver,
        broadcast
    },
    task::JoinHandle,
    spawn
};
use tokio_stream::{
    wrappers::{
        errors::BroadcastStreamRecvError,
        BroadcastStream
    },
    StreamExt
};

pub struct LogStash {
    broadcasts: Cell<Vec<BroadcastStream<Vec<u8>>>>,
    executor: Option<JoinHandle<()>>
}

impl LogStash {
    pub fn new() -> LogStash {
        LogStash{
            broadcasts: Cell::from(Vec::new()),
            executor: None
        }
    }
}

impl Provider for LogStash {
    fn add_broadcast(&mut self, broadcast: Receiver<Vec<u8>>) {
        self.broadcasts.get_mut().push(BroadcastStream::new(broadcast));
    }

    fn start(&mut self) {
        let broadcasts = self.broadcasts.take();
        let mut stream = select_all(broadcasts);
        self.executor = Option::from(spawn(async move {
            while let Some(message) = stream.next().await {
                match message {
                    Ok(message) => {
                        println!("{}", String::from_utf8_lossy(&message));
                    }
                    Err(_) => {
                        log::error!("Error reading from log stream.");
                    }
                }
            };
        }));
    }

    fn stop(&mut self) {
        match &self.executor {
            Some(executor) => {
                executor.abort();
                self.executor = None;
            }
            _ => {}
        }
    }

    fn log(&self, msg: &[u8]) {
        println!("{}", String::from_utf8_lossy(msg))
    }
}

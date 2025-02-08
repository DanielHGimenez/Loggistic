use crate::reader::api::Reader;
use retry::delay::Fixed;
use retry::retry;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use tokio::{
    spawn,
    sync::broadcast::{channel, Receiver, Sender},
    task::JoinHandle
};

const FILE_NAME_OPTION: &str = "file";
const DEFAULT_BUFFER_SIZE: &str = "1024";

pub struct SimpleFile {
    file: File,
    tx: Sender<Vec<u8>>,
    rx: Receiver<Vec<u8>>,
    executor: Option<JoinHandle<()>>
}

impl SimpleFile {
    pub fn from(options: HashMap<String, String>) -> Result<SimpleFile, Box<dyn Error>> {
        let file_location = options.get(&FILE_NAME_OPTION.to_string()).ok_or("File must be specified")?;
        let buffer_size = options.get("buffer_size")
            .unwrap_or(&DEFAULT_BUFFER_SIZE.to_string())
            .parse::<usize>()?;
        let file = File::open(file_location)?;
        let (tx, rx) = channel::<Vec<u8>>(buffer_size);
        Ok(SimpleFile { file, tx, rx, executor: None })
    }
}

impl Reader for SimpleFile {
    fn broadcast(&self) -> Receiver<Vec<u8>> {
        self.tx.subscribe()
    }

    fn start(&mut self) {
        let file = self.file.try_clone().unwrap();
        let tx = self.tx.clone();
        self.executor = Option::from(spawn(async move {
            let mut reader = BufReader::new(&file);
            let mut buffer = Vec::<u8>::with_capacity(4096);
            loop {
                match retry(
                    Fixed::from_millis(0).take(3),
                    || reader.read_until(b'\n', &mut buffer)
                ) {
                    Ok(number_of_bytes) => {
                        if number_of_bytes > 0 {
                            match retry(
                                Fixed::from_millis(0).take(3),
                                || tx.send(buffer.clone())
                            ) {
                                Err(err) => {
                                    panic!("failed to send buffer: {}", err);
                                }
                                _ => {
                                    buffer.clear();
                                }
                            }
                        }
                    }
                    _ => panic!("failed to read from file")
                }
            }
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
}

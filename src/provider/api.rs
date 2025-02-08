use tokio::sync::broadcast::Receiver;
use crate::reader::api::Reader;

pub trait Provider {

    fn add_broadcast(&mut self, broadcast: Receiver<Vec<u8>>);

    fn start(&mut self);

    fn stop(&mut self);

    fn log(&self, msg: &[u8]);

}
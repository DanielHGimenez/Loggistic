use tokio::sync::broadcast;

pub trait Reader {

    fn broadcast(&self) -> broadcast::Receiver<Vec<u8>>;

    fn start(&mut self);

    fn stop(&mut self);

}

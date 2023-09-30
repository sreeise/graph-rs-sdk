use std::fmt::Debug;
pub use tokio::sync::watch::{channel, Receiver, Sender};

#[derive(Clone)]
pub struct AutomaticTokenRefresh<T> {
    rx: Receiver<T>,
}

impl<T: Clone + Debug + Send + Sync> AutomaticTokenRefresh<T> {
    pub fn new(init: T) -> (Sender<T>, AutomaticTokenRefresh<T>) {
        let (tx, mut rx) = channel(init);

        (tx, AutomaticTokenRefresh { rx })
    }

    pub async fn call(&mut self) {
        while self.rx.changed().await.is_ok() {
            println!("received = {:?}", *self.rx.borrow());
        }
    }
}

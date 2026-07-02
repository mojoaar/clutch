use tokio::sync::watch;

pub struct StreamCancelState {
    tx: watch::Sender<bool>,
}

impl StreamCancelState {
    pub fn new() -> Self {
        let (tx, _) = watch::channel(false);
        Self { tx }
    }

    pub fn cancel(&self) {
        let _ = self.tx.send(true);
    }

    pub fn reset_and_subscribe(&self) -> watch::Receiver<bool> {
        let _ = self.tx.send(false);
        self.tx.subscribe()
    }
}

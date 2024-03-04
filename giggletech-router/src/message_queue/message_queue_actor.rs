use tokio::sync::mpsc::UnboundedReceiver;

pub struct MessageQueueActor {
    receiver: UnboundedReceiver<()>,
}

impl MessageQueueActor {
    async fn run(&mut self) {
        while let Some(msg) = self.receiver.recv().await {
            todo!()
        }
    }

    
}

use tracing::{ info, Level };
use tracing_subscriber::FmtSubscriber;
use std::sync::Arc;

pub struct AppLogger {
    subscriber: Arc<FmtSubscriber>,
}

impl AppLogger {
    pub fn new() -> Self {
        let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .with_target(true)
        .finish();

        AppLogger { subscriber: Arc::new(subscriber) }
    }

    pub fn init_global_default(&self) {
        tracing::subscriber
            ::set_global_default(self.subscriber.clone())
            .expect("Failed to set the global tracing subscriber");
    }

    pub fn info(&self, message: &str) {
        // todo wrapper does not work as don't display the original module properly
        info!("{}", message);
    }
    
}

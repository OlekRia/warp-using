use infrastructure::StorageFactory;
use ports::Storable;

pub struct Application {
    pub port: u16,
    pub db: Option<Box<dyn Storable>>,
}

pub struct ApplicationBuilder {
    storable: Option<Box<dyn Storable>>,
}

impl ApplicationBuilder {
    pub fn new() -> Self {
        Self { storable: None }
    }

    pub fn with_store(mut self, store: Box<dyn Storable>) -> Self {
        self.storable = Some(store);
        self
    }

    pub fn build(self) -> Application {
        let store = self
            .storable
            .unwrap_or_else(|| StorageFactory::create_storable());

        Application {
            port: 3000,
            db: Some(store),
        }
    }
}

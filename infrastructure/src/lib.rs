use adapters::db::in_memory::Store;
use ports::Storable;

pub struct StorageFactory;

impl StorageFactory {
    pub fn create_storable() -> Box<dyn Storable> {
        #[cfg(test)]
        {
            println!("Creating Storable in test mode");
            Store::init()
        }

        #[cfg(not(test))]
        {
            println!("Creating Storable in production mode");
            Store::init()
        }
    }
}

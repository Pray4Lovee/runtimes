#[derive(Default)]
pub struct NetworkAdapters {
    pub list: Vec<Box<dyn NetworkAdapter>>,
}

pub trait NetworkAdapter {
    fn send(&self, payload: Vec<u8>) -> bool;
}

// Example stub adapters
pub struct EVMAdapter;
impl NetworkAdapter for EVMAdapter {
    fn send(&self, _payload: Vec<u8>) -> bool { true }
}

pub struct CosmosAdapter;
impl NetworkAdapter for CosmosAdapter {
    fn send(&self, _payload: Vec<u8>) -> bool { true }
}

pub struct SolanaAdapter;
impl NetworkAdapter for SolanaAdapter {
    fn send(&self, _payload: Vec<u8>) -> bool { true }
}

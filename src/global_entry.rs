use crate::prelude::*;

pub(crate) struct Entry {
    pub value: RwLock<Arc<dyn std::any::Any + Send + Sync>>,
    pub counter: AtomicUsize,
}
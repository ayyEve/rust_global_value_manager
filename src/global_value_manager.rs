use crate::prelude::*;

pub struct GlobalValueManager;
impl GlobalValueManager {
    pub fn update<T: 'static + Send + Sync>(new_value: Arc<T>) {
        let lock = MANAGER.read().unwrap();

        let id = TypeId::of::<T>();
        if let Some(entry) = lock.get(&id) {
            entry.counter.fetch_add(1, SeqCst);
            *entry.value.write() = new_value;
        } else {
            drop(lock);
            let mut lock = MANAGER.write().unwrap();

            lock.insert(
                id,
                Entry {
                    value: RwLock::new(new_value),
                    counter: AtomicUsize::new(0),
                },
            );
        }
    }

    pub fn get<T: 'static + Send + Sync>() -> Option<Arc<T>> {
        let id = TypeId::of::<T>();
        MANAGER
            .read()
            .unwrap()
            .get(&id)
            .and_then(|i| i.value.read().clone().downcast::<T>().ok())
    }

    pub fn get_mut<T: 'static + Send + Sync + Clone>() -> Option<GlobalValueMut<T>> {
        Self::get().map(|v| GlobalValueMut::new(v))
    }

    pub fn check<T: 'static + Send + Sync>(last: &mut usize) -> Option<Arc<T>> {
        let id = TypeId::of::<T>();

        let entry = MANAGER.read().unwrap();
        let entry = entry.get(&id)?;

        let current = entry.counter.load(SeqCst);
        if current > *last {
            *last = current;
            entry.value.read().clone().downcast::<T>().ok()
        } else {
            None
        }
    }
}

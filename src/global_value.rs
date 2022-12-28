use crate::prelude::*;

pub struct GlobalValue<T: Send + Sync> {
    counter: usize,
    value: Arc<T>,
}
impl<T: 'static + Send + Sync> GlobalValue<T> {
    pub fn new() -> Self {
        let id = TypeId::of::<T>();
        let entry = MANAGER.read().unwrap();

        let Some(entry) = entry.get(&id) else {
            let backtrace = std::backtrace::Backtrace::capture();
            let name = std::any::type_name::<T>();
            panic!("Value not initialized for {name}: {backtrace}")
        };

        let counter = entry.counter.load(SeqCst);
        let value = entry.value.read().clone().downcast::<T>().unwrap();

        Self { counter, value }
    }

    pub fn update(&mut self) -> bool {
        if let Some(new_val) = GlobalValueManager::check::<T>(&mut self.counter) {
            self.value = new_val;
            true
        } else {
            false
        }
    }
}
impl<T: Send + Sync> Deref for GlobalValue<T> {
    type Target = Arc<T>;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

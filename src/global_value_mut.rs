use crate::prelude::*;

pub struct GlobalValueMut<T: 'static + Send + Sync + Clone>(T);
impl<T: 'static + Send + Sync + Clone> GlobalValueMut<T> {
    pub fn new(val: Arc<T>) -> Self {
        Self(val.as_ref().clone())
    }
}
impl<T: 'static + Send + Sync + Clone> Deref for GlobalValueMut<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T: 'static + Send + Sync + Clone> DerefMut for GlobalValueMut<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl<T: 'static + Send + Sync + Clone> Drop for GlobalValueMut<T> {
    fn drop(&mut self) {
        GlobalValueManager::update(Arc::new(self.0.clone()))
    }
}

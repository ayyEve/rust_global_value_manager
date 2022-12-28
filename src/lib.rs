
mod global_value_manager;
mod global_value_mut;
mod global_value;
mod global_entry;
pub mod prelude;


pub use global_value::GlobalValue;
pub use global_value_mut::GlobalValueMut;
pub use global_value_manager::GlobalValueManager;

use prelude::*;
lazy_static::lazy_static! {
    pub(crate) static ref MANAGER: ShardedLock<HashMap<TypeId, Entry>> = Default::default();
}



mod test {
    #[allow(unused)]
    use super::prelude::*;

    #[test]
    fn test() {
        #[derive(Clone)]
        struct B(i32);

        GlobalValueManager::update(Arc::new(B(21)));
        let mut instance = GlobalValue::<B>::new();
        let mut instance2 = GlobalValue::<B>::new();
        // test initial value
        assert_eq!(instance.0, 21);
        assert_eq!(instance2.0, 21);

        GlobalValueManager::update(Arc::new(B(55)));

        // check internal value has not changed
        assert_eq!(instance.0, 21);
        assert_eq!(instance2.0, 21);

        // update value
        instance.update();
        instance2.update();

        // check internal value has changed
        assert_eq!(instance.0, 55);
        assert_eq!(instance2.0, 55);

        // test drop behaviour
        {
            let mut b = GlobalValueManager::get_mut::<B>().unwrap();
            b.0 = 500;
        }

        // check internal value has not changed
        assert_eq!(instance.0, 55);
        assert_eq!(instance2.0, 55);

        // update value
        instance.update();
        instance2.update();

        // check internal value has changed
        assert_eq!(instance.0, 500);
        assert_eq!(instance2.0, 500);
    }
}
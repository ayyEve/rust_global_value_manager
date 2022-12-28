pub(crate) use std::any::TypeId;
pub(crate) use std::collections::HashMap;
pub(crate) use std::ops::{Deref, DerefMut};
pub(crate) use std::sync::atomic::AtomicUsize;
pub(crate) use std::sync::atomic::Ordering::SeqCst;
pub(crate) use std::sync::Arc;

pub use crossbeam::sync::ShardedLock;
pub use parking_lot::RwLock;

pub use crate::global_value_manager::*;
pub use crate::global_value_mut::*;
pub use crate::global_value::*;
pub(crate) use crate::global_entry::Entry;
pub(crate) use crate::MANAGER;

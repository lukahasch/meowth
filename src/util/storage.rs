use std::{
    collections::HashMap,
    hash::Hash,
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
#[repr(transparent)]
pub struct Id<T>(Uuid, PhantomData<fn() -> T>);

#[allow(clippy::non_canonical_clone_impl)]
impl<T> Clone for Id<T> {
    fn clone(&self) -> Self {
        Id(self.0, PhantomData)
    }
}

impl<T> Copy for Id<T> {}

impl<T> PartialEq for Id<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T> Eq for Id<T> {}

impl<T> Hash for Id<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Storage<T> {
    storage: HashMap<Id<T>, T>,
}

impl<T> Default for Storage<T> {
    fn default() -> Self {
        Storage {
            storage: HashMap::new(),
        }
    }
}

impl<T> Storage<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, value: T) -> Id<T> {
        let id = Id(Uuid::new_v4(), PhantomData);
        self.storage.insert(id, value);
        id
    }
}

impl<T> Deref for Storage<T> {
    type Target = HashMap<Id<T>, T>;

    fn deref(&self) -> &Self::Target {
        &self.storage
    }
}

impl<T> DerefMut for Storage<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.storage
    }
}

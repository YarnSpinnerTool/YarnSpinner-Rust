//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner/Dialogue.cs>, which we split off into multiple files
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::{Arc, RwLock};
use yarn_slinger_core::prelude::*;

/// Provides a mechanism for storing and retrieving instances
/// of the [`YarnValue`] type.
///
/// ## Implementation notes
///
/// The interface has been changed to make use of our [`YarnValue`] type,
/// which is more domain specific than the semi-corresponding `Convertible`.
/// We also cannot use generics in this trait because we need to be able to clone this box.
pub trait VariableStorage: Debug + Send + Sync {
    fn clone_box(&self) -> Box<dyn VariableStorage + Send + Sync>;
    fn set(&mut self, name: String, value: YarnValue);
    fn get(&self, name: &str) -> Option<YarnValue>;
    fn clear(&mut self);
}

impl Clone for Box<dyn VariableStorage + Send + Sync> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

/// A simple concrete implementation of [`VariableStorage`]
/// that keeps all variables in memory.
#[derive(Debug, Clone, Default)]
pub struct MemoryVariableStore(HashMap<String, YarnValue>);

impl MemoryVariableStore {
    pub fn new() -> Self {
        Self::default()
    }
}

impl VariableStorage for MemoryVariableStore {
    fn clone_box(&self) -> Box<dyn VariableStorage + Send + Sync> {
        Box::new(self.clone())
    }

    fn set(&mut self, name: String, value: YarnValue) {
        self.0.insert(name, value);
    }

    fn get(&self, name: &str) -> Option<YarnValue> {
        self.0.get(name).cloned()
    }

    fn clear(&mut self) {
        self.0.clear();
    }
}

/// A [`MemoryVariableStore`] that can sync its copies across threads.
#[derive(Debug, Clone)]
pub struct SharedMemoryVariableStore(
    pub(crate) Arc<RwLock<Box<dyn VariableStorage + Send + Sync>>>,
);

impl VariableStorage for SharedMemoryVariableStore {
    fn clone_box(&self) -> Box<dyn VariableStorage + Send + Sync> {
        Box::new(self.clone())
    }

    fn set(&mut self, name: String, value: YarnValue) {
        self.0.write().unwrap().set(name, value);
    }

    fn get(&self, name: &str) -> Option<YarnValue> {
        self.0.read().unwrap().get(name)
    }

    fn clear(&mut self) {
        self.0.write().unwrap().clear();
    }
}

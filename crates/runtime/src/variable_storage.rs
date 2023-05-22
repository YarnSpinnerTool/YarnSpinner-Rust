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
    fn clone_shallow(&self) -> Box<dyn VariableStorage>;
    fn set(&mut self, name: String, value: YarnValue);
    fn get(&self, name: &str) -> Option<YarnValue>;
    fn clear(&mut self);
}

impl Clone for Box<dyn VariableStorage> {
    fn clone(&self) -> Self {
        self.clone_shallow()
    }
}

/// A simple concrete implementation of [`VariableStorage`]
/// that keeps all variables in memory.
#[derive(Debug, Clone, Default)]
pub struct MemoryVariableStore(Arc<RwLock<HashMap<String, YarnValue>>>);

impl MemoryVariableStore {
    pub fn new() -> Self {
        Self::default()
    }
}

impl VariableStorage for MemoryVariableStore {
    fn clone_shallow(&self) -> Box<dyn VariableStorage> {
        Box::new(self.clone())
    }

    fn set(&mut self, name: String, value: YarnValue) {
        self.0.write().unwrap().insert(name, value);
    }

    fn get(&self, name: &str) -> Option<YarnValue> {
        self.0.read().unwrap().get(name).cloned()
    }

    fn clear(&mut self) {
        self.0.write().unwrap().clear();
    }
}

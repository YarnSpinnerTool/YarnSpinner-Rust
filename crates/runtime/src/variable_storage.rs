//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner/Dialogue.cs>, which we split off into multiple files
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::{Arc, RwLock};
use thiserror::Error;
use yarn_slinger_core::prelude::*;

#[allow(missing_docs)]
pub type Result<T> = std::result::Result<T, VariableStorageError>;

/// Provides a mechanism for storing and retrieving instances
/// of the [`YarnValue`] type.
///
/// ## Implementation notes
///
/// The interface has been changed to make use of our [`YarnValue`] type,
/// which is more domain specific than the semi-corresponding `Convertible`.
/// We also cannot use generics in this trait because we need to be able to clone this box.
pub trait VariableStorage: Debug + Send + Sync {
    /// Creates a shallow clone of this variable storage, i.e. a clone that
    /// shares the same underlying storage and will thus be perfectly in sync
    /// with the original instance.
    fn clone_shallow(&self) -> Box<dyn VariableStorage>;
    /// Sets the value of a variable. Must fail with a [`VariableStorageError::InvalidVariableName`] if the variable name does not start with a `$`.
    fn set(&mut self, name: String, value: YarnValue) -> Result<()>;
    /// Gets the value of a variable. Must fail with a [`VariableStorageError::InvalidVariableName`] if the variable name does not start with a `$`.
    /// If the variable is not defined, must fail with a [`VariableStorageError::VariableNotFound`].
    fn get(&self, name: &str) -> Result<YarnValue>;
    /// Returns `true` if the variable is defined, `false` otherwise.
    fn contains(&self, name: &str) -> bool {
        self.get(name).is_ok()
    }
    /// Extends this variable storage with the given values. Must fail with a [`VariableStorageError::InvalidVariableName`] if any of the variable names do not start with a `$`.
    /// Existing variables must be overwritten.
    fn extend(&mut self, values: HashMap<String, YarnValue>) -> Result<()>;
    /// Returns a map of all variables in this variable storage.
    fn variables(&self) -> HashMap<String, YarnValue>;
    /// Clears all variables in this variable storage.
    fn clear(&mut self);
}

impl Extend<(String, YarnValue)> for Box<dyn VariableStorage> {
    fn extend<T: IntoIterator<Item = (String, YarnValue)>>(&mut self, iter: T) {
        let hash_map = iter.into_iter().collect();
        VariableStorage::extend(self.as_mut(), hash_map)
            .unwrap_or_else(|e| panic!("Failed to extend variable storage with values: {e}",));
    }
}

#[allow(missing_docs)]
#[derive(Debug, Error)]
pub enum VariableStorageError {
    #[error("{name} is not a valid variable name: Variable names must start with a '$'. (Did you mean to use '${name}'?)")]
    InvalidVariableName { name: String },
    #[error("Variable name {name} is not defined")]
    VariableNotFound { name: String },
    #[error("Internal variable storage error: {error}")]
    InternalError {
        error: Box<dyn std::error::Error + Send + Sync>,
    },
}

impl Clone for Box<dyn VariableStorage> {
    fn clone(&self) -> Self {
        self.clone_shallow()
    }
}

/// A simple concrete implementation of [`VariableStorage`] that keeps all variables in memory.
#[derive(Debug, Clone, Default)]
pub struct MemoryVariableStorage(Arc<RwLock<HashMap<String, YarnValue>>>);

impl MemoryVariableStorage {
    /// Creates a new empty `MemoryVariableStorage`.
    pub fn new() -> Self {
        Self::default()
    }
}

impl VariableStorage for MemoryVariableStorage {
    fn clone_shallow(&self) -> Box<dyn VariableStorage> {
        Box::new(self.clone())
    }

    fn set(&mut self, name: String, value: YarnValue) -> Result<()> {
        Self::validate_name(&name)?;
        self.0.write().unwrap().insert(name, value);
        Ok(())
    }

    fn get(&self, name: &str) -> Result<YarnValue> {
        Self::validate_name(name)?;
        self.0.read().unwrap().get(name).cloned().ok_or_else(|| {
            VariableStorageError::VariableNotFound {
                name: name.to_string(),
            }
        })
    }

    fn extend(&mut self, values: HashMap<String, YarnValue>) -> Result<()> {
        for name in values.keys() {
            Self::validate_name(name)?;
        }
        self.0.write().unwrap().extend(values);
        Ok(())
    }

    fn variables(&self) -> HashMap<String, YarnValue> {
        self.0.read().unwrap().clone()
    }

    fn clear(&mut self) {
        self.0.write().unwrap().clear();
    }
}

impl MemoryVariableStorage {
    fn validate_name(name: impl AsRef<str>) -> Result<()> {
        let name = name.as_ref();
        if name.starts_with('$') {
            Ok(())
        } else {
            Err(VariableStorageError::InvalidVariableName {
                name: name.to_string(),
            })
        }
    }
}

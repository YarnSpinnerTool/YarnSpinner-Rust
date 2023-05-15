//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner/Dialogue.cs>, which we split off into multiple files
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::{Arc, RwLock};
use yarn_slinger_core::prelude::*;

pub trait LineProvider: Debug + Send + Sync {
    fn clone_shallow(&self) -> Box<dyn LineProvider + Send + Sync>;
    fn get_line(&self, id: &LineId) -> Option<String>;
    fn set_language_code(&mut self, language_code: String);
}

impl Clone for Box<dyn LineProvider + Send + Sync> {
    fn clone(&self) -> Self {
        self.clone_shallow()
    }
}

#[derive(Debug, Clone, Default)]
pub struct StringTableLineProvider {
    string_table: Arc<RwLock<HashMap<LineId, String>>>,
    language_code: Arc<RwLock<Option<String>>>,
}

impl StringTableLineProvider {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_string_table(string_table: HashMap<LineId, String>) -> Self {
        Self {
            string_table: Arc::new(RwLock::new(string_table)),
            language_code: Arc::new(RwLock::new(None)),
        }
    }

    pub fn set_string_table(&mut self, string_table: HashMap<LineId, String>) {
        *self.string_table.write().unwrap() = string_table;
    }
}

impl LineProvider for StringTableLineProvider {
    fn clone_shallow(&self) -> Box<dyn LineProvider + Send + Sync> {
        Box::new(self.clone())
    }

    fn get_line(&self, id: &LineId) -> Option<String> {
        self.string_table.read().unwrap().get(id).cloned()
    }

    fn set_language_code(&mut self, language_code: String) {
        self.language_code.write().unwrap().replace(language_code);
    }
}

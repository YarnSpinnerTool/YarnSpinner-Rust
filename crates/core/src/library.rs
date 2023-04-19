//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner/Library.cs>

mod yarn_fn;

pub use yarn_fn::*;

use crate::prelude::Value;
use std::collections::HashMap;

/// A collection of functions that can be called from Yarn scripts.
///
/// You do not create instances of this class yourself. The [`Dialogue`]
/// class creates one for you, and you can access it through the
/// [`Library`] property.
#[derive(Debug, Clone, Default)]
pub struct Library {
    /// The functions that are available to Yarn scripts.
    functions: HashMap<String, Box<dyn YarnFn>>,
}

impl Library {
    fn add<Marker, F>(&mut self, name: &str, function: F)
    where
        Marker: 'static,
        F: YarnFnWithMarker<Marker> + 'static,
        F::Out: Into<Value> + 'static + Clone,
    {
        let wrapped = YarnFnWrapper::new(function);
        self.functions.insert(name.to_string(), Box::new(wrapped));
    }

    fn get(&self, name: &str) -> Option<&dyn YarnFn> {
        self.functions.get(name).map(|f| f.as_ref())
    }
}

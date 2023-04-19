//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner/Library.cs>

//mod yarn_fn;

//pub use yarn_fn::*;

use std::collections::HashMap;

/// A collection of functions that can be called from Yarn scripts.
///
/// You do not create instances of this class yourself. The [`Dialogue`]
/// class creates one for you, and you can access it through the
/// [`Library`] property.
pub struct Library {
    /// The functions that are available to Yarn scripts.
    functions: HashMap<String, ()>,
}

pub enum MarkupValue {
    Integer(i32), // TODO: argue about size. In C# float(single) and int(32) are used.
    Float(f32),   // short is f16, but that doesnt even exist in rust?
    String(Into<String>),
    Bool(bool),
}

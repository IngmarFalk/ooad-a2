/// All traits in this file can be implemented without any methods.
///
/// E.g:
/// ```rust
/// struct ExampleView {}
///
/// impl View for ExampleView {}
///
/// struct ExampleModel {}
///
/// impl Model for ExampleModel {}
///
/// struct ExampleController {}
///
/// impl Controller for ExampleController {}
/// ```

/// All Domain models implement this trait.
pub trait Model {}

/// All ui/ux related structs implement this trait.
pub trait View {}

/// All Controllers implement this trait.
pub trait Controller {}

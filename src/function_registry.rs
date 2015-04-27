use sass_rs::sass_function::SassFunction;

/// Provides a list of sass signatures with the functions that implement them.
pub trait FunctionRegistry<T> {
    fn registry(t:&T) -> Vec<(&'static str,Box<SassFunction>)>;
}

// The "Captures trick". It is needed for lifetime binding in hidden/opaque types.
pub trait Capture<T> {}
impl<T, R> Capture<T> for R where R: ?Sized {}

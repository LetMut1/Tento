use std::marker::PhantomData;
pub mod environment_configuration;
pub struct Loader<S> {
    _subject: PhantomData<S>,
}

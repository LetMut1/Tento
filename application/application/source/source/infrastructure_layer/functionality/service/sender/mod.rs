pub mod email;
use std::marker::PhantomData;
pub struct Sender<S> {
    _subject: PhantomData<S>,
}

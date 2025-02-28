mod email;
pub use self::email::Email;
use std::marker::PhantomData;
pub struct Sender<S> {
    _subject: PhantomData<S>,
}

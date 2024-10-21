mod email;
use std::marker::PhantomData;
pub use self::email::Email;
pub struct Sender<S> {
    _subject: PhantomData<S>,
}

use std::marker::PhantomData;

pub use super::application_user___authorization::authorize_by_first_step::AuthorizeByFirstStep;

pub struct ActionProcessor<S> {
    _subject: PhantomData<S>,
}

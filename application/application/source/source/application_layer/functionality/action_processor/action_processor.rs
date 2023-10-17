use std::marker::PhantomData;

pub use super::application_user___authorization::authorize_by_first_step::ApplicationUser__Authorization___AuthorizeByFirstStep;
pub use super::application_user___authorization::authorize_by_last_step::ApplicationUser__Authorization___AuthorizeByLastStep;

pub struct ActionProcessor<S> {
    _subject: PhantomData<S>,
}

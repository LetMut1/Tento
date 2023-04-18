use std::marker::PhantomData;

pub struct SendingOpportunityResolver<S> {
    _subject: PhantomData<S>
}
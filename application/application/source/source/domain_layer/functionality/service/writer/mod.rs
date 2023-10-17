pub mod action_round_register;

use std::marker::PhantomData;

pub struct Writer<S> {
    _subject: PhantomData<S>,
}

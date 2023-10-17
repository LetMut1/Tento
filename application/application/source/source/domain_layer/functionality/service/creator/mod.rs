pub mod action_round_register__context;

use std::marker::PhantomData;

pub struct Creator<S> {
    _subject: PhantomData<S>,
}

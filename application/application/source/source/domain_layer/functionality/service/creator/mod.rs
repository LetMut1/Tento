pub mod action_round_register__context___creator;

use std::marker::PhantomData;

pub struct Creator<S> {
    _subject: PhantomData<S>,
}

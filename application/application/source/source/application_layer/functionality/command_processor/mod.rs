pub mod create_fixtures;
pub mod run_server;
pub mod remove_incomplite_state;

use std::marker::PhantomData;

pub struct CommandProcessor<S> {
    _subject: PhantomData<S>,
}

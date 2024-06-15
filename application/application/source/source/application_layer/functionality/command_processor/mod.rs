pub mod create_fixtures;
pub mod remove_incomplite_state;
pub mod run_server;
use std::marker::PhantomData;
pub struct CommandProcessor<S> {
    _subject: PhantomData<S>,
}

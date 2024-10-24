mod create_fixtures;
mod remove_incomplite_state;
mod run_server;
pub use self::{
    create_fixtures::CreateFixtures,
    remove_incomplite_state::RemoveIncompliteState,
    run_server::RunServer,
};
use std::marker::PhantomData;
pub struct CommandProcessor<S> {
    _subject: PhantomData<S>,
}

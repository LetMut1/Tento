use std::marker::PhantomData;

pub use super::create_fixtures::CreateFixtures;
pub use super::run_server::RunServer;

pub struct CommandProcessor<S> {
    _subject: PhantomData<S>,
}

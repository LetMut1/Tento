pub mod create_fixtures;
pub mod run_server;

use std::marker::PhantomData;

pub use self::create_fixtures::CreateFixtures;
pub use self::run_server::RunServer;

pub struct CommandProcessor<S> {
    _subject: PhantomData<S>,
}

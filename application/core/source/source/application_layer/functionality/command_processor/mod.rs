mod create_fixtures;
mod resolve_incomplite_state;
mod run_server;
pub use self::{
    create_fixtures::CreateFixtures,
    resolve_incomplite_state::ResolveIncompliteState,
    run_server::RunServer,
};
use std::marker::PhantomData;
pub const TOKIO_CONFIGURATION_ERROR_MESSAGE_1: &'static str = "The vaule of 'system.tokio.worker_threads_quantity' is equal to zero.";
pub const TOKIO_CONFIGURATION_ERROR_MESSAGE_2: &'static str = "The vaule of 'system.tokio.worker_threads_quantity' is not equal to the quantity of elements in the value of 'system.tokio.affinited_cores'.";
pub const TOKIO_CONFIGURATION_ERROR_MESSAGE_3: &'static str = "The vaule of 'system.tokio.worker_thread_stack_size' is less than 2MiB.";
pub const TWO_MIB: usize = 1024 * 1024 * 2;
pub struct CommandProcessor<S> {
    _subject: PhantomData<S>,
}

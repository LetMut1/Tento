use super::Spawner;
use crate::infrastructure_layer::{
    data::{
        control_type::TokioNonBlockingTask,
        alternative_workflow::AlternativeWorkflow,
    },
    functionality::service::logger::Logger,
};
use std::{
    future::Future,
    marker::Send,
};
use tokio::task::JoinHandle;
impl Spawner<TokioNonBlockingTask> {
    pub fn spawn_into_background<F, T>(future: F) -> ()
    where
        F: Future<Output = Result<T, AlternativeWorkflow>> + Send + 'static,
    {
        tokio::spawn(
            async move {
                if let Err(error) = future.await {
                    Logger::<AlternativeWorkflow>::log(&error);
                }
                return ();
            },
        );
    }
    pub fn spawn_processed<F>(future: F) -> JoinHandle<<F as Future>::Output>
    where
        F: Future + Send + 'static,
        <F as Future>::Output: Send + 'static,
    {
        return tokio::spawn(future);
    }
}

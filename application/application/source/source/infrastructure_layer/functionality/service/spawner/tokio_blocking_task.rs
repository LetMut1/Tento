use super::Spawner;
use crate::infrastructure_layer::{
    data::{
        alternative_workflow::AlternativeWorkflow,
        control_type::TokioBlockingTask,
    },
    functionality::service::logger::Logger,
};
use std::marker::Send;
use tokio::task::JoinHandle;
impl Spawner<TokioBlockingTask> {
    pub fn spawn_into_background<F, T>(closure: F) -> ()
    where
        F: FnOnce() -> Result<T, AlternativeWorkflow> + Send + 'static,
        T: Send + 'static,
    {
        tokio::task::spawn_blocking(
            move || -> () {
                if let Err(alternative_workflow) = closure() {
                    Logger::<AlternativeWorkflow>::log(&alternative_workflow);
                }
                return ();
            },
        );
    }
    pub fn spawn_processed<F, R>(closure: F) -> JoinHandle<R>
    where
        F: FnOnce() -> R + Send + 'static,
        R: Send + 'static,
    {
        return tokio::task::spawn_blocking(closure);
    }
}

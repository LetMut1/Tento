use super::Spawner;
use crate::infrastructure_layer::{
    data::{
        auditor::Auditor,
        control_type::TokioNonBlockingTask,
        error::Error,
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
        F: Future<Output = Result<T, Auditor<Error>>> + Send + 'static,
    {
        let future = async move {
            if let Err(error_auditor) = future.await {
                Logger::<Auditor<Error>>::log(&error_auditor);
            }
            return ();
        };
        tokio::spawn(future);
    }
    pub fn spawn_processed<F>(future: F) -> JoinHandle<<F as Future>::Output>
    where
        F: Future + Send + 'static,
        <F as Future>::Output: Send + 'static,
    {
        return tokio::spawn(future);
    }
}

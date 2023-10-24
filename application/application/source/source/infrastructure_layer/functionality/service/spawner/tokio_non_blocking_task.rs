use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use std::future::Future;
use std::marker::Send;
use tokio::task::JoinHandle;
use tokio::spawn;
use crate::infrastructure_layer::functionality::service::logger::Logger;
use super::Spawner;

pub use crate::infrastructure_layer::data::control_type::TokioNonBlockingTask;

impl Spawner<TokioNonBlockingTask> {
    pub fn spawn_into_background<F, T>(future: F) -> ()
    where
        F: Future<Output = Result<T, ErrorAuditor>> + Send + 'static,
    {
        spawn(
            async move {
                if let Err(error) = future.await {
                    Logger::<ErrorAuditor>::log(&error);
                }

                return ();
            }
        );
    }

    pub fn spawn_processed<F, T>(future: F) -> JoinHandle<<F as Future>::Output>
    where
        F: Future + Send + 'static,
        <F as Future>::Output: Send + 'static,
    {
        return spawn(future);
    }
}
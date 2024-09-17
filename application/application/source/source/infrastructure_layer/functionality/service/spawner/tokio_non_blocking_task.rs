use super::Spawner;
use crate::infrastructure_layer::functionality::service::logger::Logger;
use aggregate_error::AggregateError;
use std::future::Future;
use tokio::task::JoinHandle;
pub struct TokioNonBlockingTask;
impl Spawner<TokioNonBlockingTask> {
    pub fn spawn_into_background<F, T>(future: F) -> ()
    where
        F: Future<Output = Result<T, AggregateError>> + Send + 'static,
    {
        tokio::spawn(
            async move {
                if let Err(aggregate_error) = future.await {
                    Logger::<AggregateError>::log(&aggregate_error);
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

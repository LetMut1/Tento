use super::Spawner;
use crate::infrastructure_layer::functionality::service::logger::Logger;
use crate::infrastructure_layer::data::aggregate_error::AggregateError;
use tokio::task::JoinHandle;
pub struct TokioBlockingTask;
impl Spawner<TokioBlockingTask> {
    pub fn spawn_into_background<F, T>(closure: F) -> ()
    where
        F: FnOnce() -> Result<T, AggregateError> + Send + 'static,
        T: Send + 'static,
    {
        tokio::task::spawn_blocking(
            move || -> () {
                if let Result::Err(aggregate_error) = closure() {
                    Logger::<AggregateError>::log(&aggregate_error);
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

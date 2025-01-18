use super::Spawner;
use crate::infrastructure_layer::{
    data::aggregate_error::AggregateError,
    functionality::service::logger::Logger,
};
use tokio::task::JoinHandle;
pub struct TokioBlockingTask;
impl Spawner<TokioBlockingTask> {
    pub fn spawn_into_background<T>(
        closure: impl FnOnce() -> Result<T, AggregateError> + Send + 'static,
    ) -> ()
    where
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
        return ();
    }
    pub fn spawn_processed<R>(
        closure: impl FnOnce() -> R + Send + 'static,
    ) -> JoinHandle<R>
    where
        R: Send + 'static,
    {
        return tokio::task::spawn_blocking(closure);
    }
}

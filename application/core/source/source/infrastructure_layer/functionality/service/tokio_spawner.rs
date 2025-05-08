use {
    crate::infrastructure_layer::{
        data::aggregate_error::AggregateError,
        functionality::service::logger::Logger,
    },
    std::future::Future,
    tokio::task::JoinHandle,
};
pub struct TokioSpawner;
impl TokioSpawner {
    pub fn spawn_non_blocking_task_into_background<T>(future: impl Future<Output = Result<T, AggregateError>> + Send + 'static) -> () {
        tokio::spawn(
            async move {
                if let Result::Err(aggregate_error) = future.await {
                    Logger::<AggregateError>::log(&aggregate_error);
                }
                return ();
            },
        );
        return ();
    }
    pub fn spawn_non_blocking_task_processed<F>(future: F) -> JoinHandle<<F as Future>::Output>
    where
        F: Future + Send + 'static,
        <F as Future>::Output: Send + 'static,
    {
        return tokio::spawn(future);
    }
    pub fn spawn_blocking_task_into_background<T>(closure: impl FnOnce() -> Result<T, AggregateError> + Send + 'static) -> ()
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
    pub fn spawn_blocking_task_processed<R>(closure: impl FnOnce() -> R + Send + 'static) -> JoinHandle<R>
    where
        R: Send + 'static,
    {
        return tokio::task::spawn_blocking(closure);
    }
}

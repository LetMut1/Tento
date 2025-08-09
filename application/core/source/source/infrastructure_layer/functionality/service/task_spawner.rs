use {
    crate::infrastructure_layer::{
        data::aggregate_error::AggregateError,
        functionality::service::logger::Logger,
    },
    std::{future::Future, num::NonZero, time::Duration},
    tokio::{
        sync::oneshot::Receiver,
        task::JoinHandle,
    },
};
pub struct TaskSpawner;
impl TaskSpawner {
    pub fn spawn_tokio_non_blocking_task_into_background<T>(future: impl Future<Output = Result<T, AggregateError>> + Send + 'static) -> () {
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
    pub fn spawn_tokio_non_blocking_task_into_background_repeatable<T, E>(
        repeatable_for_error: RepeatableForError,
        closure: impl Fn() -> T + Send + 'static,
    ) -> ()
    where
        T: Future<Output = Result<E, AggregateError>> + Send + 'static,
        E: Send
    {
        tokio::spawn(
            async move {
                let quantity = repeatable_for_error.quantity.get();
                let interval_seconds_quantity = repeatable_for_error.interval_seconds_quantity.get();
                'a: for quantity_ in 1..=quantity {
                    match closure().await {
                        Result::Ok(_) => break 'a,
                        Result::Err(aggregate_error) => {
                            if quantity_ < quantity {
                                tokio::time::sleep(Duration::from_secs(interval_seconds_quantity)).await;
                                continue 'a;
                            } else {
                                Logger::<AggregateError>::log(&aggregate_error);
                                break 'a;
                            }
                        }
                    }
                }
                return ();
            },
        );
        return ();
    }
    pub fn spawn_tokio_non_blocking_task_processed<F>(future: F) -> JoinHandle<<F as Future>::Output>
    where
        F: Future + Send + 'static,
        <F as Future>::Output: Send + 'static,
    {
        return tokio::spawn(future);
    }
    pub fn spawn_rayon_task_processed<T>(closure: impl FnOnce() -> Result<T, AggregateError> + Send + 'static) -> Receiver<Result<T, AggregateError>>
    where
        T: Send + 'static,
    {
        let (sender, receiver) = tokio::sync::oneshot::channel::<Result<T, AggregateError>>();
        rayon::spawn_fifo(
            move || -> () {
                let _ = sender.send(closure());
                return ();
            },
        );
        return receiver;
    }
}
pub struct RepeatableForError {
    pub quantity: NonZero<usize>,
    pub interval_seconds_quantity: NonZero<u64>,
}
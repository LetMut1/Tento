use super::Spawner;
use crate::infrastructure_layer::data::auditor::Auditor;
use crate::infrastructure_layer::data::error::Error;
use crate::infrastructure_layer::functionality::service::logger::Logger;
use std::marker::Send;
use tokio::task::JoinHandle;

pub use crate::infrastructure_layer::data::control_type::TokioBlockingTask;

impl Spawner<TokioBlockingTask> {
    pub fn spawn_into_background<F, T>(closure: F) -> ()
    where
        F: FnOnce() -> Result<T, Auditor<Error>> + Send + 'static,
        T: Send + 'static,
    {
        tokio::task::spawn_blocking(|| -> () {
            if let Err(error_auditor) = closure() {
                Logger::<Auditor<Error>>::log(&error_auditor);
            }

            return ();
        });
    }

    pub fn spawn_processed<F, R>(closure: F) -> JoinHandle<R>
    where
        F: FnOnce() -> R + Send + 'static,
        R: Send + 'static,
    {
        return tokio::task::spawn_blocking(closure);
    }
}

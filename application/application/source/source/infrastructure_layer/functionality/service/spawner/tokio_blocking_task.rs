use crate::infrastructure_layer::data::auditor::Auditor;
use std::marker::Send;
use tokio::task::JoinHandle;
use crate::infrastructure_layer::functionality::service::logger::Logger;
use super::Spawner;
use crate::infrastructure_layer::data::error::Error;

pub use crate::infrastructure_layer::data::control_type::TokioBlockingTask;

impl Spawner<TokioBlockingTask> {
    pub fn spawn_into_background<F, T>(closure: F) -> ()
    where
        F: FnOnce() -> Result<T, Auditor<Error>> + Send + 'static,
        T: Send + 'static,
    {
        tokio::task::spawn_blocking(
            || -> _ {
                if let Err(error_auditor) = closure() {
                    Logger::<Auditor<Error>>::log(&error_auditor);
                }

                return ();
            }
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
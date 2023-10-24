use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use std::marker::Send;
use tokio::task::JoinHandle;
use tokio::task::spawn_blocking;
use crate::infrastructure_layer::functionality::service::logger::Logger;
use super::Spawner;

pub use crate::infrastructure_layer::data::control_type::TokioBlockingTask;

impl Spawner<TokioBlockingTask> {
    pub fn spawn_into_background<F, T>(closure: F) -> ()
    where
        F: FnOnce() -> Result<T, ErrorAuditor> + Send + 'static,
        T: Send + 'static,
    {
        spawn_blocking(
            || -> _ {
                if let Err(error) = closure() {
                    Logger::<ErrorAuditor>::log(&error);
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
        return spawn_blocking(closure);
    }
}
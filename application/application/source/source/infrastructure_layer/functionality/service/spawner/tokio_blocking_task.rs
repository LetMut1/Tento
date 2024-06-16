use super::Spawner;
use crate::infrastructure_layer::{
    data::{
        auditor::Auditor,
        control_type::TokioBlockingTask,
        error::Error,
    },
    functionality::service::logger::Logger,
};
use std::marker::Send;
use tokio::task::JoinHandle;
impl Spawner<TokioBlockingTask> {
    pub fn spawn_into_background<F, T>(closure: F) -> ()
    where
        F: FnOnce() -> Result<T, Auditor<Error>> + Send + 'static,
        T: Send + 'static,
    {
        let closure = move || -> () {
            if let Err(error_auditor) = closure() {
                Logger::<Auditor<Error>>::log(&error_auditor);
            }
            return ();
        };

        tokio::task::spawn_blocking(closure);
    }
    pub fn spawn_processed<F, R>(closure: F) -> JoinHandle<R>
    where
        F: FnOnce() -> R + Send + 'static,
        R: Send + 'static,
    {
        return tokio::task::spawn_blocking(closure);
    }
}

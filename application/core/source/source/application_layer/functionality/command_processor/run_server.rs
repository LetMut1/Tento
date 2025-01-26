use super::CommandProcessor;
use crate::infrastructure_layer::{
    data::{
        aggregate_error::AggregateError,
        environment_configuration::EnvironmentConfiguration,
    },
    functionality::service::{
        http_server::HttpServer,
        loader::Loader,
    },
};
use std::sync::OnceLock;
use tokio::runtime::{
    Builder as RuntimeBuilder,
    Runtime,
};
use tracing::Level;
use tracing_appender::non_blocking::{
    NonBlocking,
    NonBlockingBuilder,
    WorkerGuard,
};
#[cfg(feature = "logging_to_file")]
use tracing_appender::rolling::{
    RollingFileAppender,
    Rotation,
};
pub use crate::infrastructure_layer::data::environment_configuration::run_server::RunServer;
use tracing_subscriber::FmtSubscriber;
static ENVIRONMENT_CONFIGURATION: OnceLock<EnvironmentConfiguration<RunServer>> = OnceLock::new();
impl CommandProcessor<RunServer> {
    pub fn process<'a>(environment_configuration_file_path: &'a str) -> Result<(), AggregateError> {
        let _worker_guard;
        let environment_configuration = Self::initialize_environment(environment_configuration_file_path)?;
        #[cfg(feature = "logging_to_file")]
        {
            _worker_guard = Self::initialize_logging_to_fileger(environment_configuration)?;
        }
        #[cfg(not(feature = "logging_to_file"))]
        {
            _worker_guard = Self::initialize_stdout_logger();
        }
        let runtime = Self::initialize_runtime(environment_configuration)?;
        runtime.block_on(HttpServer::run(environment_configuration))?;
        return Result::Ok(());
    }
    fn initialize_environment<'a>(
        environment_configuration_file_directory: &'a str,
    ) -> Result<&'static EnvironmentConfiguration<RunServer>, AggregateError> {
        let environment_configuration = Loader::<EnvironmentConfiguration<RunServer>>::load_from_file(environment_configuration_file_directory)?;
        return match ENVIRONMENT_CONFIGURATION.get() {
            Option::Some(environment_configuration__) => Result::Ok(environment_configuration__),
            Option::None => {
                if ENVIRONMENT_CONFIGURATION.set(environment_configuration).is_err() {
                    return crate::new_logic_value_already_exist!();
                }
                crate::option_into_logic_value_does_not_exist!(ENVIRONMENT_CONFIGURATION.get())
            }
        };
    }
    #[cfg(feature = "logging_to_file")]
    fn initialize_logging_to_fileger<'a>(
        environment_configuration: &'a EnvironmentConfiguration<RunServer>,
    ) -> Result<WorkerGuard, AggregateError> {
        let rolling_file_appender = RollingFileAppender::new(
            Rotation::DAILY,
            environment_configuration.subject.logging.directory_path.as_str(),
            environment_configuration.subject.logging.file_name_prefix.as_str(),
        );
        let (non_blocking, worker_guard) = NonBlockingBuilder::default().finish(rolling_file_appender);
        Self::initialize_tracing_subscriber(non_blocking)?;
        return Result::Ok(worker_guard);
    }
    #[cfg(not(feature = "logging_to_file"))]
    fn initialize_stdout_logger() -> Result<WorkerGuard, AggregateError> {
        let (non_blocking, worker_guard) = NonBlockingBuilder::default().finish(std::io::stdout());
        Self::initialize_tracing_subscriber(non_blocking)?;
        return Result::Ok(worker_guard);
    }
    fn initialize_tracing_subscriber(non_blocking: NonBlocking) -> Result<(), AggregateError> {
        let fmt_subscriber = FmtSubscriber::builder()
            .with_max_level(Level::INFO)
            .with_writer(non_blocking)
            .with_file(false)
            .with_target(false)
            .with_thread_ids(false)
            .with_thread_names(false)
            .with_ansi(false)
            .finish();
        crate::result_return_logic!(tracing::subscriber::set_global_default(fmt_subscriber));
        return Result::Ok(());
    }
    fn initialize_runtime<'a>(environment_configuration: &'a EnvironmentConfiguration<RunServer>) -> Result<Runtime, AggregateError> {
        if environment_configuration.subject.tokio_runtime.maximum_blocking_threads_quantity == 0
            || environment_configuration.subject.tokio_runtime.worker_threads_quantity == 0
            || environment_configuration.subject.tokio_runtime.worker_thread_stack_size < (1024 * 1024)
        {
            return crate::new_logic!("Invalid Tokio runtime configuration.");
        }
        return crate::result_into_runtime!(
            RuntimeBuilder::new_multi_thread()
            .max_blocking_threads(environment_configuration.subject.tokio_runtime.maximum_blocking_threads_quantity)
            .worker_threads(environment_configuration.subject.tokio_runtime.worker_threads_quantity)
            .thread_stack_size(environment_configuration.subject.tokio_runtime.worker_thread_stack_size)
            .enable_all()
            .build()
        );
    }
}

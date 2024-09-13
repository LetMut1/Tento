use super::CommandProcessor;
use crate::infrastructure_layer::{
    data::{
        control_type::RunServer,
        environment_configuration::environment_configuration::EnvironmentConfiguration,
    },
    functionality::service::{
        http_server::HttpServer,
        loader::Loader,
    },
};
use aggregate_error::{
    AggregateError,
    Backtrace,
    Common,
    OptionConverter,
    ResultConverter,
};
use std::sync::{
    atomic::AtomicU64,
    OnceLock,
};
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
#[cfg(feature = "file_log")]
use tracing_appender::rolling::{
    RollingFileAppender,
    Rotation,
};
use tracing_subscriber::FmtSubscriber;
static CONNECTION_QUANTITY: AtomicU64 = AtomicU64::new(0);
static ENVIRONMENT_CONFIGURATION: OnceLock<EnvironmentConfiguration> = OnceLock::new();
impl CommandProcessor<RunServer> {
    pub fn process() -> Result<(), AggregateError> {
        let _worker_guard;
        let environment_configuration = Self::initialize_environment()?;
        #[cfg(feature = "file_log")]
        {
            _worker_guard = Self::initialize_file_logger(environment_configuration)?;
        }
        #[cfg(not(feature = "file_log"))]
        {
            _worker_guard = Self::initialize_stdout_logger();
        }
        let runtime = Self::initialize_runtime(environment_configuration)?;
        runtime.block_on(HttpServer::run(environment_configuration))?;
        return Ok(());
    }
    fn initialize_environment() -> Result<&'static EnvironmentConfiguration, AggregateError> {
        let environment_configuration_file_path = format!(
            "{}/environment_configuration",
            std::env::var("CARGO_MANIFEST_DIR").into_runtime(
                Backtrace::new(
                    line!(),
                    file!()
                )
            )?
            .as_str(),
        );
        let environment_configuration = Loader::<EnvironmentConfiguration>::load_from_file(environment_configuration_file_path.as_str())?;
        return match ENVIRONMENT_CONFIGURATION.get() {
            Some(environment_configuration__) => Ok(environment_configuration__),
            None => {
                if let Err(_) = ENVIRONMENT_CONFIGURATION.set(environment_configuration) {
                    return Err(
                        AggregateError::new_logic_(
                            Common::ValueAlreadyExist,
                            Backtrace::new(
                                line!(),
                                file!(),
                            ),
                        ),
                    );
                }
                ENVIRONMENT_CONFIGURATION.get().into_logic_value_does_not_exist(
                    Backtrace::new(
                        line!(),
                        file!(),
                    ),
                )
            }
        };
    }
    #[cfg(feature = "file_log")]
    fn initialize_file_logger<'a>(environment_configuration: &'a EnvironmentConfiguration) -> Result<WorkerGuard, AggregateError> {
        let rolling_file_appender = RollingFileAppender::new(
            Rotation::DAILY,
            environment_configuration.logging.directory_path.as_str(),
            environment_configuration.logging.file_name_prefix.as_str(),
        );
        let (non_blocking, worker_guard) = NonBlockingBuilder::default().finish(rolling_file_appender);
        Self::initialize_tracing_subscriber(non_blocking)?;
        return Ok(worker_guard);
    }
    #[cfg(not(feature = "file_log"))]
    fn initialize_stdout_logger() -> Result<WorkerGuard, AggregateError> {
        let (non_blocking, worker_guard) = NonBlockingBuilder::default().finish(std::io::stdout());
        Self::initialize_tracing_subscriber(non_blocking)?;
        return Ok(worker_guard);
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
        tracing::subscriber::set_global_default(fmt_subscriber).into_logic(
            Backtrace::new(
                line!(),
                file!(),
            ),
        )?;
        return Ok(());
    }
    fn initialize_runtime(environment_configuration: &'static EnvironmentConfiguration) -> Result<Runtime, AggregateError> {
        if environment_configuration.tokio_runtime.maximum_blocking_threads_quantity == 0
            || environment_configuration.tokio_runtime.worker_threads_quantity == 0
            || environment_configuration.tokio_runtime.worker_thread_stack_size < (1024 * 1024)
        {
            return Err(
                AggregateError::new_logic(
                    "Invalid Tokio runtime configuration.".into(),
                    Backtrace::new(
                        line!(),
                        file!(),
                    ),
                ),
            );
        }
        return Ok(
            RuntimeBuilder::new_multi_thread()
                .max_blocking_threads(environment_configuration.tokio_runtime.maximum_blocking_threads_quantity)
                .worker_threads(environment_configuration.tokio_runtime.worker_threads_quantity)
                .thread_stack_size(environment_configuration.tokio_runtime.worker_thread_stack_size)
                .enable_all()
                .build()
                .into_runtime(
                    Backtrace::new(
                        line!(),
                        file!(),
                    ),
                )?,
        );
    }
}

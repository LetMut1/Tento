pub use crate::infrastructure_layer::data::environment_configuration::run_server::RunServer;
use {
    super::{
        CommandProcessor,
        TOKIO_CONFUGURATION_ERROR_MESSAGE,
    },
    crate::infrastructure_layer::{
        data::{
            aggregate_error::AggregateError,
            environment_configuration::{
                EnvironmentConfiguration,
                run_server::{
                    TokioCrate,
                    RayonCrate,
                },
            },
        },
        functionality::service::{
            http_server::HttpServer,
            loader::Loader,
        },
    },
    std::sync::OnceLock,
    tokio::runtime::{
        Builder as RuntimeBuilder,
        Runtime,
    },
    tracing::Level,
    tracing_appender::non_blocking::{
        NonBlocking,
        NonBlockingBuilder,
        WorkerGuard,
    },
    tracing_subscriber::FmtSubscriber,
    rayon::ThreadPoolBuilder,
};
#[cfg(feature = "logging_to_file")]
use {
    crate::infrastructure_layer::data::environment_configuration::run_server::Logging,
    tracing_appender::rolling::{
        RollingFileAppender,
        Rotation,
    },
};
static ENVIRONMENT_CONFIGURATION: OnceLock<EnvironmentConfiguration<RunServer>> = OnceLock::new();
impl CommandProcessor<RunServer> {
    pub fn process<'a>(environment_configuration_file_path: &'a str) -> Result<(), AggregateError> {
        let environment_configuration = Self::initialize_environment(environment_configuration_file_path)?;
        if num_cpus::get() < (environment_configuration.subject.tokio_crate.worker_threads_quantity as usize + environment_configuration.subject.rayon_crate.threads_quantity as usize)
            || environment_configuration.subject.tokio_crate.worker_threads_quantity >= environment_configuration.subject.rayon_crate.threads_quantity {
            return Result::Err(
                crate::new_logic!("Invalid distribution of core threads."),
            );
        }
        let _worker_guard = {
            #[cfg(feature = "logging_to_file")]
            {
                Self::initialize_file_logger(&environment_configuration.subject.logging)?
            }
            #[cfg(not(feature = "logging_to_file"))]
            {
                Self::initialize_stdout_logger()
            }
        };
        Self::initialize_rayon_state(&environment_configuration.subject.rayon_crate)?;
        Self::initialize_tokio_runtime(&environment_configuration.subject.tokio_crate)?
            .block_on(HttpServer::run(environment_configuration))?;
        return Result::Ok(());
    }
    fn initialize_environment<'a>(environment_configuration_file_path: &'a str) -> Result<&'static EnvironmentConfiguration<RunServer>, AggregateError> {
        let environment_configuration = Loader::<EnvironmentConfiguration<RunServer>>::load_from_file(environment_configuration_file_path)?;
        return match ENVIRONMENT_CONFIGURATION.get() {
            Option::Some(environment_configuration__) => Result::Ok(environment_configuration__),
            Option::None => {
                if ENVIRONMENT_CONFIGURATION.set(environment_configuration).is_err() {
                    return Result::Err(crate::new_logic_value_already_exist!());
                }
                crate::option_into_logic_value_does_not_exist!(ENVIRONMENT_CONFIGURATION.get())
            }
        };
    }
    #[cfg(feature = "logging_to_file")]
    fn initialize_file_logger<'a>(logging: &'a Logging) -> Result<WorkerGuard, AggregateError> {
        let rolling_file_appender = RollingFileAppender::new(
            Rotation::DAILY,
            logging.directory_path.as_str(),
            logging.file_name_prefix.as_str(),
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
    fn initialize_rayon_state<'a>(rayon_crate: &'a RayonCrate) -> Result<(), AggregateError> {
        if rayon_crate.threads_quantity == 0 {
            return Result::Err(crate::new_logic!("Invalid Rayon configuration."));
        }
        return crate::result_into_runtime!(
            ThreadPoolBuilder::new()
                .num_threads(rayon_crate.threads_quantity as usize)
                .build_global()
        );
    }
    fn initialize_tokio_runtime<'a>(tokio_crate: &'a TokioCrate) -> Result<Runtime, AggregateError> {
        if tokio_crate.worker_threads_quantity == 0 || tokio_crate.worker_thread_stack_size < (1024 * 1024) {
            return Result::Err(crate::new_logic!(TOKIO_CONFUGURATION_ERROR_MESSAGE));
        }
        return crate::result_into_runtime!(
            RuntimeBuilder::new_multi_thread()
                .worker_threads(tokio_crate.worker_threads_quantity as usize)
                .max_blocking_threads(0)
                .thread_stack_size(tokio_crate.worker_thread_stack_size)
                .enable_all()
                .build()
        );
    }
}

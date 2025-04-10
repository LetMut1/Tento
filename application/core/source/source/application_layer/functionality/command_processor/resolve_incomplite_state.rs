pub use crate::infrastructure_layer::data::environment_configuration::resolve_incomplite_state::ResolveIncompliteState;
use {
    super::{
        CommandProcessor,
        TOKIO_RUNTIME_CONFUGURATION_ERROR_MESSAGE,
    },
    crate::infrastructure_layer::{
        data::{
            aggregate_error::AggregateError,
            environment_configuration::{
                EnvironmentConfiguration,
                resolve_incomplite_state::TokioRuntime,
            },
        },
        functionality::service::loader::Loader,
    },
    std::future::Future,
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
};
#[cfg(feature = "logging_to_file")]
use {
    crate::infrastructure_layer::data::environment_configuration::resolve_incomplite_state::Logging,
    tracing_appender::rolling::{
        RollingFileAppender,
        Rotation,
    },
};
impl CommandProcessor<ResolveIncompliteState> {
    pub fn process<'a>(environment_configuration_file_path: &'a str) -> Result<(), AggregateError> {
        let environment_configuration = Loader::<EnvironmentConfiguration<ResolveIncompliteState>>::load_from_file(environment_configuration_file_path)?;
        let _worker_guard = {
            #[cfg(feature = "logging_to_file")]
            {
                Self::initialize_logging_to_fileger(&environment_configuration.subject.logging)?
            }
            #[cfg(not(feature = "logging_to_file"))]
            {
                Self::initialize_stdout_logger()
            }
        };
        let runtime = Self::initialize_runtime(&environment_configuration.subject.tokio_runtime)?;
        runtime.block_on(Self::resolve_incomplite_state(&environment_configuration))?;
        return Result::Ok(());
    }
    #[cfg(feature = "logging_to_file")]
    fn initialize_logging_to_fileger<'a>(logging: &'a Logging) -> Result<WorkerGuard, AggregateError> {
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
    fn initialize_runtime<'a>(tokio_runtime: &'a TokioRuntime) -> Result<Runtime, AggregateError> {
        if tokio_runtime.maximum_blocking_threads_quantity == 0 || tokio_runtime.worker_threads_quantity == 0 || tokio_runtime.worker_thread_stack_size < (1024 * 1024) {
            return Result::Err(crate::new_logic!(TOKIO_RUNTIME_CONFUGURATION_ERROR_MESSAGE));
        }
        return crate::result_into_runtime!(
            RuntimeBuilder::new_multi_thread()
            .max_blocking_threads(tokio_runtime.maximum_blocking_threads_quantity)
            .worker_threads(tokio_runtime.worker_threads_quantity)
            .thread_stack_size(tokio_runtime.worker_thread_stack_size)
            .enable_all()
            .build()
        );
    }
    fn resolve_incomplite_state<'a>(
        environment_configuration: &'a EnvironmentConfiguration<ResolveIncompliteState>,
    ) -> impl Future<Output = Result<(), AggregateError>> + Send + use<'a> {
        return async move {
            // --------------------------------------------------------------------------
            // TODO УДалять с вышедшим сроком экспирации (expires_at) (удалять очень редно, так как нет индекса на поле, по которому будет идти поиск кандидатов.)
            // UserRegistrationToken
            // UserAuthorizationToken
            // UserResetPasswordToken
            // UserAccessRefreshToken
            // ------------------------------------------------------------------------------
            // ------------------------------------------------------------------------------
            // TODO Удалять по can_be_deleted_from и зависимые сущности. Комментарии, Лайки, View и т.п
            // ------------------------------------------------------------------------------
            return Result::Ok(());
        };
    }
}

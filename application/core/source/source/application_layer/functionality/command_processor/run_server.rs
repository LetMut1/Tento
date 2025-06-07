use std::time::Duration;
use core_affinity::CoreId;
use super::{TOKIO_CONFIGURATION_ERROR_MESSAGE_1, TOKIO_CONFIGURATION_ERROR_MESSAGE_2, TOKIO_CONFIGURATION_ERROR_MESSAGE_3, TWO_MIB};
pub use crate::infrastructure_layer::data::environment_configuration::run_server::RunServer;
use {
    super::CommandProcessor,
    crate::infrastructure_layer::{
        data::{
            aggregate_error::AggregateError,
            environment_configuration::{
                EnvironmentConfiguration,
                run_server::{
                    Tokio,
                    Rayon,
                },
            },
        },
        functionality::service::{
            http_server::HttpServer,
            loader::Loader,
        },
    },
    std::{
        sync::OnceLock,
        collections::HashSet,
        hash::RandomState,
    },
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
        Self::initialize_rayon_state(&environment_configuration.subject.system.rayon)?;
        Self::initialize_tokio_runtime(&environment_configuration.subject.system.tokio)?
            .block_on(HttpServer::run(environment_configuration))?;
        return Result::Ok(());
    }
    fn initialize_environment<'a>(environment_configuration_file_path: &'a str) -> Result<&'static EnvironmentConfiguration<RunServer>, AggregateError> {
        let environment_configuration = Loader::<EnvironmentConfiguration<RunServer>>::load_from_file(environment_configuration_file_path)?;
        if environment_configuration.subject.system.tokio.worker_threads_quantity == 0 {
            return Result::Err(
                crate::new_logic!(TOKIO_CONFIGURATION_ERROR_MESSAGE_1)
            );
        }
        if environment_configuration.subject.system.tokio.worker_threads_quantity as usize != environment_configuration.subject.system.tokio.affinited_cores.len() {
            return Result::Err(
                crate::new_logic!(TOKIO_CONFIGURATION_ERROR_MESSAGE_2)
            );
        }
        if environment_configuration.subject.system.tokio.worker_thread_stack_size < (TWO_MIB) {
            return Result::Err(
                crate::new_logic!(TOKIO_CONFIGURATION_ERROR_MESSAGE_3)
            );
        }
        if environment_configuration.subject.system.rayon.threads_quantity == 0 {
            return Result::Err(
                crate::new_logic!("The vaule of 'system.rayon.threads_quantity' is equal to zero.")
            );
        }
        if environment_configuration.subject.system.rayon.threads_quantity as usize != environment_configuration.subject.system.rayon.affinited_cores.len() {
            return Result::Err(
                crate::new_logic!("The vaule of 'system.rayon.threads_quantity' is not equal to the quantity of elements in the value of 'system.rayon.affinited_cores'.")
            );
        }
        let core_id_registry = crate::option_return_logic_value_does_not_exist!(
            core_affinity::get_core_ids()
        );
        if core_id_registry.len() < (environment_configuration.subject.system.tokio.worker_threads_quantity as usize + environment_configuration.subject.system.rayon.threads_quantity as usize) {
            return Result::Err(
                crate::new_logic!("The summ of values of 'system.tokio.worker_threads_quantity' and 'system.rayon.threads_quantity'is greater than quantity of available processor logical cores."),
            );
        }
        let mut is_exist = true;
        'a: for core_id in environment_configuration.subject.system.tokio.affinited_cores.iter() {
            '_b: for core_id_ in core_id_registry.iter() {
                if core_id_.id == *core_id as usize {
                    continue 'a;
                }
            }
            is_exist = false;
            break 'a;
        }
        if !is_exist {
            return Result::Err(
                crate::new_logic!("The value of 'system.tokio.affinited_cores' is invalid."),
            );
        }
        is_exist = true;
        'a: for core_id in environment_configuration.subject.system.rayon.affinited_cores.iter() {
            '_b: for core_id_ in core_id_registry.iter() {
                if core_id_.id == *core_id as usize {
                    continue 'a;
                }
            }
            is_exist = false;
            break 'a;
        }
        if !is_exist {
            return Result::Err(
                crate::new_logic!("The value of 'system.rayon.affinited_cores' is invalid."),
            );
        }
        let encryption_private_keys = vec![
            environment_configuration.subject.encryption.private_key.user_access_token.as_str(),
            environment_configuration.subject.encryption.private_key.user_access_refresh_token.as_str(),
            environment_configuration.subject.encryption.private_key.channel_token.as_str(),
            environment_configuration.subject.encryption.private_key.channel_subscription_token.as_str(),
            environment_configuration.subject.encryption.private_key.channel_publication1_token.as_str(),
            environment_configuration.subject.encryption.private_key.channel_publication1_commentary_token.as_str(),
        ];
        let encryption_private_key_hash_set = HashSet::<_, RandomState>::from_iter(encryption_private_keys.iter());
        if encryption_private_key_hash_set.len() != encryption_private_keys.len() {
            return Result::Err(
                crate::new_logic!("The some values of 'encryption.private_key' are the same."),
            );
        }
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
    fn initialize_rayon_state<'a>(rayon: &'a Rayon) -> Result<(), AggregateError> {


        todo!();



        return crate::result_into_runtime!(
            ThreadPoolBuilder::new()
                .num_threads(rayon.threads_quantity as usize)
                .build_global()
        );
    }
    fn initialize_tokio_runtime(tokio: &'static Tokio) -> Result<Runtime, AggregateError> {
        return crate::result_into_runtime!(
            RuntimeBuilder::new_multi_thread()
                .worker_threads(tokio.worker_threads_quantity as usize)
                .max_blocking_threads(1)
                .thread_keep_alive(Duration::from_secs(1))
                .thread_stack_size(tokio.worker_thread_stack_size)
                .on_thread_start(
                    || -> _ {
                        '_a: for core_id in tokio.affinited_cores.iter() {
                            core_affinity::set_for_current(
                                CoreId {
                                    id: *core_id as usize,
                                },
                            );
                        }
                        return ();
                    },
                )
                .enable_all()
                .build()
        );
    }
}

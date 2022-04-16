use crate::domain_layer::entity::application_user::ApplicationUser;
use crate::infrastructure_layer::error::error_auditor::_component::error_aggregator::_component::run_time_error::_component::resource_error::resource_error::ResourceError;
use crate::infrastructure_layer::error::error_auditor::_component::error_aggregator::_component::run_time_error::run_time_error::RunTimeError;
use crate::infrastructure_layer::error::error_auditor::_component::error_aggregator::error_aggregator::ErrorAggregator;
use crate::infrastructure_layer::error::error_auditor::_component::simple_backtrace::_component::backtrace_part::BacktracePart;
use crate::infrastructure_layer::error::error_auditor::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::service::_in_context_for::infrastructure_layer::repository::_new_for_context::_in_context_for::_resource::postgresql::_new_for_context::prepared_statemant_parameter_convertation_resolver::PreparedStatementParameterConvertationResolver;
use tokio_postgres::Client as Connection;
use tokio_postgres::types::Type;

pub struct Base;

impl Base {
    pub async fn is_exist_by_nickanme<'a>(
        connection: &'a mut Connection,
        nickname: &'a str
    ) -> Result<bool, ErrorAuditor> {
        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query = 
            "SELECT \
                au.id AS i \
            FROM public.application_user au \
            WHERE au.nickname = $1;";

        prepared_statemant_parameter_convertation_resolver.add_parameter(&nickname, Type::TEXT);

        match connection.prepare_typed(query, prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry()).await {
            Ok(ref statement) => {
                match connection.query(statement, prepared_statemant_parameter_convertation_resolver.get_parameter_registry()).await {
                    Ok(row_registry) => {
                        if row_registry.is_empty() {
                            return Ok(false);
                        }
        
                        return Ok(true);
                    }
                    Err(error) => {
                        return Err(
                            ErrorAuditor::new(
                                ErrorAggregator::RunTimeError {run_time_error: RunTimeError::ResourceError {resource_error: ResourceError::PostgresqlError {postgresql_error: error }}},
                                BacktracePart::new(line!(), file!(), None)
                            )
                        );
                    }
                }
            }
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        ErrorAggregator::RunTimeError {run_time_error: RunTimeError::ResourceError {resource_error: ResourceError::PostgresqlError {postgresql_error: error }}},
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        }
    }

    pub async fn is_exist_by_email<'a>(
        connection: &'a mut Connection,
        email: &'a str
    ) -> Result<bool, ErrorAuditor> {
        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query = 
            "SELECT \
                au.id AS i \
            FROM public.application_user au \
            WHERE au.email = $1;";

        prepared_statemant_parameter_convertation_resolver.add_parameter(&email, Type::TEXT);

        match connection.prepare_typed(query, prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry()).await {
            Ok(ref statement) => {
                match connection.query(statement, prepared_statemant_parameter_convertation_resolver.get_parameter_registry()).await {
                    Ok(row_registry) => {
                        if row_registry.is_empty() {
                            return Ok(false);
                        }
                
                        return Ok(true);
                    }
                    Err(error) => {
                        return Err(
                            ErrorAuditor::new(
                                ErrorAggregator::RunTimeError {run_time_error: RunTimeError::ResourceError {resource_error: ResourceError::PostgresqlError {postgresql_error: error }}},
                                BacktracePart::new(line!(), file!(), None)
                            )
                        );
                    }
                }
            }
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        ErrorAggregator::RunTimeError {run_time_error: RunTimeError::ResourceError {resource_error: ResourceError::PostgresqlError {postgresql_error: error }}},
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        }
    }

    pub async fn find_by_email<'a>(
        connection: &'a mut Connection,
        email: &'a str
    ) -> Result<Option<ApplicationUser>, ErrorAuditor> {
        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query = 
            "SELECT \
                au.id AS i, \
                au.email AS e, \
                au.nickname AS n, \
                au.password_hash AS ph, \
                au.created_at::TEXT AS ca \
            FROM public.application_user au \
            WHERE au.email = $1;";

        prepared_statemant_parameter_convertation_resolver.add_parameter(&email, Type::TEXT);

        match connection.prepare_typed(query, prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry()).await {
            Ok(ref statement) => {
                match connection.query(statement, prepared_statemant_parameter_convertation_resolver.get_parameter_registry()).await {
                    Ok(row_registry) => {
                        if !row_registry.is_empty() {
                            let id: i64;
                            match row_registry[0].try_get::<'_, usize, i64>(0) {
                                Ok(id_) => {
                                    id = id_;
                                }
                                Err(error) => {
                                    return Err(
                                        ErrorAuditor::new(
                                            ErrorAggregator::RunTimeError {run_time_error: RunTimeError::ResourceError {resource_error: ResourceError::PostgresqlError {postgresql_error: error }}},
                                            BacktracePart::new(line!(), file!(), None)
                                        )
                                    );
                                }
                            }

                            let email: String;
                            match row_registry[0].try_get::<'_, usize, String>(1) {
                                Ok(email_) => {
                                    email = email_;
                                }
                                Err(error) => {
                                    return Err(
                                        ErrorAuditor::new(
                                            ErrorAggregator::RunTimeError {run_time_error: RunTimeError::ResourceError {resource_error: ResourceError::PostgresqlError {postgresql_error: error }}},
                                            BacktracePart::new(line!(), file!(), None)
                                        )
                                    );
                                }
                            }

                            let nickname: String;
                            match row_registry[0].try_get::<'_, usize, String>(2) {
                                Ok(nickname_) => {
                                    nickname = nickname_;
                                }
                                Err(error) => {
                                    return Err(
                                        ErrorAuditor::new(
                                            ErrorAggregator::RunTimeError {run_time_error: RunTimeError::ResourceError {resource_error: ResourceError::PostgresqlError {postgresql_error: error }}},
                                            BacktracePart::new(line!(), file!(), None)
                                        )
                                    );
                                }
                            }

                            let password_hash: String;
                            match row_registry[0].try_get::<'_, usize, String>(3) {
                                Ok(password_hash_) => {
                                    password_hash = password_hash_;
                                }
                                Err(error) => {
                                    return Err(
                                        ErrorAuditor::new(
                                            ErrorAggregator::RunTimeError {run_time_error: RunTimeError::ResourceError {resource_error: ResourceError::PostgresqlError {postgresql_error: error }}},
                                            BacktracePart::new(line!(), file!(), None)
                                        )
                                    );
                                }
                            }

                            let created_at: String;
                            match row_registry[0].try_get::<'_, usize, String>(4) {
                                Ok(created_at_) => {
                                    created_at = created_at_;
                                }
                                Err(error) => {
                                    return Err(
                                        ErrorAuditor::new(
                                            ErrorAggregator::RunTimeError {run_time_error: RunTimeError::ResourceError {resource_error: ResourceError::PostgresqlError {postgresql_error: error }}},
                                            BacktracePart::new(line!(), file!(), None)
                                        )
                                    );
                                }
                            }

                            return Ok(Some(
                                ApplicationUser::new(
                                    Some(id),
                                    email,
                                    nickname,
                                    password_hash,
                                    created_at,
                                )
                            ));
                        }
        
                        return Ok(None);
                    }
                    Err(error) => {
                        return Err(
                            ErrorAuditor::new(
                                ErrorAggregator::RunTimeError {run_time_error: RunTimeError::ResourceError {resource_error: ResourceError::PostgresqlError {postgresql_error: error }}},
                                BacktracePart::new(line!(), file!(), None)
                            )
                        );
                    }
                }
            }
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        ErrorAggregator::RunTimeError {run_time_error: RunTimeError::ResourceError {resource_error: ResourceError::PostgresqlError {postgresql_error: error }}},
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        }
    }

    pub async fn find_by_nickname<'a>(
        connection: &'a mut Connection,
        nickname: &'a str
    ) -> Result<Option<ApplicationUser>, ErrorAuditor> {
        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query = 
            "SELECT \
                au.id AS i, \
                au.email AS e, \
                au.nickname AS n, \
                au.password_hash AS ph, \
                au.created_at::TEXT AS ca \
            FROM public.application_user au \
            WHERE au.nickname = $1;";

        prepared_statemant_parameter_convertation_resolver.add_parameter(&nickname, Type::TEXT);

        match connection.prepare_typed(query, prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry()).await {
            Ok(ref statement) => {
                match connection.query(statement, prepared_statemant_parameter_convertation_resolver.get_parameter_registry()).await {
                    Ok(row_registry) => {
                        if !row_registry.is_empty() {
                            let id: i64;
                            match row_registry[0].try_get::<'_, usize, i64>(0) {
                                Ok(id_) => {
                                    id = id_;
                                }
                                Err(error) => {
                                    return Err(
                                        ErrorAuditor::new(
                                            ErrorAggregator::RunTimeError {run_time_error: RunTimeError::ResourceError {resource_error: ResourceError::PostgresqlError {postgresql_error: error }}},
                                            BacktracePart::new(line!(), file!(), None)
                                        )
                                    );
                                }
                            }

                            let email: String;
                            match row_registry[0].try_get::<'_, usize, String>(1) {
                                Ok(email_) => {
                                    email = email_;
                                }
                                Err(error) => {
                                    return Err(
                                        ErrorAuditor::new(
                                            ErrorAggregator::RunTimeError {run_time_error: RunTimeError::ResourceError {resource_error: ResourceError::PostgresqlError {postgresql_error: error }}},
                                            BacktracePart::new(line!(), file!(), None)
                                        )
                                    );
                                }
                            }

                            let nickname: String;
                            match row_registry[0].try_get::<'_, usize, String>(2) {
                                Ok(nickname_) => {
                                    nickname = nickname_;
                                }
                                Err(error) => {
                                    return Err(
                                        ErrorAuditor::new(
                                            ErrorAggregator::RunTimeError {run_time_error: RunTimeError::ResourceError {resource_error: ResourceError::PostgresqlError {postgresql_error: error }}},
                                            BacktracePart::new(line!(), file!(), None)
                                        )
                                    );
                                }
                            }

                            let password_hash: String;
                            match row_registry[0].try_get::<'_, usize, String>(3) {
                                Ok(password_hash_) => {
                                    password_hash = password_hash_;
                                }
                                Err(error) => {
                                    return Err(
                                        ErrorAuditor::new(
                                            ErrorAggregator::RunTimeError {run_time_error: RunTimeError::ResourceError {resource_error: ResourceError::PostgresqlError {postgresql_error: error }}},
                                            BacktracePart::new(line!(), file!(), None)
                                        )
                                    );
                                }
                            }

                            let created_at: String;
                            match row_registry[0].try_get::<'_, usize, String>(4) {
                                Ok(created_at_) => {
                                    created_at = created_at_;
                                }
                                Err(error) => {
                                    return Err(
                                        ErrorAuditor::new(
                                            ErrorAggregator::RunTimeError {run_time_error: RunTimeError::ResourceError {resource_error: ResourceError::PostgresqlError {postgresql_error: error }}},
                                            BacktracePart::new(line!(), file!(), None)
                                        )
                                    );
                                }
                            }

                            return Ok(Some(
                                ApplicationUser::new(
                                    Some(id),
                                    email,
                                    nickname,
                                    password_hash,
                                    created_at,
                                )
                            ));
                        }
        
                        return Ok(None);
                    }
                    Err(error) => {
                        return Err(
                            ErrorAuditor::new(
                                ErrorAggregator::RunTimeError {run_time_error: RunTimeError::ResourceError {resource_error: ResourceError::PostgresqlError {postgresql_error: error }}},
                                BacktracePart::new(line!(), file!(), None)
                            )
                        );
                    }
                }
            }
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        ErrorAggregator::RunTimeError {run_time_error: RunTimeError::ResourceError {resource_error: ResourceError::PostgresqlError {postgresql_error: error }}},
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        }
    }

    pub async fn find_by_id<'a>(
        connection: &'a mut Connection,
        id: &'a i64
    ) -> Result<Option<ApplicationUser>, ErrorAuditor> {
        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query = 
            "SELECT \
                au.id AS i, \
                au.email AS e, \
                au.nickname AS n, \
                au.password_hash AS ph, \
                au.created_at::TEXT AS ca \
            FROM public.application_user au \
            WHERE au.id = $1;";

        prepared_statemant_parameter_convertation_resolver.add_parameter(id, Type::INT8);

        match connection.prepare_typed(query, prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry()).await {
            Ok(ref statement) => {
                match connection.query(statement, prepared_statemant_parameter_convertation_resolver.get_parameter_registry()).await {
                    Ok(row_registry) => {
                        if !row_registry.is_empty() {
                            let id: i64;
                            match row_registry[0].try_get::<'_, usize, i64>(0) {
                                Ok(id_) => {
                                    id = id_;
                                }
                                Err(error) => {
                                    return Err(
                                        ErrorAuditor::new(
                                            ErrorAggregator::RunTimeError {run_time_error: RunTimeError::ResourceError {resource_error: ResourceError::PostgresqlError {postgresql_error: error }}},
                                            BacktracePart::new(line!(), file!(), None)
                                        )
                                    );
                                }
                            }

                            let email: String;
                            match row_registry[0].try_get::<'_, usize, String>(1) {
                                Ok(email_) => {
                                    email = email_;
                                }
                                Err(error) => {
                                    return Err(
                                        ErrorAuditor::new(
                                            ErrorAggregator::RunTimeError {run_time_error: RunTimeError::ResourceError {resource_error: ResourceError::PostgresqlError {postgresql_error: error }}},
                                            BacktracePart::new(line!(), file!(), None)
                                        )
                                    );
                                }
                            }

                            let nickname: String;
                            match row_registry[0].try_get::<'_, usize, String>(2) {
                                Ok(nickname_) => {
                                    nickname = nickname_;
                                }
                                Err(error) => {
                                    return Err(
                                        ErrorAuditor::new(
                                            ErrorAggregator::RunTimeError {run_time_error: RunTimeError::ResourceError {resource_error: ResourceError::PostgresqlError {postgresql_error: error }}},
                                            BacktracePart::new(line!(), file!(), None)
                                        )
                                    );
                                }
                            }

                            let password_hash: String;
                            match row_registry[0].try_get::<'_, usize, String>(3) {
                                Ok(password_hash_) => {
                                    password_hash = password_hash_;
                                }
                                Err(error) => {
                                    return Err(
                                        ErrorAuditor::new(
                                            ErrorAggregator::RunTimeError {run_time_error: RunTimeError::ResourceError {resource_error: ResourceError::PostgresqlError {postgresql_error: error }}},
                                            BacktracePart::new(line!(), file!(), None)
                                        )
                                    );
                                }
                            }

                            let created_at: String;
                            match row_registry[0].try_get::<'_, usize, String>(4) {
                                Ok(created_at_) => {
                                    created_at = created_at_;
                                }
                                Err(error) => {
                                    return Err(
                                        ErrorAuditor::new(
                                            ErrorAggregator::RunTimeError {run_time_error: RunTimeError::ResourceError {resource_error: ResourceError::PostgresqlError {postgresql_error: error }}},
                                            BacktracePart::new(line!(), file!(), None)
                                        )
                                    );
                                }
                            }

                            return Ok(Some(
                                ApplicationUser::new(
                                    Some(id),
                                    email,
                                    nickname,
                                    password_hash,
                                    created_at,
                                )
                            ));
                        }
        
                        return Ok(None);
                    }
                    Err(error) => {
                        return Err(
                            ErrorAuditor::new(
                                ErrorAggregator::RunTimeError {run_time_error: RunTimeError::ResourceError {resource_error: ResourceError::PostgresqlError {postgresql_error: error }}},
                                BacktracePart::new(line!(), file!(), None)
                            )
                        );
                    }
                }
            }
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        ErrorAggregator::RunTimeError {run_time_error: RunTimeError::ResourceError {resource_error: ResourceError::PostgresqlError {postgresql_error: error }}},
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        }
    }
}
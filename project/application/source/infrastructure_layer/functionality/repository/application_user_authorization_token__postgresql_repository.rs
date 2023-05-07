use crate::domain_layer::data::entity::application_user_authorization_token::ApplicationUserAuthorizationToken_1;
use crate::domain_layer::data::entity::application_user_authorization_token::ApplicationUserAuthorizationToken_2;
use crate::domain_layer::data::entity::application_user_authorization_token::ApplicationUserAuthorizationToken_3;
use crate::domain_layer::data::entity::application_user_authorization_token::ApplicationUserAuthorizationToken_4;
use crate::domain_layer::data::entity::application_user_authorization_token::ApplicationUserAuthorizationToken_5;
use crate::domain_layer::data::entity::application_user_authorization_token::ApplicationUserAuthorizationToken_CanBeResentFrom;
use crate::domain_layer::data::entity::application_user_authorization_token::ApplicationUserAuthorizationToken_ExpiresAt;
use crate::domain_layer::data::entity::application_user_authorization_token::ApplicationUserAuthorizationToken_Value;
use crate::domain_layer::data::entity::application_user_authorization_token::ApplicationUserAuthorizationToken_WrongEnterTriesQuantity;
use crate::domain_layer::data::entity::application_user_authorization_token::ApplicationUserAuthorizationToken;
use crate::domain_layer::functionality::service::getter::GetterDELETE;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::BaseError;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::error_auditor::ResourceError;
use crate::infrastructure_layer::data::error_auditor::RuntimeError;
use crate::infrastructure_layer::functionality::service::prepared_statemant_parameter_convertation_resolver::PreparedStatementParameterConvertationResolver;
use extern_crate::tokio_postgres::Client as Connection;
use extern_crate::tokio_postgres::types::Type;
use std::borrow::Cow;
use super::postgresql_repository::PostgresqlRepository;

impl PostgresqlRepository<ApplicationUserAuthorizationToken<'_>> {
    pub async fn create<'a>(
        database_2_connection: &'a Connection,
        insert: Insert<'a>
    ) -> Result<ApplicationUserAuthorizationToken<'a>, ErrorAuditor> {
        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let application_user_device_id = insert.application_user_device_id.as_ref();

        let query =
            "INSERT INTO public.application_user_authorization_token AS auat ( \
                application_user_id, \
                application_user_device_id, \
                value, \
                wrong_enter_tries_quantity, \
                expires_at, \
                can_be_resent_from \
            ) VALUES ( \
                $1, \
                $2, \
                $3, \
                $4, \
                $5, \
                $6 \
            );";

        prepared_statemant_parameter_convertation_resolver
            .add_parameter(&insert.application_user_id, Type::INT8)
            .add_parameter(&application_user_device_id, Type::TEXT)
            .add_parameter(&insert.application_user_authorization_token_value, Type::TEXT)
            .add_parameter(&insert.application_user_authorization_token_wrong_enter_tries_quantity, Type::INT2)
            .add_parameter(&insert.application_user_authorization_token_expires_at, Type::INT8)
            .add_parameter(&insert.application_user_authorization_token_can_be_resent_from, Type::INT8);

        let statement = match database_2_connection.prepare_typed(
            query,
            prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry()
        ).await {
            Ok(statement_) => statement_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        if let Err(error) = database_2_connection.query(
            &statement,
            prepared_statemant_parameter_convertation_resolver.get_parameter_registry()
        ).await {
            return Err(
                ErrorAuditor::new(
                    BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                    BacktracePart::new(line!(), file!(), None)
                )
            );
        };

        return Ok(
            ApplicationUserAuthorizationToken::new(
                insert.application_user_id,
                insert.application_user_device_id,
                insert.application_user_authorization_token_value,
                insert.application_user_authorization_token_wrong_enter_tries_quantity,
                insert.application_user_authorization_token_expires_at,
                insert.application_user_authorization_token_can_be_resent_from
            )
        );
    }

    pub async fn delete<'a>(
        database_2_connection: &'a Connection,
        application_user_id: i64,
        application_user_device_id: &'a str
    ) -> Result<(), ErrorAuditor> {
        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query =
            "DELETE FROM ONLY public.application_user_authorization_token AS auat \
            WHERE auat.application_user_id = $1 AND auat.application_user_device_id = $2;";

        prepared_statemant_parameter_convertation_resolver
            .add_parameter(&application_user_id, Type::INT8)
            .add_parameter(&application_user_device_id, Type::TEXT);

        let statement = match database_2_connection.prepare_typed(
            query,
            prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry()
        ).await {
            Ok(statement_) => statement_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        if let Err(error) = database_2_connection.query(
            &statement,
            prepared_statemant_parameter_convertation_resolver.get_parameter_registry()
        ).await {
            return Err(
                ErrorAuditor::new(
                    BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                    BacktracePart::new(line!(), file!(), None)
                )
            );
        };

        return Ok(());
    }
}

impl PostgresqlRepository<ApplicationUserAuthorizationToken_1> {
    pub async fn update<'a, T>(
        database_2_connection: &'a Connection,
        subject: &'a T,
        application_user_id: i64,
        application_user_device_id: &'a str
    ) -> Result<(), ErrorAuditor>
    where
        T: GetterDELETE<&'a T, ApplicationUserAuthorizationToken_Value, &'a str>,
        T: GetterDELETE<&'a T, ApplicationUserAuthorizationToken_WrongEnterTriesQuantity, i16>,
        T: GetterDELETE<&'a T, ApplicationUserAuthorizationToken_ExpiresAt, i64>,
        T: GetterDELETE<&'a T, ApplicationUserAuthorizationToken_CanBeResentFrom, i64>
    {
        let application_user_authorization_token_value = <T as GetterDELETE<&'_ T, ApplicationUserAuthorizationToken_Value, &'_ str>>::get(subject);

        let application_user_authorization_token_wrong_enter_tries_quantity = <T as GetterDELETE<&'_ T, ApplicationUserAuthorizationToken_WrongEnterTriesQuantity, i16>>::get(subject);

        let application_user_authorization_token_expires_at = <T as GetterDELETE<&'_ T, ApplicationUserAuthorizationToken_ExpiresAt, i64>>::get(subject);

        let application_user_authorization_token_can_be_resent_from = <T as GetterDELETE<&'_ T, ApplicationUserAuthorizationToken_CanBeResentFrom, i64>>::get(subject);

        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query =
            "UPDATE ONLY public.application_user_authorization_token AS auat \
            SET ( \
                value, \
                wrong_enter_tries_quantity, \
                expires_at, \
                can_be_resent_from \
            ) = ROW( \
                $1, \
                $2, \
                $3, \
                $4 \
            ) \
            WHERE auat.application_user_id = $5 AND auat.application_user_device_id = $6;";

        prepared_statemant_parameter_convertation_resolver
            .add_parameter(&application_user_authorization_token_value, Type::TEXT)
            .add_parameter(&application_user_authorization_token_wrong_enter_tries_quantity, Type::INT2)
            .add_parameter(&application_user_authorization_token_expires_at, Type::INT8)
            .add_parameter(&application_user_authorization_token_can_be_resent_from, Type::INT8)
            .add_parameter(&application_user_id, Type::INT8)
            .add_parameter(&application_user_device_id, Type::TEXT);

        let statement = match database_2_connection.prepare_typed(
            query,
            prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry()
        ).await {
            Ok(statement_) => statement_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        if let Err(error) = database_2_connection.query(
            &statement,
            prepared_statemant_parameter_convertation_resolver.get_parameter_registry()
        ).await {
            return Err(
                ErrorAuditor::new(
                    BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                    BacktracePart::new(line!(), file!(), None)
                )
            );
        };

        return Ok(());
    }

    pub async fn find_1<'a>(
        database_2_connection: &'a Connection,
        application_user_id: i64,
        application_user_device_id: &'a str
    ) -> Result<Option<ApplicationUserAuthorizationToken_1>, ErrorAuditor> {
        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query =
            "SELECT \
                auat.value AS v, \
                auat.wrong_enter_tries_quantity AS wetq, \
                auat.expires_at AS ea, \
                auat.can_be_resent_from AS cbrf \
            FROM public.application_user_authorization_token auat \
            WHERE auat.application_user_id = $1 AND auat.application_user_device_id = $2;";

        prepared_statemant_parameter_convertation_resolver
            .add_parameter(&application_user_id, Type::INT8)
            .add_parameter(&application_user_device_id, Type::TEXT);

        let statement = match database_2_connection.prepare_typed(
            query,
            prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry()
        ).await {
            Ok(statement_) => statement_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let row_registry = match database_2_connection.query(
            &statement,
            prepared_statemant_parameter_convertation_resolver.get_parameter_registry()
        ).await {
            Ok(row_registry_) => row_registry_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        if row_registry.is_empty() {
            return Ok(None);
        }

        let application_user_authorization_token_value = match row_registry[0].try_get::<'_, usize, String>(0) {
            Ok(application_user_authorization_token_value_) => application_user_authorization_token_value_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let application_user_authorization_token_wrong_enter_tries_quantity = match row_registry[0].try_get::<'_, usize, i16>(1) {
            Ok(application_user_authorization_token_wrong_enter_tries_quantity_) => application_user_authorization_token_wrong_enter_tries_quantity_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let application_user_authorization_token_expires_at = match row_registry[0].try_get::<'_, usize, i64>(2) {
            Ok(application_user_authorization_token_expires_at_) => application_user_authorization_token_expires_at_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let application_user_authorization_token_can_be_resent_from = match row_registry[0].try_get::<'_, usize, i64>(3) {
            Ok(application_user_authorization_token_can_be_resent_from_) => application_user_authorization_token_can_be_resent_from_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        return Ok(
            Some(
                ApplicationUserAuthorizationToken_1::new(
                    application_user_authorization_token_value,
                    application_user_authorization_token_wrong_enter_tries_quantity,
                    application_user_authorization_token_expires_at,
                    application_user_authorization_token_can_be_resent_from
                )
            )
        );
    }
}

impl PostgresqlRepository<ApplicationUserAuthorizationToken_2> {
    pub async fn update<'a, T>(
        database_2_connection: &'a Connection,
        subject: &'a T,
        application_user_id: i64,
        application_user_device_id: &'a str
    ) -> Result<(), ErrorAuditor>
    where
        T: GetterDELETE<&'a T, ApplicationUserAuthorizationToken_Value, &'a str>,
        T: GetterDELETE<&'a T, ApplicationUserAuthorizationToken_WrongEnterTriesQuantity, i16>,
        T: GetterDELETE<&'a T, ApplicationUserAuthorizationToken_ExpiresAt, i64>
    {
        let application_user_authorization_token_value = <T as GetterDELETE<&'_ T, ApplicationUserAuthorizationToken_Value, &'_ str>>::get(subject);

        let application_user_authorization_token_wrong_enter_tries_quantity = <T as GetterDELETE<&'_ T, ApplicationUserAuthorizationToken_WrongEnterTriesQuantity, i16>>::get(subject);

        let application_user_authorization_token_expires_at = <T as GetterDELETE<&'_ T, ApplicationUserAuthorizationToken_ExpiresAt, i64>>::get(subject);

        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query =
            "UPDATE ONLY public.application_user_authorization_token AS auat \
            SET ( \
                value, \
                wrong_enter_tries_quantity, \
                expires_at \
            ) = ROW( \
                $1, \
                $2, \
                $3 \
            ) \
            WHERE auat.application_user_id = $4 AND auat.application_user_device_id = $5;";

        prepared_statemant_parameter_convertation_resolver
            .add_parameter(&application_user_authorization_token_value, Type::TEXT)
            .add_parameter(&application_user_authorization_token_wrong_enter_tries_quantity, Type::INT2)
            .add_parameter(&application_user_authorization_token_expires_at, Type::INT8)
            .add_parameter(&application_user_id, Type::INT8)
            .add_parameter(&application_user_device_id, Type::TEXT);

        let statement = match database_2_connection.prepare_typed(
            query,
            prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry()
        ).await {
            Ok(statement_) => statement_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        if let Err(error) = database_2_connection.query(
            &statement,
            prepared_statemant_parameter_convertation_resolver.get_parameter_registry()
        ).await {
            return Err(
                ErrorAuditor::new(
                    BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                    BacktracePart::new(line!(), file!(), None)
                )
            );
        };

        return Ok(());
    }

    pub async fn find_1<'a>(
        database_2_connection: &'a Connection,
        application_user_id: i64,
        application_user_device_id: &'a str
    ) -> Result<Option<ApplicationUserAuthorizationToken_2>, ErrorAuditor> {
        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query =
            "SELECT \
                auat.value AS v, \
                auat.wrong_enter_tries_quantity AS wetq, \
                auat.expires_at AS ea \
            FROM public.application_user_authorization_token auat \
            WHERE auat.application_user_id = $1 AND auat.application_user_device_id = $2;";

        prepared_statemant_parameter_convertation_resolver
            .add_parameter(&application_user_id, Type::INT8)
            .add_parameter(&application_user_device_id, Type::TEXT);

        let statement = match database_2_connection.prepare_typed(
            query,
            prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry()
        ).await {
            Ok(statement_) => statement_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let row_registry = match database_2_connection.query(
            &statement,
            prepared_statemant_parameter_convertation_resolver.get_parameter_registry()
        ).await {
            Ok(row_registry_) => row_registry_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        if row_registry.is_empty() {
            return Ok(None);
        }

        let application_user_authorization_token_value = match row_registry[0].try_get::<'_, usize, String>(0) {
            Ok(application_user_authorization_token_value_) => application_user_authorization_token_value_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let application_user_authorization_token_wrong_enter_tries_quantity = match row_registry[0].try_get::<'_, usize, i16>(1) {
            Ok(application_user_authorization_token_wrong_enter_tries_quantity_) => application_user_authorization_token_wrong_enter_tries_quantity_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let application_user_authorization_token_expires_at = match row_registry[0].try_get::<'_, usize, i64>(2) {
            Ok(application_user_authorization_token_expires_at_) => application_user_authorization_token_expires_at_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        return Ok(
            Some(
                ApplicationUserAuthorizationToken_2::new(
                    application_user_authorization_token_value,
                    application_user_authorization_token_wrong_enter_tries_quantity,
                    application_user_authorization_token_expires_at
                )
            )
        );
    }
}

impl PostgresqlRepository<ApplicationUserAuthorizationToken_3> {
    pub async fn update<'a, T>(
        database_2_connection: &'a Connection,
        subject: &'a T,
        application_user_id: i64,
        application_user_device_id: &'a str
    ) -> Result<(), ErrorAuditor>
    where
        T: GetterDELETE<&'a T, ApplicationUserAuthorizationToken_CanBeResentFrom, i64>
    {
        let application_user_authorization_token_can_be_resent_from = <T as GetterDELETE<&'_ T, ApplicationUserAuthorizationToken_CanBeResentFrom, i64>>::get(subject);

        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query =
            "UPDATE ONLY public.application_user_authorization_token AS auat \
            SET ( \
                can_be_resent_from \
            ) = ROW( \
                $1 \
            ) \
            WHERE auat.application_user_id = $2 AND auat.application_user_device_id = $3;";

        prepared_statemant_parameter_convertation_resolver
            .add_parameter(&application_user_authorization_token_can_be_resent_from, Type::INT8)
            .add_parameter(&application_user_id, Type::INT8)
            .add_parameter(&application_user_device_id, Type::TEXT);

        let statement = match database_2_connection.prepare_typed(
            query,
            prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry()
        ).await {
            Ok(statement_) => statement_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        if let Err(error) = database_2_connection.query(
            &statement,
            prepared_statemant_parameter_convertation_resolver.get_parameter_registry()
        ).await {
            return Err(
                ErrorAuditor::new(
                    BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                    BacktracePart::new(line!(), file!(), None)
                )
            );
        };

        return Ok(());
    }
}

impl PostgresqlRepository<ApplicationUserAuthorizationToken_4> {
    pub async fn update<'a, T>(
        database_2_connection: &'a Connection,
        subject: &'a T,
        application_user_id: i64,
        application_user_device_id: &'a str
    ) -> Result<(), ErrorAuditor>
    where
        T: GetterDELETE<&'a T, ApplicationUserAuthorizationToken_WrongEnterTriesQuantity, i16>
    {
        let application_user_authorization_token_wrong_enter_tries_quantity = <T as GetterDELETE<&'_ T, ApplicationUserAuthorizationToken_WrongEnterTriesQuantity, i16>>::get(subject);

        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query =
            "UPDATE ONLY public.application_user_authorization_token AS auat \
            SET ( \
                wrong_enter_tries_quantity \
            ) = ROW( \
                $1 \
            ) \
            WHERE auat.application_user_id = $2 AND auat.application_user_device_id = $3;";

        prepared_statemant_parameter_convertation_resolver
            .add_parameter(&application_user_authorization_token_wrong_enter_tries_quantity, Type::INT2)
            .add_parameter(&application_user_id, Type::INT8)
            .add_parameter(&application_user_device_id, Type::TEXT);

        let statement = match database_2_connection.prepare_typed(
            query,
            prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry()
        ).await {
            Ok(statement_) => statement_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        if let Err(error) = database_2_connection.query(
            &statement,
            prepared_statemant_parameter_convertation_resolver.get_parameter_registry()
        ).await {
            return Err(
                ErrorAuditor::new(
                    BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                    BacktracePart::new(line!(), file!(), None)
                )
            );
        };

        return Ok(());
    }
}

impl PostgresqlRepository<ApplicationUserAuthorizationToken_5> {
    pub async fn find_1<'a>(
        database_2_connection: &'a Connection,
        application_user_id: i64,
        application_user_device_id: &'a str
    ) -> Result<Option<ApplicationUserAuthorizationToken_5>, ErrorAuditor> {
        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query =
            "SELECT \
                auat.value AS v, \
                auat.expires_at AS ea, \
                auat.can_be_resent_from AS cbrf \
            FROM public.application_user_authorization_token auat \
            WHERE auat.application_user_id = $1 AND auat.application_user_device_id = $2;";

        prepared_statemant_parameter_convertation_resolver
            .add_parameter(&application_user_id, Type::INT8)
            .add_parameter(&application_user_device_id, Type::TEXT);

        let statement = match database_2_connection.prepare_typed(
            query,
            prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry()
        ).await {
            Ok(statement_) => statement_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let row_registry = match database_2_connection.query(
            &statement,
            prepared_statemant_parameter_convertation_resolver.get_parameter_registry()
        ).await {
            Ok(row_registry_) => row_registry_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        if row_registry.is_empty() {
            return Ok(None);
        }

        let application_user_authorization_token_value = match row_registry[0].try_get::<'_, usize, String>(0) {
            Ok(application_user_authorization_token_value_) => application_user_authorization_token_value_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let application_user_authorization_token_expires_at = match row_registry[0].try_get::<'_, usize, i64>(1) {
            Ok(application_user_authorization_token_expires_at_) => application_user_authorization_token_expires_at_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let application_user_authorization_token_can_be_resent_from = match row_registry[0].try_get::<'_, usize, i64>(2) {
            Ok(application_user_authorization_token_can_be_resent_from_) => application_user_authorization_token_can_be_resent_from_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        return Ok(
            Some(
                ApplicationUserAuthorizationToken_5::new(
                    application_user_authorization_token_value,
                    application_user_authorization_token_expires_at,
                    application_user_authorization_token_can_be_resent_from
                )
            )
        );
    }
}

pub struct Insert<'a> {
    pub application_user_id: i64,
    pub application_user_device_id: Cow<'a, str>,
    pub application_user_authorization_token_value: String,
    pub application_user_authorization_token_wrong_enter_tries_quantity: i16,
    pub application_user_authorization_token_expires_at: i64,
    pub application_user_authorization_token_can_be_resent_from: i64
}
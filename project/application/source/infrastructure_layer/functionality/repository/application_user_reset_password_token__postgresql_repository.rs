use crate::domain_layer::data::entity::application_user_device::ApplicationUserDevice_Id;
use crate::domain_layer::data::entity::application_user_reset_password_token::ApplicationUserResetPasswordToken_1;
use crate::domain_layer::data::entity::application_user_reset_password_token::ApplicationUserResetPasswordToken_2;
use crate::domain_layer::data::entity::application_user_reset_password_token::ApplicationUserResetPasswordToken_3;
use crate::domain_layer::data::entity::application_user_reset_password_token::ApplicationUserResetPasswordToken_4;
use crate::domain_layer::data::entity::application_user_reset_password_token::ApplicationUserResetPasswordToken_5;
use crate::domain_layer::data::entity::application_user_reset_password_token::ApplicationUserResetPasswordToken_6;
use crate::domain_layer::data::entity::application_user_reset_password_token::ApplicationUserResetPasswordToken_CanBeResentFrom;
use crate::domain_layer::data::entity::application_user_reset_password_token::ApplicationUserResetPasswordToken_ExpiresAt;
use crate::domain_layer::data::entity::application_user_reset_password_token::ApplicationUserResetPasswordToken_IsApproved;
use crate::domain_layer::data::entity::application_user_reset_password_token::ApplicationUserResetPasswordToken_Value;
use crate::domain_layer::data::entity::application_user_reset_password_token::ApplicationUserResetPasswordToken_WrongEnterTriesQuantity;
use crate::domain_layer::data::entity::application_user_reset_password_token::ApplicationUserResetPasswordToken;
use crate::domain_layer::data::entity::application_user::ApplicationUser_Id;
use crate::domain_layer::functionality::service::getter::Getter;
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

impl PostgresqlRepository<ApplicationUserResetPasswordToken<'_>> {
    pub async fn create<'a>(
        database_2_connection: &'a Connection,
        insert: Insert<'a>
    ) -> Result<ApplicationUserResetPasswordToken<'a>, ErrorAuditor> {
        let application_user_id = insert.application_user_id.get();

        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let application_user_device_id = insert.application_user_device_id.as_ref().get();

        let application_user_reset_password_token_value = insert.application_user_reset_password_token_value.get();

        let application_user_reset_password_token_wrong_enter_tries_quantity = insert.application_user_reset_password_token_wrong_enter_tries_quantity.get();

        let application_user_reset_password_token_is_approved = insert.application_user_reset_password_token_is_approved.get();

        let application_user_reset_password_token_expires_at = insert.application_user_reset_password_token_expires_at.get();

        let application_user_reset_password_token_can_be_resent_from = insert.application_user_reset_password_token_can_be_resent_from.get();

        let query =
            "INSERT INTO public.application_user_reset_password_token AS aurpt ( \
                application_user_id, \
                application_user_device_id, \
                value, \
                wrong_enter_tries_quantity, \
                is_approved, \
                expires_at, \
                can_be_resent_from \
            ) VALUES ( \
                $1, \
                $2, \
                $3, \
                $4, \
                $5, \
                $6, \
                $7 \
            );";

        prepared_statemant_parameter_convertation_resolver
            .add_parameter(&application_user_id, Type::INT8)
            .add_parameter(&application_user_device_id, Type::TEXT)
            .add_parameter(&application_user_reset_password_token_value, Type::TEXT)
            .add_parameter(&application_user_reset_password_token_wrong_enter_tries_quantity, Type::INT2)
            .add_parameter(&application_user_reset_password_token_is_approved, Type::BOOL)
            .add_parameter(&application_user_reset_password_token_expires_at, Type::INT8)
            .add_parameter(&application_user_reset_password_token_can_be_resent_from, Type::INT8);

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
            ApplicationUserResetPasswordToken::new(
                insert.application_user_id,
                insert.application_user_device_id,
                insert.application_user_reset_password_token_value,
                insert.application_user_reset_password_token_wrong_enter_tries_quantity,
                insert.application_user_reset_password_token_is_approved,
                insert.application_user_reset_password_token_expires_at,
                insert.application_user_reset_password_token_can_be_resent_from
            )
        );
    }

    pub async fn delete<'a>(
        database_2_connection: &'a Connection,
        application_user_id: ApplicationUser_Id,
        application_user_device_id: &'a ApplicationUserDevice_Id
    ) -> Result<(), ErrorAuditor> {
        let application_user_id_ = application_user_id.get();

        let application_user_device_id_ = application_user_device_id.get();

        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query =
            "DELETE FROM ONLY public.application_user_reset_password_token AS aurpt \
            WHERE aurpt.application_user_id = $1 AND aurpt.application_user_device_id = $2;";

        prepared_statemant_parameter_convertation_resolver
            .add_parameter(&application_user_id_, Type::INT8)
            .add_parameter(&application_user_device_id_, Type::TEXT);

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

impl PostgresqlRepository<ApplicationUserResetPasswordToken_1> {
    pub async fn update<'a, T>(
        database_2_connection: &'a Connection,
        subject: &'a T,
        application_user_id: ApplicationUser_Id,
        application_user_device_id: &'a ApplicationUserDevice_Id
    ) -> Result<(), ErrorAuditor>
    where
        T: Getter<'a, &'a ApplicationUserResetPasswordToken_Value>,
        T: Getter<'a, ApplicationUserResetPasswordToken_WrongEnterTriesQuantity>,
        T: Getter<'a, ApplicationUserResetPasswordToken_IsApproved>,
        T: Getter<'a, ApplicationUserResetPasswordToken_ExpiresAt>,
        T: Getter<'a, ApplicationUserResetPasswordToken_CanBeResentFrom>
    {
        let application_user_id_ = application_user_id.get();

        let application_user_device_id_ = application_user_device_id.get();

        let application_user_reset_password_token_value = <T as Getter<'a, &'a ApplicationUserResetPasswordToken_Value>>::get(subject).get();

        let application_user_reset_password_token_wrong_enter_tries_quantity = <T as Getter<'a, ApplicationUserResetPasswordToken_WrongEnterTriesQuantity>>::get(subject).get();

        let application_user_reset_password_token_is_approved = <T as Getter<'a, ApplicationUserResetPasswordToken_IsApproved>>::get(subject).get();

        let application_user_reset_password_token_expires_at = <T as Getter<'a, ApplicationUserResetPasswordToken_ExpiresAt>>::get(subject).get();

        let application_user_reset_password_token_can_be_resent_from = <T as Getter<'a, ApplicationUserResetPasswordToken_CanBeResentFrom>>::get(subject).get();

        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query =
            "UPDATE ONLY public.application_user_reset_password_token AS aurpt
            SET ( \
                value, \
                wrong_enter_tries_quantity, \
                is_approved, \
                expires_at, \
                can_be_resent_from \
            ) = ROW( \
                $1, \
                $2, \
                $3, \
                $4, \
                $5 \
            ) \
            WHERE aurpt.application_user_id = $6 AND aurpt.application_user_device_id = $7;";

        prepared_statemant_parameter_convertation_resolver
            .add_parameter(&application_user_reset_password_token_value, Type::TEXT)
            .add_parameter(&application_user_reset_password_token_wrong_enter_tries_quantity, Type::INT2)
            .add_parameter(&application_user_reset_password_token_is_approved, Type::BOOL)
            .add_parameter(&application_user_reset_password_token_expires_at, Type::INT8)
            .add_parameter(&application_user_reset_password_token_can_be_resent_from, Type::INT8)
            .add_parameter(&application_user_id_, Type::INT8)
            .add_parameter(&application_user_device_id_, Type::TEXT);

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
        application_user_id: ApplicationUser_Id,
        application_user_device_id: &'a ApplicationUserDevice_Id
    ) -> Result<Option<ApplicationUserResetPasswordToken_1>, ErrorAuditor> {
        let application_user_id_ = application_user_id.get();

        let application_user_device_id_ = application_user_device_id.get();

        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query =
            "SELECT \
                aurpt.value AS v, \
                aurpt.wrong_enter_tries_quantity AS wetq, \
                aurpt.is_approved AS ia, \
                aurpt.expires_at AS ea,
                aurpt.can_be_resent_from AS cbrf \
            FROM public.application_user_reset_password_token aurpt \
            WHERE aurpt.application_user_id = $1 AND aurpt.application_user_device_id = $2;";

        prepared_statemant_parameter_convertation_resolver
            .add_parameter(&application_user_id_, Type::INT8)
            .add_parameter(&application_user_device_id_, Type::TEXT);

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

        let application_user_reset_password_token_value = match row_registry[0].try_get::<'_, usize, String>(0) {
            Ok(application_user_reset_password_token_value_) => ApplicationUserResetPasswordToken_Value::new(application_user_reset_password_token_value_),
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let application_user_reset_password_token_wrong_enter_tries_quantity = match row_registry[0].try_get::<'_, usize, i16>(1) {
            Ok(application_user_reset_password_token_wrong_enter_tries_quantity_) => ApplicationUserResetPasswordToken_WrongEnterTriesQuantity::new(application_user_reset_password_token_wrong_enter_tries_quantity_),
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let application_user_reset_password_token_is_approved = match row_registry[0].try_get::<'_, usize, bool>(2) {
            Ok(application_user_reset_password_token_is_approved_) => ApplicationUserResetPasswordToken_IsApproved::new(application_user_reset_password_token_is_approved_),
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let application_user_reset_password_token_expires_at = match row_registry[0].try_get::<'_, usize, i64>(3) {
            Ok(application_user_reset_password_token_expires_at_) => ApplicationUserResetPasswordToken_ExpiresAt::new(application_user_reset_password_token_expires_at_),
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let application_user_reset_password_token_can_be_resent_from = match row_registry[0].try_get::<'_, usize, i64>(4) {
            Ok(application_user_reset_password_token_can_be_resent_from_) => ApplicationUserResetPasswordToken_CanBeResentFrom::new(application_user_reset_password_token_can_be_resent_from_),
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
                ApplicationUserResetPasswordToken_1::new(
                    application_user_reset_password_token_value,
                    application_user_reset_password_token_wrong_enter_tries_quantity,
                    application_user_reset_password_token_is_approved,
                    application_user_reset_password_token_expires_at,
                    application_user_reset_password_token_can_be_resent_from
                )
            )
        );
    }
}

impl PostgresqlRepository<ApplicationUserResetPasswordToken_2> {
    pub async fn update<'a, T>(
        database_2_connection: &'a Connection,
        subject: &'a T,
        application_user_id: ApplicationUser_Id,
        application_user_device_id: &'a ApplicationUserDevice_Id
    ) -> Result<(), ErrorAuditor>
    where
    T: Getter<'a, ApplicationUserResetPasswordToken_CanBeResentFrom>
    {
        let application_user_id_ = application_user_id.get();

        let application_user_device_id_ = application_user_device_id.get();

        let application_user_reset_password_token_can_be_resent_from = <T as Getter<'a, ApplicationUserResetPasswordToken_CanBeResentFrom>>::get(subject).get();

        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query =
            "UPDATE ONLY public.application_user_reset_password_token AS aurpt
            SET ( \
                can_be_resent_from \
            ) = ROW( \
                $1 \
            ) \
            WHERE aurpt.application_user_id = $2 AND aurpt.application_user_device_id = $3;";

        prepared_statemant_parameter_convertation_resolver
            .add_parameter(&application_user_reset_password_token_can_be_resent_from, Type::INT8)
            .add_parameter(&application_user_id_, Type::INT8)
            .add_parameter(&application_user_device_id_, Type::TEXT);

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

impl PostgresqlRepository<ApplicationUserResetPasswordToken_3> {
    pub async fn update<'a, T>(
        database_2_connection: &'a Connection,
        subject: &'a T,
        application_user_id: ApplicationUser_Id,
        application_user_device_id: &'a ApplicationUserDevice_Id
    ) -> Result<(), ErrorAuditor>
    where
        T: Getter<'a, &'a ApplicationUserResetPasswordToken_Value>,
        T: Getter<'a, ApplicationUserResetPasswordToken_WrongEnterTriesQuantity>,
        T: Getter<'a, ApplicationUserResetPasswordToken_IsApproved>,
        T: Getter<'a, ApplicationUserResetPasswordToken_ExpiresAt>
    {
        let application_user_id_ = application_user_id.get();

        let application_user_device_id_ = application_user_device_id.get();

        let application_user_reset_password_token_value = <T as Getter<'a, &'a ApplicationUserResetPasswordToken_Value>>::get(subject).get();

        let application_user_reset_password_token_wrong_enter_tries_quantity = <T as Getter<'a, ApplicationUserResetPasswordToken_WrongEnterTriesQuantity>>::get(subject).get();

        let application_user_reset_password_token_is_approved = <T as Getter<'a, ApplicationUserResetPasswordToken_IsApproved>>::get(subject).get();

        let application_user_reset_password_token_expires_at = <T as Getter<'a, ApplicationUserResetPasswordToken_ExpiresAt>>::get(subject).get();

        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query =
            "UPDATE ONLY public.application_user_reset_password_token AS aurpt
            SET ( \
                value, \
                wrong_enter_tries_quantity, \
                is_approved, \
                expires_at \
            ) = ROW( \
                $1, \
                $2, \
                $3, \
                $4 \
            ) \
            WHERE aurpt.application_user_id = $5 AND aurpt.application_user_device_id = $6;";

        prepared_statemant_parameter_convertation_resolver
            .add_parameter(&application_user_reset_password_token_value, Type::TEXT)
            .add_parameter(&application_user_reset_password_token_wrong_enter_tries_quantity, Type::INT2)
            .add_parameter(&application_user_reset_password_token_is_approved, Type::BOOL)
            .add_parameter(&application_user_reset_password_token_expires_at, Type::INT8)
            .add_parameter(&application_user_id_, Type::INT8)
            .add_parameter(&application_user_device_id_, Type::TEXT);

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
        application_user_id: ApplicationUser_Id,
        application_user_device_id: &'a ApplicationUserDevice_Id
    ) -> Result<Option<ApplicationUserResetPasswordToken_3>, ErrorAuditor> {
        let application_user_id_ = application_user_id.get();

        let application_user_device_id_ = application_user_device_id.get();

        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query =
            "SELECT \
                aurpt.value AS v, \
                aurpt.wrong_enter_tries_quantity AS wetq, \
                aurpt.is_approved AS ia, \
                aurpt.expires_at AS ea \
            FROM public.application_user_reset_password_token aurpt \
            WHERE aurpt.application_user_id = $1 AND aurpt.application_user_device_id = $2;";

        prepared_statemant_parameter_convertation_resolver
            .add_parameter(&application_user_id_, Type::INT8)
            .add_parameter(&application_user_device_id_, Type::TEXT);

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

        let application_user_reset_password_token_value = match row_registry[0].try_get::<'_, usize, String>(0) {
            Ok(application_user_reset_password_token_value_) => ApplicationUserResetPasswordToken_Value::new(application_user_reset_password_token_value_),
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let application_user_reset_password_token_wrong_enter_tries_quantity = match row_registry[0].try_get::<'_, usize, i16>(1) {
            Ok(application_user_reset_password_token_wrong_enter_tries_quantity_) => ApplicationUserResetPasswordToken_WrongEnterTriesQuantity::new(application_user_reset_password_token_wrong_enter_tries_quantity_),
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let application_user_reset_password_token_is_approved = match row_registry[0].try_get::<'_, usize, bool>(2) {
            Ok(application_user_reset_password_token_is_approved_) => ApplicationUserResetPasswordToken_IsApproved::new(application_user_reset_password_token_is_approved_),
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let application_user_reset_password_token_expires_at = match row_registry[0].try_get::<'_, usize, i64>(3) {
            Ok(application_user_reset_password_token_expires_at_) => ApplicationUserResetPasswordToken_ExpiresAt::new(application_user_reset_password_token_expires_at_),
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
                ApplicationUserResetPasswordToken_3::new(
                    application_user_reset_password_token_value,
                    application_user_reset_password_token_wrong_enter_tries_quantity,
                    application_user_reset_password_token_is_approved,
                    application_user_reset_password_token_expires_at,
                )
            )
        );
    }
}

impl PostgresqlRepository<ApplicationUserResetPasswordToken_4> {
    pub async fn update<'a, T>(
        database_2_connection: &'a Connection,
        subject: &'a T,
        application_user_id: ApplicationUser_Id,
        application_user_device_id: &'a ApplicationUserDevice_Id
    ) -> Result<(), ErrorAuditor>
    where
        T: Getter<'a, ApplicationUserResetPasswordToken_WrongEnterTriesQuantity>
    {
        let application_user_id_ = application_user_id.get();

        let application_user_device_id_ = application_user_device_id.get();

        let application_user_reset_password_token_wrong_enter_tries_quantity = <T as Getter<'a, ApplicationUserResetPasswordToken_WrongEnterTriesQuantity>>::get(subject).get();

        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query =
            "UPDATE ONLY public.application_user_reset_password_token AS aurpt
            SET ( \
                wrong_enter_tries_quantity \
            ) = ROW( \
                $1 \
            ) \
            WHERE aurpt.application_user_id = $2 AND aurpt.application_user_device_id = $3;";

        prepared_statemant_parameter_convertation_resolver
            .add_parameter(&application_user_reset_password_token_wrong_enter_tries_quantity, Type::INT2)
            .add_parameter(&application_user_id_, Type::INT8)
            .add_parameter(&application_user_device_id_, Type::TEXT);

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

impl PostgresqlRepository<ApplicationUserResetPasswordToken_5> {
    pub async fn update<'a, T>(
        database_2_connection: &'a Connection,
        subject: &'a T,
        application_user_id: ApplicationUser_Id,
        application_user_device_id: &'a ApplicationUserDevice_Id
    ) -> Result<(), ErrorAuditor>
    where
        T: Getter<'a, ApplicationUserResetPasswordToken_IsApproved>
    {
        let application_user_id_ = application_user_id.get();

        let application_user_device_id_ = application_user_device_id.get();

        let application_user_reset_password_token_is_approved = <T as Getter<'a, ApplicationUserResetPasswordToken_IsApproved>>::get(subject).get();

        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query =
            "UPDATE ONLY public.application_user_reset_password_token AS aurpt
            SET ( \
                is_approved \
            ) = ROW( \
                $1 \
            ) \
            WHERE aurpt.application_user_id = $2 AND aurpt.application_user_device_id = $3;";

        prepared_statemant_parameter_convertation_resolver
            .add_parameter(&application_user_reset_password_token_is_approved, Type::BOOL)
            .add_parameter(&application_user_id_, Type::INT8)
            .add_parameter(&application_user_device_id_, Type::TEXT);

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

impl PostgresqlRepository<ApplicationUserResetPasswordToken_6> {
    pub async fn find_1<'a>(
        database_2_connection: &'a Connection,
        application_user_id: ApplicationUser_Id,
        application_user_device_id: &'a ApplicationUserDevice_Id
    ) -> Result<Option<ApplicationUserResetPasswordToken_6>, ErrorAuditor> {
        let application_user_id_ = application_user_id.get();

        let application_user_device_id_ = application_user_device_id.get();

        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query =
            "SELECT \
                aurpt.value AS v, \
                aurpt.is_approved AS ia, \
                aurpt.expires_at AS ea,
                aurpt.can_be_resent_from AS cbrf \
            FROM public.application_user_reset_password_token aurpt \
            WHERE aurpt.application_user_id = $1 AND aurpt.application_user_device_id = $2;";

        prepared_statemant_parameter_convertation_resolver
            .add_parameter(&application_user_id_, Type::INT8)
            .add_parameter(&application_user_device_id_, Type::TEXT);

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

        let application_user_reset_password_token_value = match row_registry[0].try_get::<'_, usize, String>(0) {
            Ok(application_user_reset_password_token_value_) => ApplicationUserResetPasswordToken_Value::new(application_user_reset_password_token_value_),
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let application_user_reset_password_token_is_approved = match row_registry[0].try_get::<'_, usize, bool>(1) {
            Ok(application_user_reset_password_token_is_approved_) => ApplicationUserResetPasswordToken_IsApproved::new(application_user_reset_password_token_is_approved_),
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let application_user_reset_password_token_expires_at = match row_registry[0].try_get::<'_, usize, i64>(2) {
            Ok(application_user_reset_password_token_expires_at_) => ApplicationUserResetPasswordToken_ExpiresAt::new(application_user_reset_password_token_expires_at_),
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError { runtime_error: RuntimeError::ResourceError { resource_error: ResourceError::PostgresqlError { postgresql_error: error } } },
                        BacktracePart::new(line!(), file!(), None)
                    )
                );
            }
        };

        let application_user_reset_password_token_can_be_resent_from = match row_registry[0].try_get::<'_, usize, i64>(3) {
            Ok(application_user_reset_password_token_can_be_resent_from_) => ApplicationUserResetPasswordToken_CanBeResentFrom::new(application_user_reset_password_token_can_be_resent_from_),
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
                ApplicationUserResetPasswordToken_6::new(
                    application_user_reset_password_token_value,
                    application_user_reset_password_token_is_approved,
                    application_user_reset_password_token_expires_at,
                    application_user_reset_password_token_can_be_resent_from
                )
            )
        );
    }
}

pub struct Insert<'a> {
    pub application_user_id: ApplicationUser_Id,
    pub application_user_device_id: Cow<'a, ApplicationUserDevice_Id>,
    pub application_user_reset_password_token_value: ApplicationUserResetPasswordToken_Value,
    pub application_user_reset_password_token_wrong_enter_tries_quantity: ApplicationUserResetPasswordToken_WrongEnterTriesQuantity,
    pub application_user_reset_password_token_is_approved: ApplicationUserResetPasswordToken_IsApproved,
    pub application_user_reset_password_token_expires_at: ApplicationUserResetPasswordToken_ExpiresAt,
    pub application_user_reset_password_token_can_be_resent_from: ApplicationUserResetPasswordToken_CanBeResentFrom
}
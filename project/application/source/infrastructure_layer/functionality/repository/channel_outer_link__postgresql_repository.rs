use super::postgresql_repository::PostgresqlRepository;
use crate::domain_layer::data::entity::channel::Channel_Id;
use crate::domain_layer::data::entity::channel_outer_link::ChannelOuterLink;
use crate::domain_layer::data::entity::channel_outer_link::ChannelOuterLink_Address;
use crate::domain_layer::data::entity::channel_outer_link::ChannelOuterLink_Alias;
use crate::domain_layer::data::entity::channel_outer_link::ChannelOuterLink_CreatedAt;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::BaseError;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::error_auditor::ResourceError;
use crate::infrastructure_layer::data::error_auditor::RuntimeError;
use crate::infrastructure_layer::functionality::service::prepared_statemant_parameter_convertation_resolver::PreparedStatementParameterConvertationResolver;
use extern_crate::serde::Serialize;
use extern_crate::tokio_postgres::types::Type;
use extern_crate::tokio_postgres::Client as Connection;

#[cfg(feature = "manual_testing")]
use extern_crate::serde::Deserialize;

impl PostgresqlRepository<ChannelOuterLink> {
    pub async fn create<'a>(
        database_1_connection: &'a Connection,
        insert: Insert,
    ) -> Result<ChannelOuterLink, ErrorAuditor> {
        let channel_outer_link_from = insert.channel_outer_link_from.get();

        let channel_outer_link_alias = insert.channel_outer_link_alias.get();

        let channel_outer_link_address = insert.channel_outer_link_address.get();

        let mut prepared_statemant_parameter_convertation_resolver =
            PreparedStatementParameterConvertationResolver::new();

        let query = "INSERT INTO public.channel_inner_link AS cil ( \
                from_, \
                alias, \
                address, \
                created_at \
            ) VALUES ( \
                $1, \
                $2, \
                $3, \
                current_timestamp(6) \
            ) \
            RETURNING \
                cs.created_at::TEXT AS ca;";

        prepared_statemant_parameter_convertation_resolver
            .add_parameter(
                &channel_outer_link_from,
                Type::INT8,
            )
            .add_parameter(
                &channel_outer_link_alias,
                Type::TEXT,
            )
            .add_parameter(
                &channel_outer_link_address,
                Type::TEXT,
            );

        let statement = match database_1_connection
            .prepare_typed(
                query,
                prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry(),
            )
            .await
        {
            Ok(statement_) => statement_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError {
                            runtime_error: RuntimeError::ResourceError {
                                resource_error: ResourceError::PostgresqlError {
                                    postgresql_error: error,
                                },
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
                        ),
                    ),
                );
            }
        };

        let row_registry = match database_1_connection
            .query(
                &statement,
                prepared_statemant_parameter_convertation_resolver.get_parameter_registry(),
            )
            .await
        {
            Ok(row_registry_) => row_registry_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError {
                            runtime_error: RuntimeError::ResourceError {
                                resource_error: ResourceError::PostgresqlError {
                                    postgresql_error: error,
                                },
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
                        ),
                    ),
                );
            }
        };

        let channel_outer_link_created_at = match row_registry[0].try_get::<'_, usize, String>(0) {
            Ok(channel_outer_link_created_at_) => ChannelOuterLink_CreatedAt::new(channel_outer_link_created_at_),
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError {
                            runtime_error: RuntimeError::ResourceError {
                                resource_error: ResourceError::PostgresqlError {
                                    postgresql_error: error,
                                },
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
                        ),
                    ),
                );
            }
        };

        let channel_outer_link = ChannelOuterLink::new(
            insert.channel_outer_link_from,
            insert.channel_outer_link_alias,
            insert.channel_outer_link_address,
            channel_outer_link_created_at,
        );

        return Ok(channel_outer_link);
    }

    pub async fn find_1<'a>(
        database_1_connection: &'a Connection,
        channel_outer_link_from: Channel_Id,
        limit: i16,
    ) -> Result<Vec<ChannelOuterLink1>, ErrorAuditor> {
        let channel_outer_link_from_ = channel_outer_link_from.get();

        let mut prepared_statemant_parameter_convertation_resolver =
            PreparedStatementParameterConvertationResolver::new();

        let query = "SELECT \
                col.alias AS al, \
                col.address AS ad \
            FROM public.channel_outer_link col \
            WHERE col.from_ = $1 \
            LIMIT $2";

        prepared_statemant_parameter_convertation_resolver
            .add_parameter(
                &channel_outer_link_from_,
                Type::INT8,
            )
            .add_parameter(
                &limit,
                Type::INT2,
            );

        let statement = match database_1_connection
            .prepare_typed(
                query,
                prepared_statemant_parameter_convertation_resolver.get_parameter_type_registry(),
            )
            .await
        {
            Ok(statement_) => statement_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError {
                            runtime_error: RuntimeError::ResourceError {
                                resource_error: ResourceError::PostgresqlError {
                                    postgresql_error: error,
                                },
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
                        ),
                    ),
                );
            }
        };

        let row_registry = match database_1_connection
            .query(
                &statement,
                prepared_statemant_parameter_convertation_resolver.get_parameter_registry(),
            )
            .await
        {
            Ok(row_registry_) => row_registry_,
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::RuntimeError {
                            runtime_error: RuntimeError::ResourceError {
                                resource_error: ResourceError::PostgresqlError {
                                    postgresql_error: error,
                                },
                            },
                        },
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
                        ),
                    ),
                );
            }
        };

        let mut channel_outer_link_registry: Vec<ChannelOuterLink1> = vec![];

        if row_registry.is_empty() {
            return Ok(channel_outer_link_registry);
        }

        '_a: for row in row_registry.iter() {
            let channel_outer_link_alias = match row.try_get::<'_, usize, String>(0) {
                Ok(channel_outer_link_alias_) => ChannelOuterLink_Alias::new(channel_outer_link_alias_),
                Err(error) => {
                    return Err(
                        ErrorAuditor::new(
                            BaseError::RuntimeError {
                                runtime_error: RuntimeError::ResourceError {
                                    resource_error: ResourceError::PostgresqlError {
                                        postgresql_error: error,
                                    },
                                },
                            },
                            BacktracePart::new(
                                line!(),
                                file!(),
                                None,
                            ),
                        ),
                    );
                }
            };

            let channel_outer_link_address = match row.try_get::<'_, usize, String>(1) {
                Ok(channel_outer_link_address_) => ChannelOuterLink_Address::new(channel_outer_link_address_),
                Err(error) => {
                    return Err(
                        ErrorAuditor::new(
                            BaseError::RuntimeError {
                                runtime_error: RuntimeError::ResourceError {
                                    resource_error: ResourceError::PostgresqlError {
                                        postgresql_error: error,
                                    },
                                },
                            },
                            BacktracePart::new(
                                line!(),
                                file!(),
                                None,
                            ),
                        ),
                    );
                }
            };

            let channel_outer_link = ChannelOuterLink1 {
                channel_outer_link_alias,
                channel_outer_link_address,
            };

            channel_outer_link_registry.push(channel_outer_link);
        }

        return Ok(channel_outer_link_registry);
    }
}

pub struct Insert {
    pub channel_outer_link_from: Channel_Id,
    pub channel_outer_link_alias: ChannelOuterLink_Alias,
    pub channel_outer_link_address: ChannelOuterLink_Address,
}

#[cfg_attr(
    feature = "manual_testing",
    derive(Deserialize)
)]
#[derive(Serialize)]
#[serde(crate = "extern_crate::serde")]
pub struct ChannelOuterLink1 {
    pub channel_outer_link_alias: ChannelOuterLink_Alias,
    pub channel_outer_link_address: ChannelOuterLink_Address,
}

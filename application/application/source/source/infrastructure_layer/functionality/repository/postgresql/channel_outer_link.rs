use super::by::By9;
use super::insert::Insert9;
use super::PostgresqlRepository;
use crate::domain_layer::data::entity::channel_outer_link::ChannelOuterLink;
use crate::domain_layer::data::entity::channel_outer_link::ChannelOuterLink_Address;
use crate::domain_layer::data::entity::channel_outer_link::ChannelOuterLink_Alias;
use crate::domain_layer::data::entity::channel_outer_link::ChannelOuterLink_CreatedAt;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::Error;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::error_auditor::ResourceError;
use crate::infrastructure_layer::data::error_auditor::Runtime;
use crate::infrastructure_layer::functionality::service::prepared_statemant_parameter_convertation_resolver::PreparedStatementParameterConvertationResolver;
use tokio_postgres::types::Type;
use tokio_postgres::Client as Connection;
pub use action_processor_incoming_outcoming::ChannelOuterLink1;

impl PostgresqlRepository<ChannelOuterLink> {
    pub async fn create<'a>(
        database_1_connection: &'a Connection,
        insert_9: Insert9,
    ) -> Result<ChannelOuterLink, ErrorAuditor> {
        let channel_outer_link_alias = insert_9.channel_outer_link_alias.0.as_str();

        let channel_outer_link_address = insert_9.channel_outer_link_address.0.as_str();

        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query = "\
            INSERT INTO public.channel_inner_link AS cil ( \
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
                &insert_9.channel_outer_link_from.0,
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
                        Error::Runtime {
                            runtime: Runtime::Resource {
                                resource: ResourceError::Postgresql {
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
                        Error::Runtime {
                            runtime: Runtime::Resource {
                                resource: ResourceError::Postgresql {
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
            Ok(channel_outer_link_created_at_) => ChannelOuterLink_CreatedAt(channel_outer_link_created_at_),
            Err(error) => {
                return Err(
                    ErrorAuditor::new(
                        Error::Runtime {
                            runtime: Runtime::Resource {
                                resource: ResourceError::Postgresql {
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

        let channel_outer_link = ChannelOuterLink {
            from: insert_9.channel_outer_link_from,
            alias: insert_9.channel_outer_link_alias,
            address: insert_9.channel_outer_link_address,
            created_at: channel_outer_link_created_at,
        };

        return Ok(channel_outer_link);
    }

    pub async fn find_1<'a>(
        database_1_connection: &'a Connection,
        by_9: &'a By9,
        limit: i16,
    ) -> Result<Vec<ChannelOuterLink1>, ErrorAuditor> {
        let mut prepared_statemant_parameter_convertation_resolver = PreparedStatementParameterConvertationResolver::new();

        let query = "\
            SELECT \
                col.alias AS al, \
                col.address AS ad \
            FROM public.channel_outer_link col \
            WHERE col.from_ = $1 \
            LIMIT $2";

        prepared_statemant_parameter_convertation_resolver
            .add_parameter(
                &by_9.channel_outer_link_from.0,
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
                        Error::Runtime {
                            runtime: Runtime::Resource {
                                resource: ResourceError::Postgresql {
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
                        Error::Runtime {
                            runtime: Runtime::Resource {
                                resource: ResourceError::Postgresql {
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
                Ok(channel_outer_link_alias_) => ChannelOuterLink_Alias(channel_outer_link_alias_),
                Err(error) => {
                    return Err(
                        ErrorAuditor::new(
                            Error::Runtime {
                                runtime: Runtime::Resource {
                                    resource: ResourceError::Postgresql {
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
                Ok(channel_outer_link_address_) => ChannelOuterLink_Address(channel_outer_link_address_),
                Err(error) => {
                    return Err(
                        ErrorAuditor::new(
                            Error::Runtime {
                                runtime: Runtime::Resource {
                                    resource: ResourceError::Postgresql {
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

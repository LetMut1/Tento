use crate::{
    presentation_layer::{
        data::action_route::{
            ActionRoute_,
            ApplicationUser__Authorization_,
            ChannelSubscription__Base_,
            Channel__Base_,
            ACTION_ROUTE,
        },
    },
};
use aggregate_error::{
    AggregateError,
    Backtrace,
    ResultConverter,
};
use matchit::Router as MatchitRouter;
use super::Creator;
pub type Router = MatchitRouter<ActionRoute_>;
impl Creator<Router> {
    pub fn create() -> Result<Router, AggregateError> {
        let mut router = Router::new();
        router
            .insert(
                ACTION_ROUTE.application_user___authorization.check_nickname_for_existing,
                ActionRoute_::ApplicationUser__Authorization {
                    application_user___authorization: ApplicationUser__Authorization_::CheckNicknameForExisting,
                },
            )
            .into_logic(
                Backtrace::new(
                    line!(),
                    file!(),
                ),
            )?;
        router
            .insert(
                ACTION_ROUTE.application_user___authorization.check_email_for_existing,
                ActionRoute_::ApplicationUser__Authorization {
                    application_user___authorization: ApplicationUser__Authorization_::CheckEmailForExisting,
                },
            )
            .into_logic(
                Backtrace::new(
                    line!(),
                    file!(),
                ),
            )?;
        router
            .insert(
                ACTION_ROUTE.application_user___authorization.regisgter_by_first_step,
                ActionRoute_::ApplicationUser__Authorization {
                    application_user___authorization: ApplicationUser__Authorization_::RegisterByFirstStep,
                },
            )
            .into_logic(
                Backtrace::new(
                    line!(),
                    file!(),
                ),
            )?;
        router
            .insert(
                ACTION_ROUTE.application_user___authorization.regisgter_by_second_step,
                ActionRoute_::ApplicationUser__Authorization {
                    application_user___authorization: ApplicationUser__Authorization_::RegisterBySecondStep,
                },
            )
            .into_logic(
                Backtrace::new(
                    line!(),
                    file!(),
                ),
            )?;
        router
            .insert(
                ACTION_ROUTE.application_user___authorization.regisgter_by_last_step,
                ActionRoute_::ApplicationUser__Authorization {
                    application_user___authorization: ApplicationUser__Authorization_::RegisterByLastStep,
                },
            )
            .into_logic(
                Backtrace::new(
                    line!(),
                    file!(),
                ),
            )?;
        router
            .insert(
                ACTION_ROUTE.application_user___authorization.send_email_for_register,
                ActionRoute_::ApplicationUser__Authorization {
                    application_user___authorization: ApplicationUser__Authorization_::SendEmailForRegister,
                },
            )
            .into_logic(
                Backtrace::new(
                    line!(),
                    file!(),
                ),
            )?;
        router
            .insert(
                ACTION_ROUTE.application_user___authorization.authorize_by_first_step,
                ActionRoute_::ApplicationUser__Authorization {
                    application_user___authorization: ApplicationUser__Authorization_::AuthorizeByFirstStep,
                },
            )
            .into_logic(
                Backtrace::new(
                    line!(),
                    file!(),
                ),
            )?;
        router
            .insert(
                ACTION_ROUTE.application_user___authorization.authorize_by_last_step,
                ActionRoute_::ApplicationUser__Authorization {
                    application_user___authorization: ApplicationUser__Authorization_::AuthorizeByLastStep,
                },
            )
            .into_logic(
                Backtrace::new(
                    line!(),
                    file!(),
                ),
            )?;
        router
            .insert(
                ACTION_ROUTE.application_user___authorization.send_email_for_authorize,
                ActionRoute_::ApplicationUser__Authorization {
                    application_user___authorization: ApplicationUser__Authorization_::SendEmailForAuthorize,
                },
            )
            .into_logic(
                Backtrace::new(
                    line!(),
                    file!(),
                ),
            )?;
        router
            .insert(
                ACTION_ROUTE.application_user___authorization.reset_password_by_first_step,
                ActionRoute_::ApplicationUser__Authorization {
                    application_user___authorization: ApplicationUser__Authorization_::ResetPasswordByFirstStep,
                },
            )
            .into_logic(
                Backtrace::new(
                    line!(),
                    file!(),
                ),
            )?;
        router
            .insert(
                ACTION_ROUTE.application_user___authorization.reset_password_by_second_step,
                ActionRoute_::ApplicationUser__Authorization {
                    application_user___authorization: ApplicationUser__Authorization_::ResetPasswordBySecondStep,
                },
            )
            .into_logic(
                Backtrace::new(
                    line!(),
                    file!(),
                ),
            )?;
        router
            .insert(
                ACTION_ROUTE.application_user___authorization.reset_password_by_last_step,
                ActionRoute_::ApplicationUser__Authorization {
                    application_user___authorization: ApplicationUser__Authorization_::ResetPasswordByLastStep,
                },
            )
            .into_logic(
                Backtrace::new(
                    line!(),
                    file!(),
                ),
            )?;
        router
            .insert(
                ACTION_ROUTE.application_user___authorization.send_email_for_reset_password,
                ActionRoute_::ApplicationUser__Authorization {
                    application_user___authorization: ApplicationUser__Authorization_::SendEmailForResetPassword,
                },
            )
            .into_logic(
                Backtrace::new(
                    line!(),
                    file!(),
                ),
            )?;
        router
            .insert(
                ACTION_ROUTE.application_user___authorization.refresh_access_token,
                ActionRoute_::ApplicationUser__Authorization {
                    application_user___authorization: ApplicationUser__Authorization_::RefreshAccessToken,
                },
            )
            .into_logic(
                Backtrace::new(
                    line!(),
                    file!(),
                ),
            )?;
        router
            .insert(
                ACTION_ROUTE.application_user___authorization.deauthorize_from_one_device,
                ActionRoute_::ApplicationUser__Authorization {
                    application_user___authorization: ApplicationUser__Authorization_::DeauthorizeFromOneDevice,
                },
            )
            .into_logic(
                Backtrace::new(
                    line!(),
                    file!(),
                ),
            )?;
        router
            .insert(
                ACTION_ROUTE.application_user___authorization.deauthorize_from_all_devices,
                ActionRoute_::ApplicationUser__Authorization {
                    application_user___authorization: ApplicationUser__Authorization_::DeauthorizeFromAllDevices,
                },
            )
            .into_logic(
                Backtrace::new(
                    line!(),
                    file!(),
                ),
            )?;
        router
            .insert(
                ACTION_ROUTE.channel___base.get_one_by_id,
                ActionRoute_::Channel__Base {
                    channel___base: Channel__Base_::GetOneById,
                },
            )
            .into_logic(
                Backtrace::new(
                    line!(),
                    file!(),
                ),
            )?;
        router
            .insert(
                ACTION_ROUTE.channel___base.get_many_by_name_in_subscription,
                ActionRoute_::Channel__Base {
                    channel___base: Channel__Base_::GetManyByNameInSubscriptions,
                },
            )
            .into_logic(
                Backtrace::new(
                    line!(),
                    file!(),
                ),
            )?;
        router
            .insert(
                ACTION_ROUTE.channel___base.get_many_by_subscription,
                ActionRoute_::Channel__Base {
                    channel___base: Channel__Base_::GetManyBySubscription,
                },
            )
            .into_logic(
                Backtrace::new(
                    line!(),
                    file!(),
                ),
            )?;
        router
            .insert(
                ACTION_ROUTE.channel___base.get_many_piblic_by_name,
                ActionRoute_::Channel__Base {
                    channel___base: Channel__Base_::GetManyPublicByName,
                },
            )
            .into_logic(
                Backtrace::new(
                    line!(),
                    file!(),
                ),
            )?;
        router
            .insert(
                ACTION_ROUTE.channel_subscription___base.create,
                ActionRoute_::ChannelSubscription__Base {
                    channel_subscription___base: ChannelSubscription__Base_::Create,
                },
            )
            .into_logic(
                Backtrace::new(
                    line!(),
                    file!(),
                ),
            )?;
        #[cfg(feature = "manual_testing")]
        {
            router
                .insert(
                    ACTION_ROUTE.application_user___authorization.check_nickname_for_existing_,
                    ActionRoute_::ApplicationUser__Authorization {
                        application_user___authorization: ApplicationUser__Authorization_::CheckNicknameForExisting_,
                    },
                )
                .into_logic(
                    Backtrace::new(
                        line!(),
                        file!(),
                    ),
                )?;
            router
                .insert(
                    ACTION_ROUTE.application_user___authorization.check_email_for_existing_,
                    ActionRoute_::ApplicationUser__Authorization {
                        application_user___authorization: ApplicationUser__Authorization_::CheckEmailForExisting_,
                    },
                )
                .into_logic(
                    Backtrace::new(
                        line!(),
                        file!(),
                    ),
                )?;
            router
                .insert(
                    ACTION_ROUTE.application_user___authorization.regisgter_by_first_step_,
                    ActionRoute_::ApplicationUser__Authorization {
                        application_user___authorization: ApplicationUser__Authorization_::RegisterByFirstStep_,
                    },
                )
                .into_logic(
                    Backtrace::new(
                        line!(),
                        file!(),
                    ),
                )?;
            router
                .insert(
                    ACTION_ROUTE.application_user___authorization.regisgter_by_second_step_,
                    ActionRoute_::ApplicationUser__Authorization {
                        application_user___authorization: ApplicationUser__Authorization_::RegisterBySecondStep_,
                    },
                )
                .into_logic(
                    Backtrace::new(
                        line!(),
                        file!(),
                    ),
                )?;
            router
                .insert(
                    ACTION_ROUTE.application_user___authorization.regisgter_by_last_step_,
                    ActionRoute_::ApplicationUser__Authorization {
                        application_user___authorization: ApplicationUser__Authorization_::RegisterByLastStep_,
                    },
                )
                .into_logic(
                    Backtrace::new(
                        line!(),
                        file!(),
                    ),
                )?;
            router
                .insert(
                    ACTION_ROUTE.application_user___authorization.send_email_for_register_,
                    ActionRoute_::ApplicationUser__Authorization {
                        application_user___authorization: ApplicationUser__Authorization_::SendEmailForRegister_,
                    },
                )
                .into_logic(
                    Backtrace::new(
                        line!(),
                        file!(),
                    ),
                )?;
            router
                .insert(
                    ACTION_ROUTE.application_user___authorization.authorize_by_first_step_,
                    ActionRoute_::ApplicationUser__Authorization {
                        application_user___authorization: ApplicationUser__Authorization_::AuthorizeByFirstStep_,
                    },
                )
                .into_logic(
                    Backtrace::new(
                        line!(),
                        file!(),
                    ),
                )?;
            router
                .insert(
                    ACTION_ROUTE.application_user___authorization.authorize_by_last_step_,
                    ActionRoute_::ApplicationUser__Authorization {
                        application_user___authorization: ApplicationUser__Authorization_::AuthorizeByLastStep_,
                    },
                )
                .into_logic(
                    Backtrace::new(
                        line!(),
                        file!(),
                    ),
                )?;
            router
                .insert(
                    ACTION_ROUTE.application_user___authorization.send_email_for_authorize_,
                    ActionRoute_::ApplicationUser__Authorization {
                        application_user___authorization: ApplicationUser__Authorization_::SendEmailForAuthorize_,
                    },
                )
                .into_logic(
                    Backtrace::new(
                        line!(),
                        file!(),
                    ),
                )?;
            router
                .insert(
                    ACTION_ROUTE.application_user___authorization.reset_password_by_first_step_,
                    ActionRoute_::ApplicationUser__Authorization {
                        application_user___authorization: ApplicationUser__Authorization_::ResetPasswordByFirstStep_,
                    },
                )
                .into_logic(
                    Backtrace::new(
                        line!(),
                        file!(),
                    ),
                )?;
            router
                .insert(
                    ACTION_ROUTE.application_user___authorization.reset_password_by_second_step_,
                    ActionRoute_::ApplicationUser__Authorization {
                        application_user___authorization: ApplicationUser__Authorization_::ResetPasswordBySecondStep_,
                    },
                )
                .into_logic(
                    Backtrace::new(
                        line!(),
                        file!(),
                    ),
                )?;
            router
                .insert(
                    ACTION_ROUTE.application_user___authorization.reset_password_by_last_step_,
                    ActionRoute_::ApplicationUser__Authorization {
                        application_user___authorization: ApplicationUser__Authorization_::ResetPasswordByLastStep_,
                    },
                )
                .into_logic(
                    Backtrace::new(
                        line!(),
                        file!(),
                    ),
                )?;
            router
                .insert(
                    ACTION_ROUTE.application_user___authorization.send_email_for_reset_password_,
                    ActionRoute_::ApplicationUser__Authorization {
                        application_user___authorization: ApplicationUser__Authorization_::SendEmailForResetPassword_,
                    },
                )
                .into_logic(
                    Backtrace::new(
                        line!(),
                        file!(),
                    ),
                )?;
            router
                .insert(
                    ACTION_ROUTE.application_user___authorization.refresh_access_token_,
                    ActionRoute_::ApplicationUser__Authorization {
                        application_user___authorization: ApplicationUser__Authorization_::RefreshAccessToken_,
                    },
                )
                .into_logic(
                    Backtrace::new(
                        line!(),
                        file!(),
                    ),
                )?;
            router
                .insert(
                    ACTION_ROUTE.application_user___authorization.deauthorize_from_one_device_,
                    ActionRoute_::ApplicationUser__Authorization {
                        application_user___authorization: ApplicationUser__Authorization_::DeauthorizeFromOneDevice_,
                    },
                )
                .into_logic(
                    Backtrace::new(
                        line!(),
                        file!(),
                    ),
                )?;
            router
                .insert(
                    ACTION_ROUTE.application_user___authorization.deauthorize_from_all_devices_,
                    ActionRoute_::ApplicationUser__Authorization {
                        application_user___authorization: ApplicationUser__Authorization_::DeauthorizeFromAllDevices_,
                    },
                )
                .into_logic(
                    Backtrace::new(
                        line!(),
                        file!(),
                    ),
                )?;
            router
                .insert(
                    ACTION_ROUTE.channel___base.get_one_by_id_,
                    ActionRoute_::Channel__Base {
                        channel___base: Channel__Base_::GetOneById_,
                    },
                )
                .into_logic(
                    Backtrace::new(
                        line!(),
                        file!(),
                    ),
                )?;
            router
                .insert(
                    ACTION_ROUTE.channel___base.get_many_by_name_in_subscription_,
                    ActionRoute_::Channel__Base {
                        channel___base: Channel__Base_::GetManyByNameInSubscriptions_,
                    },
                )
                .into_logic(
                    Backtrace::new(
                        line!(),
                        file!(),
                    ),
                )?;
            router
                .insert(
                    ACTION_ROUTE.channel___base.get_many_by_subscription_,
                    ActionRoute_::Channel__Base {
                        channel___base: Channel__Base_::GetManyBySubscription_,
                    },
                )
                .into_logic(
                    Backtrace::new(
                        line!(),
                        file!(),
                    ),
                )?;
            router
                .insert(
                    ACTION_ROUTE.channel___base.get_many_piblic_by_name_,
                    ActionRoute_::Channel__Base {
                        channel___base: Channel__Base_::GetManyPublicByName_,
                    },
                )
                .into_logic(
                    Backtrace::new(
                        line!(),
                        file!(),
                    ),
                )?;
            router
                .insert(
                    ACTION_ROUTE.channel_subscription___base.create_,
                    ActionRoute_::ChannelSubscription__Base {
                        channel_subscription___base: ChannelSubscription__Base_::Create_,
                    },
                )
                .into_logic(
                    Backtrace::new(
                        line!(),
                        file!(),
                    ),
                )?;
        }
        return Ok(router);
    }
}
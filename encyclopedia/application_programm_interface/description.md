# Data standards
- `user_access_token_signed`:
```
struct UserAccessTokenEncoded {
    user_access_token__id: String,
    user__id: i64,
    user_device__id: String,
    user_access_token__expires_at: i64,
    signature: Vec<u8>,
}
```
- `user_access_refresh_token_signed`:
```
struct UserAccessRefreshTokenSigned {
    user_access_refresh_token__expires_at: i64,
    signature: Vec<u8>,
}
```
# Request standards
- All payload data is transferred in `HTTP Body` and described under each API endpoint as `Incoming`.
- Every request should contain this `HTTP Header`s:
```
- content-type: application/octet-stream
- content-length: <calculate>
```
# Response standards
- All payload data is transferred in `HTTP Body` and described under each API endpoint as `Outcoming` and `Precedent`.
- Every response should contain this `HTTP Header`s:
```
- content-type: application/octet-stream
- content-length: <calculate>
```
 - The permanent general structure of the each response data with `HTTP Status code` equal to `200` looks like:
```
enum UnifiedReport<T, P> {
    Target {
        data: Data<T>,
    },
    Precedent {
        precedent: P,
    },
}
enum Data<D> {
    Empty,
    Filled {
        data: D,
    },
}
```
- `Outcoming` described under each API endpoint will be nested in the `Filled` variant of `Data<D>`, but in case `Outcoming` is not presented, it will be `Empty` variant of `Data<D>`
- `Precedent` described under each API endpoint will be nested in the `Precedent` variant of `UnifiedReport<T, P>`.
- `HTTP Status code` unequal to `200` have not got `HTTP Body`.

<br/><br/>

# API for authorized user.
 Every endpoint at this area requires an existing of `user_access_token_signed`.
 - ## UserAuthorization_DeauthorizeFromOneDevice POST /user_authorization/deauthorize_from_one_device
```
Deauthorizes user from one device.
```
```
struct Incoming {
    user_access_token_signed: <Data standards>
}
```
```
enum Precedent {
    UserAccessToken__AlreadyExpired,
}
```
 - ## UserAuthorization_DeauthorizeFromAllDevices POST /user_authorization/deauthorize_from_all_devices
```
Deauthorizes user from all devices.
```
```
struct Incoming {
    user_access_token_signed: <Data standards>
}
```
```
enum Precedent {
    UserAccessToken__AlreadyExpired,
}
```
 - ## Channel_CheckLinkedNameForExisting POST /channel/check_linked_name_for_existing
```
Checks linked name for existing.
```
```
struct Incoming {
    user_access_token_signed: <Data standards>,
    channel__linked_name: String,
}
```
```
enum Precedent {
    UserAccessToken__AlreadyExpired,
}
```
```
struct Outcoming {
    result: bool,
}
```
 - ## Channel_CheckNameForExisting POST /channel/check_name_for_existing
```
Checks name for existing.
```
```
struct Incoming {
    user_access_token_signed: <Data standards>,
    channel__name: String,
}
```
```
enum Precedent {
    UserAccessToken__AlreadyExpired,
}
```
```
struct Outcoming {
    result: bool,
}
```
 - ## Channel_Create POST /channel/create
```
Creates channel.
```
```
struct Incoming {
    user_access_token_signed: <Data standards>,
    channel__name: String,
    channel__linked_name: String,
    channel__access_modifier: i16,
    channel__visability_modifier: i16,
}
```
```
enum Precedent {
    UserAccessToken__AlreadyExpired,
    Channel__NameAlreadyExist,
    Channel__LinkedNameAlreadyExist,
}
```
```
struct Outcoming {
    channel__id: i64,
}
```
 - ## Channel_GetOneById POST /channel/get_one_by_id
```
Returns channel data by id.
```
```
struct Incoming {
    user_access_token_signed: <Data standards>
    channel__id: i64,
    channel_token_hashed: Option<ChannelTokenHashed>
}

struct ChannelTokenHashed {
    channel_token__expires_at: i64,
    hash: u64,
}
```
```
struct Outcoming {
    channel__name: String,
    channel__linked_name: String,
    channel__description: Option<String>,
    channel__access_modifier: i16,
    channel__visability_modifier: i16,
    channel__orientation: Vec<i16>,
    channel__cover_image_path: Option<String>,
    channel__background_image_path: Option<String>,
    channel__subscribers_quantity: i64,
    channel__marks_quantity: i64,
    channel__viewing_quantity: i64,
    user_is_channel_owner: bool,
    channel_subscription_token_hashed: ChannelSubscriptionTokenHashed,
}

struct ChannelSubscriptionTokenHashed {
    channel_subscription_token__expires_at: i64,
    hash: u64,
}
```
```
enum Precedent {
    UserAccessToken__AlreadyExpired,
    Channel__NotFound,
    Channel__IsClose,
    ChannelToken__NotFound,
    ChannelToken__AlreadyExpired,
}
```
 - ## Channel_GetManyByNameInSubscriptions POST /channel/get_many_by_name_in_subscriptions
```
Returns channels the user is subscribed to by name.
```
```
struct Incoming {
    user_access_token_signed: <Data standards>
    channel__name: String,
    requery___channel__name: Option<String>,
    limit: i16
}


requery___channel__name - an alternative for offset. Used only for requering with persistent channel__name. The value must be equal to the last channel__name of data_registry registry in received early response.

Incoming parameters validation rule:
- requery___channel__name:
    -- same as channel__name.
- limit:
    -- [1, 100] values.
```
```
struct Outcoming {
    data_registry: Vec<Data>,
}

struct Data {
    channel__id: i64,
    channel__name: String,
    channel__linked_name: String,
    channel__access_modifier: i16,
    channel__visability_modifier: i16,
    channel__cover_image_path: Option<String>,
    channel__background_image_path: Option<String>,
}
```
```
enum Precedent {
    UserAccessToken__AlreadyExpired,
}
```
 - ## Channel_GetManyBySubscription POST /channel/get_many_by_subscription
```
Returns channels the user is subscribed to.
```
```
struct Incoming {
    user_access_token_signed: <Data standards>
    requery___channel__id: Option<i64>,
    limit: i16
}


requery___channel__id - an alternative for offset. The value must be equal to the last channel__id of data_registry registry in received early response.

Incoming parameters validation rule:
- requery___channel__id:
    -- same as channel__id.
- limit:
    -- [1, 100] values.
```
```
struct Outcoming {
    data_registry: Vec<Data>,
}

struct Data {
    channel__id: i64,
    channel__name: String,
    channel__linked_name: String,
    channel__access_modifier: i16,
    channel__visability_modifier: i16,
    channel__cover_image_path: Option<String>,
    channel__background_image_path: Option<String>,
}
```
```
enum Precedent {
    UserAccessToken__AlreadyExpired,
}
```
 - ## Channel_GetManyPublicByName POST /channel/get_many_public_by_name
```
Returns public channels by name.
```
```
struct Incoming {
    user_access_token_signed: <Data standards>
    channel__name: String,
    requery___channel__name: Option<String>,
    limit: i16
}


requery___channel__name - an alternative for offset. Used only for requering with persistent channel__name. The value must be equal to the last channel__name of data_registry registry in received early response.

Incoming parameters validation rule:
- requery___channel__name:
    -- same as channel__name.
- limit:
    -- [1, 15] values.
```
```
struct Outcoming {
    data_registry: Vec<Data>,
}

struct Data {
    channel__id: i64,
    channel__name: String,
    channel__linked_name: String,
    channel__access_modifier: i16,
    channel__cover_image_path: Option<String>,
    channel__background_image_path: Option<String>,
    channel_token_hashed_for_unsubscribed_users: Option<ChannelTokenHashed>,
}

struct ChannelTokenHashed {
    channel_token__expires_at: i64,
    hash: u64,
}
```
```
enum Precedent {
    UserAccessToken__AlreadyExpired,
}
```
 - ## ChannelSubscription_Create POST /channel_subscription/create
```
Subscribes user to channel.
```
```
struct Incoming {
    user_access_token_signed: <Data standards>
    channel__id: i64,
    channel_subscription_token_hashed: ChannelSubscriptionTokenHashed,
}

struct ChannelSubscriptionTokenHashed {
    channel_subscription_token__expires_at: i64,
    hash: u64,
}
```
```
enum Precedent {
    UserAccessToken__AlreadyExpired,
    Channel__NotFound,
    Channel__IsClose,
    User__IsChannelOwner,
    ChannelSubscription__AlreadyExist,
    ChannelSubscriptionToken__AlreadyExpired,
}
```
 - ## ChannelSubscription_Delete POST /channel_subscription/delete
```
Unsubscribes user from channel.
```
```
struct Incoming {
    user_access_token_signed: <Data standards>
    channel__id: i64,
}
```
```
enum Precedent {
    UserAccessToken__AlreadyExpired,
    ChannelSubscription__NotFound,
}
```
 - ## ChannelPublication1_Create POST /channel_publication1/create
```
Creates channel publications of type 1.
```
```
struct Incoming {
    user_access_token_signed: <Data standards>
    channel__id: i64,
    channel_publication1__images_pathes: Vec<String>,
    channel_publication1__text: Option<String>,
}
```
```
struct Outcoming {
    channel_publication1__id: i64,
    channel_publication1__created_at: i64,
}
```
```
enum Precedent {
    UserAccessToken__AlreadyExpired,
    Channel__NotFound,
    User__IsNotChannelOwner,
}
```
- ## ChannelPublication1_Delete POST /channel_publication1/delete
```
Deletes channel publications of type 1.
```
```
struct Incoming {
    user_access_token_signed: <Data standards>
    channel_publication1__id: i64,
}
```
```
enum Precedent {
    UserAccessToken__AlreadyExpired,
    User__IsNotChannelOwner,
    ChannelPublication1__NotFound,
    ChannelPublication1__IsAlreadyDeleted,
}
```
 - ## ChannelPublication1_GetMany POST /channel_publication1/get_many
```
Returns channel publications of type 1.
```
```
struct Incoming {
    user_access_token_signed: <Data standards>
    channel__id: i64,
    channel_publication1__created_at: i64,
    limit: i16,
}


channel_publication1__created_at - an alternative for offset. The value for next requests must be equal to the last channel_publication1__created_at of data_regitry registry in received early response.

Incoming parameters validation rule:
- limit:
    -- [1, 30] values.
```
```
struct Outcoming {
    data_registry: Vec<Data>,
}

struct Data {
    channel_publication1__id: i64,
    channel_publication1__images_pathes: Vec<String>,
    channel_publication1__text: Option<String>,
    channel_publication1__marks_quantity: i64,
    channel_publication1__viewing_quantity: i64,
    channel_publication1__created_at: i64,
    channel_publication1_mark__created_at: Option<i64>,
    channel_publication1_token_signed: ChannelPublication1TokenSigned,
}

struct ChannelPublication1TokenSigned {
    channel_publication1__obfuscation_value: i64,
    channel_publication1_token__expires_at: i64,
    signature: Vec<u8>,
}
```
```
enum Precedent {
    UserAccessToken__AlreadyExpired,
    Channel__NotFound,
    Channel__IsClose,
}
```
 - ## ChannelPublication1Mark_Create POST /channel_publication1/create
```
Creates a mark for channel publication.
```
```
struct Incoming {
    user_access_token_signed: <Data standards>
    pub channel_publication1__id: i64,
    pub channel_publication1_token_signed: ChannelPublication1TokenSigned,
}

struct ChannelPublication1TokenSigned {
    channel_publication1__obfuscation_value: i64,
    channel_publication1_token__expires_at: i64,
    signature: Vec<u8>,
}
```
```
enum Precedent {
    UserAccessToken__AlreadyExpired,
    ChannelPublication1Token__AlreadyExist,
    ChannelPublication1Mark__AlreadyExist,
}
```
<br/><br/>

# API for not authorized user.
 - ## UserAuthorization_CheckEmailForExisting POST /user_authorization/check_email_for_existing
```
Checks user email for existing.
```
```
struct Incoming {
    user__email: String
}
```
```
struct Outcoming {
    result: bool
}
```
 - ## UserAuthorization_CheckNicknameForExisting POST /user_authorization/check_nickname_for_existing
```
Checks user nickname for existing.
```
```
struct Incoming {
    user__nickname: String
}
```
```
struct Outcoming {
    result: bool
}
```
 - ## UserAuthorization_RegisterByFirstStep POST /user_authorization/register_by_first_step
```
Registers user for the first step and sends email to user.
```
```
struct Incoming {
    user__email: String,
    user_device__id: String
}
```
```
struct Outcoming {
    verification_message_sent: bool,
    user_registration_token__can_be_resent_from: i64,
    user_registration_token__wrong_enter_tries_quantity: i16,
    user_registration_token__wrong_enter_tries_quantity_limit: i16,
}


verification_message_sent - determines if a verification message has been sent. The value will be false only if the request was retried
with unchanged parameters without waiting a certain amount of time.

user_registration_token__can_be_resent_from - unixtime after wich it will be allowed to resend the verification message.
```
```
Precedent {
    User__EmailAlreadyExist,
}
```
- ## UserAuthorization_RegisterBySecondStep POST /user_authorization/register_by_second_step
```
Registers user for the second step through token value approving.
```
```
struct Incoming {
    user__email: String,
    user_device__id: String,
    user_registration_token__value: String
}
```
```
enum Precedent {
    UserRegistrationToken__NotFound,
    UserRegistrationToken__AlreadyExpired,
    UserRegistrationToken__AlreadyApproved,
    UserRegistrationToken__WrongValue {
        user_registration_token__wrong_enter_tries_quantity: i16,
    },
}
```

 - ## UserAuthorization_RegisterByLastStep POST /user_authorization/register_by_last_step
```
Registers user for the last step.
```
```
struct Incoming {
    user_device__id: String,
    user__nickname: String,
    user__password: String,
    user__email: String,
    user_registration_token__value: String
}
```
```
struct Outcoming {
    user_access_token_signed: <Data standards>
    user_access_refresh_token_signed: <Data standards>
}
```
```
enum Precedent {
    User__NicknameAlreadyExist,
    User__EmailAlreadyExist,
    UserRegistrationToken__NotFound,
    UserRegistrationToken__AlreadyExpired,
    UserRegistrationToken__IsNotApproved,
    UserRegistrationToken__WrongValue,
}
```
 - ## UserAuthorization_SendEmailForRegister POST /user_authorization/send_email_for_register
```
Sends email for register. (Should be used only if the user does not receive an email.)
```
```
struct Incoming {
    user__email: String,
    user_device__id: String
}
```
```
struct Outcoming {
    user_registration_token__can_be_resent_from: i64
}


user_registration_token__can_be_resent_from - unixtime after wich it will be allowed to resend the verification message.
```
```
enum Precedent {
    UserRegistrationToken__NotFound,
    UserRegistrationToken__AlreadyExpired,
    UserRegistrationToken__AlreadyApproved,
    UserRegistrationToken__TimeToResendHasNotCome,
}
```
 - ## UserAuthorization_AuthorizeByFirstStep POST /user_authorization/authorize_by_first_step
```
Authorizes user for the first step and send email to user.
```
```
struct Incoming {
    user_device__id: String,
    user__email___or___user__nickname: String,
    user__password: String
}
```
```
struct Outcoming {
    user__id: i64,
    verification_message_sent: bool,
    user_authorization_token__can_be_resent_from: i64,
    user_authorization_token__wrong_enter_tries_quantity: i16,
    user_authorization_token__wrong_enter_tries_quantity_limit: i16,
}


user_authorization_token__can_be_resent_from - unixtime after wich it will be allowed to resend the verification message.
```
```
enum Precedent {
    User__WrongEmailOrNicknameOrPassword,
}
```
 - ## UserAuthorization_AuthorizeByLastStep POST /user_authorization/authorize_by_last_step
```
Authorizes user for the last step.
```
```
struct Incoming {
    user__id: i64,
    user_device__id: String,
    user_authorization_token__value: String
}
```
```
struct Outcoming {
    user_access_token_signed: <Data standards>
    user_access_refresh_token_signed: <Data standards>
}
```
```
enum Precedent {
    UserAuthorizationToken__NotFound,
    UserAuthorizationToken__AlreadyExpired,
    UserAuthorizationToken__WrongValue {
        user_authorization_token__wrong_enter_tries_quantity: i16,
    },
    User__NotFound,
}
```
 - ## UserAuthorization_SendEmailForAuthorize POST /user_authorization/send_email_for_authorize
```
Sends email for authorization. (Should be used only if the user does not receive an email.)
```
```
struct Incoming {
    user_device__id: String,
    user__id: i64
}
```
```
struct Outcoming {
    user_authorization_token__can_be_resent_from: i64
}
```
```
enum Precedent {
    User__NotFound,
    UserAuthorizationToken__NotFound,
    UserAuthorizationToken__AlreadyExpired,
    UserAuthorizationToken__TimeToResendHasNotCome,
}
```
 - ## UserAuthorization_ResetPasswordByFirstStep POST /user_authorization/reset_password_by_first_step
```
Resets user password for the first step and send email to user.
```
```
struct Incoming {
    user__email: String,
    user_device__id: String,
}
```
```
struct Outcoming {
    user__id: i64,
    verification_message_sent: bool,
    user_reset_password_token__can_be_resent_from: i64,
    user_reset_password_token__wrong_enter_tries_quantity: i16,
    user_reset_password_token__wrong_enter_tries_quantity_limit: i16,
}
```
```
enum Precedent {
    User__NotFound,
}
```
 - ## UserAuthorization_ResetPasswordBySecondStep POST /user_authorization/reset_password_by_second_step
```
Resets user password for the second step through token value approving.
```
```
struct Incoming {
    user__id: i64,
    user_device__id: String,
    user_reset_password_token__value: String
}
```
```
enum Precedent {
    UserResetPasswordToken__NotFound,
    UserResetPasswordToken__AlreadyExpired,
    UserResetPasswordToken__AlreadyApproved,
    UserResetPasswordToken__WrongValue {
        user_reset_password_token__wrong_enter_tries_quantity: i16,
    },
}
```
 - ## UserAuthorization_ResetPasswordByLastStep POST /user_authorization/reset_password_by_last_step
```
Resets user password for the last step.
```
```
struct Incoming {
    user_device__id: String,
    user__id: i64,
    user__password: String,
    user_reset_password_token__value: String
}
```
```
enum Precedent {
    User__NotFound,
    UserResetPasswordToken__NotFound,
    UserResetPasswordToken__AlreadyExpired,
    UserResetPasswordToken__IsNotApproved,
    UserResetPasswordToken__WrongValue,
}
```
 - ## UserAuthorization_SendEmailForResetPassword POST /user_authorization/send_email_for_reset_password
```
Sends email for reset password.  (Should be used only if the user does not receive an email.)
```
```
struct Incoming {
    user__id: i64,
    user_device__id: String,
}
```
```
struct Outcoming {
    user_registration_token__can_be_resent_from: i64
}
```
```
enum Precedent {
    User__NotFound,
    UserResetPasswordToken__NotFound,
    UserResetPasswordToken__AlreadyExpired,
    UserResetPasswordToken__AlreadyApproved,
    UserResetPasswordToken__TimeToResendHasNotCome,
}
```
 - ## UserAuthorization_RefreshAccessToken POST /user_authorization/refresh_access_token
```
Refreshs user access token.
```
```
struct Incoming {
    user_access_token_signed: <Data standards>
    user_access_refresh_token_signed: <Data standards>
}
```
```
struct Outcoming {
    user_access_token_signed: <Data standards>
    user_access_refresh_token_signed: <Data standards>
}
```
```
enum Precedent {
    UserAccessRefreshToken__NotFound,
    UserAccessRefreshToken__AlreadyExpired,
}
```
# Parameters validation rule.
 - ## user_registration_token__value
```
^[0-9]{6}$ - regular expression.
```
 - ## user_reset_password_token__value
```
^[0-9]{6}$ - regular expression.
```
 - ## user_authorization_token__value
```
^[0-9]{6}$ - regular expression.
```
 - ## user__id
```
>= 0
```
 - ## user__email
```
(?:[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*|"(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21\x23-\x5b\x5d-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])*")@(?:(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?|\[(?:(?:(2(5[0-5]|[0-4][0-9])|1[0-9][0-9]|[1-9]?[0-9]))\.){3}(?:(2(5[0-5]|[0-4][0-9])|1[0-9][0-9]|[1-9]?[0-9])|[a-z0-9-]*[a-z0-9]:(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21-\x5a\x53-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])+)\]) - regular expression.

320 - maximum number of characters.

Lowercase.

```
 - ## user__nickname
```
55 - maximum number of characters.

@ - can not contain this character.

Can not contain whitespace character.

Can not be empty.

Lowercase.

```
 - ## user__password
```
7 - minimum number of characters.

65 - maximum number of characters.

Can not contain whitespace character.

Can not be equal to user__email.

Can not be equal to user__nickname.
```
 - ## channel__id
```
>= 0
```
 - ## channel__name
```
75 - maximum number of characters.
```
 - ## channel__linked_name
```
Lowercase.
```
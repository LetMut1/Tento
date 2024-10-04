# Data standards
- `user_access_token_encoded`:
```
struct UserAccessTokenEncoded {
    serialized: Vec<u8>,
    encoded: Vec<u8>,
}
```
- `user_access_refresh_token_encoded`:
```
struct UserAccessRefreshTokenEncoded(Vec<u8>)
```
- `sort_order`:
```
0 - is equal to 'ASC'
1 - is equal to 'DESC'
```
# Request standards
- All data is transferred in `HTTP Body`
- Every request should contain this `HTTP Header`s:
```
- content-type: application/octet-stream
- content-length: <calculate>
```
# Response standards
- All data is transferred in `HTTP Body`
- Every response should contain this `HTTP Header`s:
```
- content-type: application/octet-stream
- content-length: <calculate>
```
 - The permanent general structure of the each response with `status code` equal to `200` looks like:
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
- `Outcoming` structures written under each API endpoint will be nested in the `Filled` variant of `enum Data<D>`.
- `HTTP Status code` unequal to `200` have not got `HTTP Body`.

<br/><br/>

# API for authorized user.
 Every endpoint at this area requires an existing of `user_access_token_encoded`.
 - ## UserAuthorization_DeauthorizeFromOneDevice POST /user_authorization/deauthorize_from_one_device
```
Deauthorizes user from one device.
```
```
Request data:
struct Incoming {
    user_access_token_encoded: <Data standards>
}
```
```
Result data: absent.
```
```
Communication codes:
- APPLICATION_USER_ACCESS_TOKEN__ALREADY_EXPIRED
- APPLICATION_USER_ACCESS_TOKEN__IN_APPLICATION_USER_ACCESS_TOKEN_BLACK_LIST
```
 - ## UserAuthorization_DeauthorizeFromAllDevicec POST /user_authorization/deauthorize_from_all_devices
```
Deauthorizes user from all devices.
```
```
Request data:
struct Incoming {
    user_access_token_encoded: <Data standards>
}
```
```
Result data: absent.
```
```
Communication codes:
- APPLICATION_USER_ACCESS_TOKEN__ALREADY_EXPIRED
- APPLICATION_USER_ACCESS_TOKEN__IN_APPLICATION_USER_ACCESS_TOKEN_BLACK_LIST
```
 - ## Channel_GetOneById POST (GET) /channel/get_one_by_id
```
Returns channel data by id.
```
```
Request data:
struct Incoming {
    user_access_token_encoded: <Data standards>
    channel__id: i64
}
```
```
Result data:

struct Outcoming {
    channel: Channel,
    channel_inner_link_registry: Vec<ChannelInnerLink>,
    channel_outer_link_registry: Vec<ChannelOuterLink>,
}

struct Channel {
    channel__owner: i64,
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
    channel__viewing_quantity: i64
}

struct ChannelInnerLink {
    channel_inner_link__to: i64
}

struct ChannelOuterLink {
    channel_outer_link__alias: String,
    channel_outer_link__address: String
}
```
```
Communication codes:
- APPLICATION_USER_ACCESS_TOKEN__ALREADY_EXPIRED
- APPLICATION_USER_ACCESS_TOKEN__IN_APPLICATION_USER_ACCESS_TOKEN_BLACK_LIST
- Channel_NOT_FOUND
- Channel_IS_CLOSED
```
 - ## Channel_GetManyByNameInSubscription POST (GET) /channel/get_many_by_name_in_subscription
```
Returns channels the user is subscribed to by name.
```
```
Request data:
struct Incoming {
    user_access_token_encoded: <Data standards>
    channel__name: String,
    requery___channel__name: Option<String>,
    limit: i16
}


requery___channel__name - an alternative for offset. Used only for requering with persistent channel__name. The value must be equal to the last channel__name of channel registry in received early response.

Incoming parameters validation rule:
- requery___channel__name:
    -- same as channel__name.
- limit:
    -- [1, 100] values.
```
```
Result data:
struct Outcoming {
    common_registry: Vec<Common1>
}

struct Common1 {
    channel: Channel1,
    is_user_subscribed: bool
}

struct Channel1 {
    channel__id: i64,
    channel__name: String,
    channel__linked_name: String,
    channel__access_modifier: i16,
    channel__visability_modifier: i16,
    channel__cover_image_path: Option<String>,
    channel__background_image_path: Option<String>
}
```
```
Communication codes:
- APPLICATION_USER_ACCESS_TOKEN__ALREADY_EXPIRED
- APPLICATION_USER_ACCESS_TOKEN__IN_APPLICATION_USER_ACCESS_TOKEN_BLACK_LIST
```
 - ## Channel_GetManyBySubscription POST (GET) /channel/get_many_by_subscription
```
Returns channels the user is subscribed to.
```
```
Request data:
struct Incoming {
    user_access_token_encoded: <Data standards>
    requery___channel__id: Option<i64>,
    limit: i16
}


requery___channel__id - an alternative for offset. The value must be equal to the last channel__id of channel registry in received early response.

Incoming parameters validation rule:
- requery___channel__id:
    -- same as channel__id.
- limit:
    -- [1, 100] values.
```
```
Result data:
struct Outcoming {
    common_registry: Vec<Common1>
}

struct Common1 {
    channel: Channel1,
    is_user_subscribed: bool
}

struct Channel1 {
    channel__id: i64,
    channel__name: String,
    channel__linked_name: String,
    channel__access_modifier: i16,
    channel__visability_modifier: i16,
    channel__cover_image_path: Option<String>,
    channel__background_image_path: Option<String>
}
```
```
Communication codes:
- APPLICATION_USER_ACCESS_TOKEN__ALREADY_EXPIRED
- APPLICATION_USER_ACCESS_TOKEN__IN_APPLICATION_USER_ACCESS_TOKEN_BLACK_LIST
```
 - ## Channel_GetManyPublicByName POST (GET) /channel/get_many_public_by_name
```
Returns public channels by name.
```
```
Request data:
struct Incoming {
    user_access_token_encoded: <Data standards>
    channel__name: String,
    requery___channel__name: Option<String>,
    limit: i16
}


requery___channel__name - an alternative for offset. Used only for requering with persistent channel__name. The value must be equal to the last channel__name of channel registry in received early response.

Incoming parameters validation rule:
- requery___channel__name:
    -- same as channel__name.
- limit:
    -- [1, 100] values.
```
```
Result data:
struct Outcoming {
    common_registry: Vec<Common1>
}

struct Common1 {
    channel: Channel1,
    is_user_subscribed: bool
}

struct Channel1 {
    channel__id: i64,
    channel__name: String,
    channel__linked_name: String,
    channel__access_modifier: i16,
    channel__visability_modifier: i16,
    channel__cover_image_path: Option<String>,
    channel__background_image_path: Option<String>
}
```
```
Communication codes:
- APPLICATION_USER_ACCESS_TOKEN__ALREADY_EXPIRED
- APPLICATION_USER_ACCESS_TOKEN__IN_APPLICATION_USER_ACCESS_TOKEN_BLACK_LIST
```
 - ## ChannelSubscription_Create POST /channel_subscription/create
```
Subscribes user to channel.
```
```
Request data:
struct Incoming {
    user_access_token_encoded: <Data standards>
    channel__id: i64
}
```
```
Result data: absent.
```
```
Communication codes:
- UserAuthorization_IS_CHANNEL_OWNER
- APPLICATION_USER_ACCESS_TOKEN__ALREADY_EXPIRED
- APPLICATION_USER_ACCESS_TOKEN__IN_APPLICATION_USER_ACCESS_TOKEN_BLACK_LIST
- Channel_IS_CLOSED
- Channel_NOT_FOUND
```
<br/><br/>

# API for not authorized user.
 - ## UserAuthorization_CheckEmailForExisting POST (GET) /user_authorization/check_email_for_existing
```
Checks user email for existing.
```
```
Request data:
struct Incoming {
    user__email: String
}
```
```
Result data:
struct Outcoming {
    result: bool
}
```
```
Communication codes: absent.
```
 - ## UserAuthorization_CheckNicknameForExisting POST (GET) /user_authorization/check_nickname_for_existing
```
Checks user nickname for existing.
```
```
Request data:
struct Incoming {
    user__nickname: String
}
```
```
Result data:
struct Outcoming {
    result: bool
}
```
```
Communication codes: absent.
```
 - ## UserAuthorization_RegisterByFirstStep POST /user_authorization/register_by_first_step
```
Registers user for the first step and sends email to user.
```
```
Request data:
struct Incoming {
    user__email: String,
    user_device__id: String
}
```
```
Result data:
struct Outcoming {
    verification_message_sent: bool,
    user_registration_token__can_be_resent_from: i64
}


verification_message_sent - determines if a verification message has been sent. The value will be false only if the request was retried
with unchanged parameters without waiting a certain amount of time.

user_registration_token__can_be_resent_from - unixtime after wich it will be allowed to resend the verification message.
```
```
Communication codes:
- UserAuthorization_EMAIL_ALREADY_EXIST
```
- ## UserAuthorization_RegisterBySecondStep POST /user_authorization/register_by_second_step
```
Registers user for the second step through token value approving.
```
```
Request data:
struct Incoming {
    user__email: String,
    user_device__id: String,
    user_registration_token__value: String
}
```
```
Result data: absent.
```
```
Communication codes:
- APPLICATION_USER_REGISTRATION_TOKEN__NOT_FOUND
- APPLICATION_USER_REGISTRATION_TOKEN__ALREADY_APPROVED
- APPLICATION_USER_REGISTRATION_TOKEN__ALREADY_EXPIRED
- APPLICATION_USER_REGISTRATION_TOKEN__WRONG_VALUE
```

 - ## UserAuthorization_RegisterByLastStep POST /user_authorization/register_by_last_step
```
Registers user for the last step.
```
```
Request data:
struct Incoming {
    user_device__id: String,
    user__nickname: String,
    user__password: String,
    user__email: String,
    user_registration_token__value: String
}
```
```
Result data:
struct Outcoming {
    user_access_token_encoded: <Data standards>
    user_access_refresh_token_encoded: <Data standards>
}
```
```
Communication codes:
- UserAuthorization_NICKNAME_ALREADY_EXIST
- UserAuthorization_EMAIL_ALREADY_EXIST
- APPLICATION_USER_REGISTRATION_TOKEN__NOT_FOUND
- APPLICATION_USER_REGISTRATION_TOKEN__IS_NOT_APPROVED
- APPLICATION_USER_REGISTRATION_TOKEN__WRONG_VALUE
- APPLICATION_USER_REGISTRATION_TOKEN__ALREADY_EXPIRED
```
 - ## UserAuthorization_SendEmailForRegister POST /user_authorization/send_email_for_register
```
Sends email for register. (Should be used only if the user does not receive an email.)
```
```
Request data:
struct Incoming {
    user__email: String,
    user_device__id: String
}
```
```
Result data:
struct Outcoming {
    user_registration_token__can_be_resent_from: i64
}


user_registration_token__can_be_resent_from - unixtime after wich it will be allowed to resend the verification message.
```
```
Communication codes:
- APPLICATION_USER_REGISTRATION_TOKEN__NOT_FOUND
- APPLICATION_USER_REGISTRATION_TOKEN__ALREADY_APPROVED
- APPLICATION_USER_REGISTRATION_TOKEN__ALREADY_EXPIRED
- APPLICATION_USER_REGISTRATION_TOKEN__TIME_TO_RESEND_HAS_NOT_COME
```
 - ## UserAuthorization_AuthorizeByFirstStep POST /user_authorization/authorize_by_first_step
```
Authorizes user for the first step and send email to user.
```
```
Request data:
struct Incoming {
    user_device__id: String,
    user__email___or___user__nickname: String,
    user__password: String
}
```
```
Result data:
struct Outcoming {
    user__id: i64,
    verification_message_sent: bool,
    user_authorization_token__can_be_resent_from: i64
}

user_authorization_token__can_be_resent_from - unixtime after wich it will be allowed to resend the verification message.
```
```
Communication codes:
- UserAuthorization_WRONG_EMAIL_OR_NICKNAME_OR_PASSWORD
```
 - ## UserAuthorization_AuthorizeByLastStep POST /user_authorization/authorize_by_last_step
```
Authorizes user for the last step.
```
```
Request data:
struct Incoming {
    user__id: i64,
    user_device__id: String,
    user_authorization_token__value: String
}
```
```
Result data:
struct Outcoming {
    user_access_token_encoded: <Data standards>
    user_access_refresh_token_encoded: <Data standards>
}
```
```
Communication codes:
- APPLICATION_USER_AUTHORIZATION_TOKEN__NOT_FOUND
- APPLICATION_USER_AUTHORIZATION_TOKEN__ALREADY_EXPIRED
- APPLICATION_USER_AUTHORIZATION_TOKEN__WRONG_VALUE
- UserAuthorization_NOT_FOUND
```
 - ## UserAuthorization_SendEmailForAuthorize POST /user_authorization/send_email_for_authorize
```
Sends email for authorization. (Should be used only if the user does not receive an email.)
```
```
Request data:
struct Incoming {
    user_device__id: String,
    user__id: i64
}
```
```
Result data:
struct Outcoming {
    user_authorization_token__can_be_resent_from: i64
}
```
```
Communication codes:
- UserAuthorization_NOT_FOUND
- APPLICATION_USER_AUTHORIZATION_TOKEN__NOT_FOUND
- APPLICATION_USER_AUTHORIZATION_TOKEN__ALREADY_EXPIRED
- APPLICATION_USER_AUTHORIZATION_TOKEN__TIME_TO_RESEND_HAS_NOT_COME
```
 - ## UserAuthorization_ResetPasswordByFirstStep POST /user_authorization/reset_password_by_first_step
```
Resets user password for the first step and send email to user.
```
```
Request data:
struct Incoming {
    user__email: String,
    user_device__id: String
}
```
```
Result data:
struct Outcoming {
    user__id: i64,
    verification_message_sent: bool,
    user_reset_password_token__can_be_resent_from: i64
}
```
```
Communication codes:
- UserAuthorization_NOT_FOUND
```
 - ## UserAuthorization_ResetPasswordBySecondStep POST /user_authorization/reset_password_by_second_step
```
Resets user password for the second step through token value approving.
```
```
Request data:
struct Incoming {
    user__id: i64,
    user_device__id: String,
    user_reset_password_token__value: String
}
```
```
Result data: absent.
```
```
Communication codes:
- APPLICATION_USER_RESET_PASSWORD_TOKEN__NOT_FOUND
- APPLICATION_USER_RESET_PASSWORD_TOKEN__ALREADY_EXPIRED
- APPLICATION_USER_RESET_PASSWORD_TOKEN__ALREADY_APPROVED
- APPLICATION_USER_RESET_PASSWORD_TOKEN__WRONG_VALUE
```
 - ## UserAuthorization_ResetPasswordByLastStep POST /user_authorization/reset_password_by_last_step
```
Resets user password for the last step.
```
```
Request data:
struct Incoming {
    user_device__id: String,
    user__id: i64,
    user__password: String,
    user_reset_password_token__value: String
}
```
```
Result data: absent.
```
```
Communication codes:
- UserAuthorization_NOT_FOUND
- APPLICATION_USER_RESET_PASSWORD_TOKEN__NOT_FOUND
- APPLICATION_USER_RESET_PASSWORD_TOKEN__ALREADY_EXPIRED
- APPLICATION_USER_RESET_PASSWORD_TOKEN__IS_NOT_APPROVED
- APPLICATION_USER_RESET_PASSWORD_TOKEN__WRONG_VALUE
```
 - ## UserAuthorization_SendEmailForResetPassword POST /user_authorization/send_email_for_reset_password
```
Sends email for reset password.  (Should be used only if the user does not receive an email.)
```
```
Request data:
struct Incoming {
    user__id: i64,
    user_device__id: String,
}
```
```
Result data:
struct Outcoming {
    user_registration_token__can_be_resent_from: i64
}
```
```
Communication codes:
- UserAuthorization_NOT_FOUND
- APPLICATION_USER_RESET_PASSWORD_TOKEN__NOT_FOUND
- APPLICATION_USER_RESET_PASSWORD_TOKEN__ALREADY_APPROVED
- APPLICATION_USER_RESET_PASSWORD_TOKEN__ALREADY_EXPIRED
- APPLICATION_USER_RESET_PASSWORD_TOKEN__TIME_TO_RESEND_HAS_NOT_COME
```
 - ## UserAuthorization_RefreshAccessToken POST /user_authorization/refresh_access_token
```
Refreshs user access token.
```
```
Request data:
struct Incoming {
    user_access_token_encoded: <Data standards>
    user_access_refresh_token_encoded: <Data standards>
}
```
```
Result data:
struct Outcoming {
    user_access_token_encoded: <Data standards>
    user_access_refresh_token_encoded: <Data standards>
}
```
```
Communication codes:
- APPLICATION_USER_ACCESS_REFRESH_TOKEN__ALREADY_EXPIRED
- APPLICATION_USER_ACCESS_REFRESH_TOKEN__NOT_FOUND
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

```
 - ## user__nickname
```
55 - maximum number of characters.

@ - can not contain this character.

Can not contain whitespace character.

Can not be empty.

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
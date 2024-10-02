# Data standards
- `application_user_access_token_encoded`:
```
struct UserAccessTokenEncoded {
    serialized: Vec<u8>,
    encoded: Vec<u8>,
}
```
- `application_user_access_refresh_token_encoded`:
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
- `Result data` structures written under each API endpoint will be nested in the `data` field in the `struct Data<S>`.
- Existing `communication_code`s can be founded here:
```
/application/application/source/presentation_layer/data/communication_code_registry.rs
```
- Existing `http_route`s can be founded here:
```
/application/application/source/presentation_layer/data/http_route_registry.rs
```
- `HTTP status code` unequal to `200` (it is [`400`, `599`]) have not got `HTTP body`.

<br/><br/>

# API for authorized application user.
 Every endpoint at this area requires an existing of `application_user_access_token_encoded`.
 - ## APPLICATION_USER__DEAUTHORIZE_FROM_ONE_DEVICE POST
```
Deauthorizes application user from one device.
```
```
Request data:
struct Incoming {
    application_user_access_token_encoded: <Data standards>
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
 - ## APPLICATION_USER__DEAUTHORIZE_FROM_ALL_DEVICE POST
```
Deauthorizes application user from all devices.
```
```
Request data:
struct Incoming {
    application_user_access_token_encoded: <Data standards>
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
 - ## CHANNEL__GET_ONE_BY_ID POST (GET functional)
```
Returns channel data by id.
```
```
Request data:
struct Incoming {
    application_user_access_token_encoded: <Data standards>
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
- CHANNEL__NOT_FOUND
- CHANNEL__IS_CLOSED
```
 - ## CHANNEL__GET_MANY_BY_NAME_IN_SUBSCRIPTION POST (GET functional)
```
Returns channels the user is subscribed to by name.
```
```
Request data:
struct Incoming {
    application_user_access_token_encoded: <Data standards>
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
    is_application_user_subscribed: bool
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
 - ## CHANNEL__GET_MANY_BY_SUBSCRIPTION POST (GET functional)
```
Returns channels the user is subscribed to.
```
```
Request data:
struct Incoming {
    application_user_access_token_encoded: <Data standards>
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
    is_application_user_subscribed: bool
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
 - ## CHANNEL__GET_MANY_PUBLIC_BY_NAME POST (GET functional)
```
Returns public channels by name.
```
```
Request data:
struct Incoming {
    application_user_access_token_encoded: <Data standards>
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
    is_application_user_subscribed: bool
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
 - ## CHANNEL_SUBSCRIPTION__CREATE POST (GET functional)
```
Subscribes application user to channel.
```
```
Request data:
struct Incoming {
    application_user_access_token_encoded: <Data standards>
    channel__id: i64
}
```
```
Result data: absent.
```
```
Communication codes:
- APPLICATION_USER__IS_CHANNEL_OWNER
- APPLICATION_USER_ACCESS_TOKEN__ALREADY_EXPIRED
- APPLICATION_USER_ACCESS_TOKEN__IN_APPLICATION_USER_ACCESS_TOKEN_BLACK_LIST
- CHANNEL__IS_CLOSED
- CHANNEL__NOT_FOUND
```
<br/><br/>

# API for not authorized application user.
 - ## APPLICATION_USER__CHECK_EMAIL_FOR_EXISTING POST (GET functional)
```
Checks application user email for existing.
```
```
Request data:
struct Incoming {
    application_user__email: String
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
 - ## APPLICATION_USER__CHECK_NICKNAME_FOR_EXISTING POST (GET functional)
```
Checks application user nickname for existing.
```
```
Request data:
struct Incoming {
    application_user__nickname: String
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
 - ## APPLICATION_USER__REGISTER_BY_FIRST_STEP POST
```
Registers application user for the first step and sends email to user.
```
```
Request data:
struct Incoming {
    application_user__email: String,
    application_user_device__id: String
}
```
```
Result data:
struct Outcoming {
    verification_message_sent: bool,
    application_user_registration_token__can_be_resent_from: i64
}


verification_message_sent - determines if a verification message has been sent. The value will be false only if the request was retried
with unchanged parameters without waiting a certain amount of time.

application_user_registration_token__can_be_resent_from - unixtime after wich it will be allowed to resend the verification message.
```
```
Communication codes:
- APPLICATION_USER__EMAIL_ALREADY_EXIST
```
- ## APPLICATION_USER__REGISTER_BY_SECOND_STEP POST
```
Registers application user for the second step through token value approving.
```
```
Request data:
struct Incoming {
    application_user__email: String,
    application_user_device__id: String,
    application_user_registration_token__value: String
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

 - ## APPLICATION_USER__REGISTER_BY_LAST_STEP POST
```
Registers application user for the last step.
```
```
Request data:
struct Incoming {
    application_user_device__id: String,
    application_user__nickname: String,
    application_user__password: String,
    application_user__email: String,
    application_user_registration_token__value: String
}
```
```
Result data:
struct Outcoming {
    application_user_access_token_encoded: <Data standards>
    application_user_access_refresh_token_encoded: <Data standards>
}
```
```
Communication codes:
- APPLICATION_USER__NICKNAME_ALREADY_EXIST
- APPLICATION_USER__EMAIL_ALREADY_EXIST
- APPLICATION_USER_REGISTRATION_TOKEN__NOT_FOUND
- APPLICATION_USER_REGISTRATION_TOKEN__IS_NOT_APPROVED
- APPLICATION_USER_REGISTRATION_TOKEN__WRONG_VALUE
- APPLICATION_USER_REGISTRATION_TOKEN__ALREADY_EXPIRED
```
 - ## APPLICATION_USER__SEND_EMAIL_FOR_REGISTER POST
```
Sends email for register. (Should be used only if the user does not receive an email.)
```
```
Request data:
struct Incoming {
    application_user__email: String,
    application_user_device__id: String
}
```
```
Result data:
struct Outcoming {
    application_user_registration_token__can_be_resent_from: i64
}


application_user_registration_token__can_be_resent_from - unixtime after wich it will be allowed to resend the verification message.
```
```
Communication codes:
- APPLICATION_USER_REGISTRATION_TOKEN__NOT_FOUND
- APPLICATION_USER_REGISTRATION_TOKEN__ALREADY_APPROVED
- APPLICATION_USER_REGISTRATION_TOKEN__ALREADY_EXPIRED
- APPLICATION_USER_REGISTRATION_TOKEN__TIME_TO_RESEND_HAS_NOT_COME
```
 - ## APPLICATION_USER__AUTHORIZE_BY_FIRST_STEP POST
```
Authorizes application user for the firs step and send email to user.
```
```
Request data:
struct Incoming {
    application_user_device__id: String,
    application_user__email___or___application_user__nickname: String,
    application_user__password: String
}
```
```
Result data:
struct Outcoming {
    application_user__id: i64,
    verification_message_sent: bool,
    application_user_authorization_token__can_be_resent_from: i64
}

application_user_authorization_token__can_be_resent_from - unixtime after wich it will be allowed to resend the verification message.
```
```
Communication codes:
- APPLICATION_USER__WRONG_EMAIL_OR_NICKNAME_OR_PASSWORD
```
 - ## APPLICATION_USER__AUTHORIZE_BY_LAST_STEP POST
```
Authorizes application user for the last step.
```
```
Request data:
struct Incoming {
    application_user__id: i64,
    application_user_device__id: String,
    application_user_authorization_token__value: String
}
```
```
Result data:
struct Outcoming {
    application_user_access_token_encoded: <Data standards>
    application_user_access_refresh_token_encoded: <Data standards>
}
```
```
Communication codes:
- APPLICATION_USER_AUTHORIZATION_TOKEN__NOT_FOUND
- APPLICATION_USER_AUTHORIZATION_TOKEN__ALREADY_EXPIRED
- APPLICATION_USER_AUTHORIZATION_TOKEN__WRONG_VALUE
- APPLICATION_USER__NOT_FOUND
```
 - ## APPLICATION_USER__SEND_EMAIL_FOR_AUTHORIZE POST
```
Sends email for authorization. (Should be used only if the user does not receive an email.)
```
```
Request data:
struct Incoming {
    application_user_device__id: String,
    application_user__id: i64
}
```
```
Result data:
struct Outcoming {
    application_user_authorization_token__can_be_resent_from: i64
}
```
```
Communication codes:
- APPLICATION_USER__NOT_FOUND
- APPLICATION_USER_AUTHORIZATION_TOKEN__NOT_FOUND
- APPLICATION_USER_AUTHORIZATION_TOKEN__ALREADY_EXPIRED
- APPLICATION_USER_AUTHORIZATION_TOKEN__TIME_TO_RESEND_HAS_NOT_COME
```
 - ## APPLICATION_USER__RESET_PASSWORD_BY_FIRST_STEP POST
```
Resets application user password for the first step and send email to user.
```
```
Request data:
struct Incoming {
    application_user__email: String,
    application_user_device__id: String
}
```
```
Result data:
struct Outcoming {
    application_user__id: i64,
    verification_message_sent: bool,
    application_user_reset_password_token__can_be_resent_from: i64
}
```
```
Communication codes:
- APPLICATION_USER__NOT_FOUND
```
 - ## APPLICATION_USER__RESET_PASSWORD_BY_SECOND_STEP POST
```
Resets application user password for the second step through token value approving.
```
```
Request data:
struct Incoming {
    application_user__id: i64,
    application_user_device__id: String,
    application_user_reset_password_token__value: String
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
 - ## APPLICATION_USER__RESET_PASSWORD_BY_LAST_STEP POST
```
Resets application user password for the last step.
```
```
Request data:
struct Incoming {
    application_user_device__id: String,
    application_user__id: i64,
    application_user__password: String,
    application_user_reset_password_token__value: String
}
```
```
Result data: absent.
```
```
Communication codes:
- APPLICATION_USER__NOT_FOUND
- APPLICATION_USER_RESET_PASSWORD_TOKEN__NOT_FOUND
- APPLICATION_USER_RESET_PASSWORD_TOKEN__ALREADY_EXPIRED
- APPLICATION_USER_RESET_PASSWORD_TOKEN__IS_NOT_APPROVED
- APPLICATION_USER_RESET_PASSWORD_TOKEN__WRONG_VALUE
```
 - ## APPLICATION_USER__SEND_EMAIL_FOR_RESET_PASSWORD POST
```
Sends email for reset password.  (Should be used only if the user does not receive an email.)
```
```
Request data:
struct Incoming {
    application_user__id: i64,
    application_user_device__id: String,
}
```
```
Result data:
struct Outcoming {
    application_user_registration_token__can_be_resent_from: i64
}
```
```
Communication codes:
- APPLICATION_USER__NOT_FOUND
- APPLICATION_USER_RESET_PASSWORD_TOKEN__NOT_FOUND
- APPLICATION_USER_RESET_PASSWORD_TOKEN__ALREADY_APPROVED
- APPLICATION_USER_RESET_PASSWORD_TOKEN__ALREADY_EXPIRED
- APPLICATION_USER_RESET_PASSWORD_TOKEN__TIME_TO_RESEND_HAS_NOT_COME
```
 - ## APPLICATION_USER__REFRESH_ACCESS_TOKEN POST
```
Refreshs application user access token.
```
```
Request data:
struct Incoming {
    application_user_access_token_encoded: <Data standards>
    application_user_access_refresh_token_encoded: <Data standards>
}
```
```
Result data:
struct Outcoming {
    application_user_access_token_encoded: <Data standards>
    application_user_access_refresh_token_encoded: <Data standards>
}
```
```
Communication codes:
- APPLICATION_USER_ACCESS_REFRESH_TOKEN__ALREADY_EXPIRED
- APPLICATION_USER_ACCESS_REFRESH_TOKEN__NOT_FOUND
```
# Parameters validation rule.
 - ## application_user_registration_token__value
```
^[0-9]{6}$ - regular expression.
```
 - ## application_user_reset_password_token__value
```
^[0-9]{6}$ - regular expression.
```
 - ## application_user_authorization_token__value
```
^[0-9]{6}$ - regular expression.
```
 - ## application_user__id
```
>= 0
```
 - ## application_user__email
```
(?:[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*|"(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21\x23-\x5b\x5d-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])*")@(?:(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?|\[(?:(?:(2(5[0-5]|[0-4][0-9])|1[0-9][0-9]|[1-9]?[0-9]))\.){3}(?:(2(5[0-5]|[0-4][0-9])|1[0-9][0-9]|[1-9]?[0-9])|[a-z0-9-]*[a-z0-9]:(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21-\x5a\x53-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])+)\]) - regular expression.

320 - maximum number of characters.

```
 - ## application_user__nickname
```
55 - maximum number of characters.

@ - can not contain this character.

Can not contain whitespace character.

Can not be empty.

```
 - ## application_user__password
```
7 - minimum number of characters.

65 - maximum number of characters.

Can not contain whitespace character.

Can not be equal to application_user__email.

Can not be equal to application_user__nickname.
```
 - ## channel__id
```
>= 0
```
 - ## channel__name
```
75 - maximum number of characters.
```
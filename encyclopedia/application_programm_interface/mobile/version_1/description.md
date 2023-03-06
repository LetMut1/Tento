# Request standards
 - All data is transferred in `HTTP body` as `bytes` in encoded with `MessagePack protocol` form.
 - Every request should contain this `HTTP header`s:
```
- content-type: application/octet-stream
- content-length: ...
- x-content-type-options: nosniff
```
 - Values of variable for `sort_order` looks like:
```
0 - is equal to 'ASC'
1 - is equal to 'DESC'
```

# Response standards
- All data is transferred in `HTTP body` as `bytes` in encoded with `MessagePack protocol` form.
- Every response should contain this `HTTP header`s:
```
- content-type: application/octet-stream
- x-content-type-options: nosniff
- content-length: ...
```
 - The permanent general structure of the each response with `HTTP status code` equal to `200` looks like:
```rust
struct UnifiedReport<S>
{
    data: Option<Data<S>>,
    communication_code: Option<i64>
}

struct Data<S>
{
    data: Option<S>
}
```
- `Result data` structures written under each API endpoint will be nested in the `data` field in the `struct Data<S>`.
- Existing `communication_code`s can be founded here:
```
/project/application/source/presentation_layer/data/communication_code_registry.rs
```
- Existing `http_route`s can be founded here:
```
/project/application/source/presentation_layer/data/http_route_registry.rs
```
- `HTTP status code` unequal to `200` (it is [`400`, `599`]) have not got `HTTP body`

<br/><br/>

# API for authorized application user.
```
 Every endpoint at this area requires an existing of access token.
```
 - ## VERSION_1__APPLICATION_USER__DEAUTHORIZE_FROM_ONE_DEVICE POST
```
Deauthorizes application user from one device.
```
```rust
Request data:
struct Incoming {
    application_user_access_token_deserialized_form: String
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
 - ## VERSION_1__APPLICATION_USER__DEAUTHORIZE_FROM_ALL_DEVICE POST
```
Deauthorizes application user from all devices.
```
```rust
Request data:
struct Incoming {
    application_user_access_token_deserialized_form: String
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
 - ## VERSION_1__CHANNEL__GET_ONE_BY_ID POST (GET functional)
```
Returns channel data by id.
```
```rust
Request data:
struct Incoming {
    application_user_access_token_deserialized_form: String,
    channel_id: i64
}
```
```rust
Result data:
struct Outcoming {
    channel_owner: i64,
    channel_name: String,
    channel_linked_name: String,
    channel_description: Option<String>,
    channel_is_private: bool,
    channel_orientation: Vec<i16>,
    channel_personalization_image_path: String,
    channel_subscribers_quantity: i64,
    channel_marks_quantity: i64,
    channel_viewing_quantity: i64
}
```
```
Communication codes:
- APPLICATION_USER_ACCESS_TOKEN__ALREADY_EXPIRED
- APPLICATION_USER_ACCESS_TOKEN__IN_APPLICATION_USER_ACCESS_TOKEN_BLACK_LIST
- CHANNEL__NOT_FOUND
- CHANNEL__IS_PRIVATE
```
 - ## VERSION_1__CHANNEL__GET_MANY_BY_NAME POST (GET functional)
```
Returns channels by name.
```
```rust
Request data:
struct Incoming {
    application_user_access_token_deserialized_form: String,
    search_in_subscriptions: Option<SearchInSubscriptions>,
    search_in_all: Option<SearchInAll>
}

struct SearchInSubscriptions {
    channel_name: String,
    requery_channel_name: Option<String>,
    limit: i16
}

struct SearchInAll {
    channel_name: String,
    requery_channel_name: Option<String>,
    limit: i16
}
```
```
If we have already requested information about ALL channels to which the user is subscribed through some other method, then the search for the channels to which the user is subscribed must occur on the front side - that is, search_in_subscription must be equal to null.

If we have not yet requested information about ALL channels to which the user is subscribed, then the first search request must have search_in_subscribers and search_in_all with the same parameters. The first request is the request where .channel_name changes its value.
If the number of records in search_in_subscribers_result is equal to the search_in_subscribers.limit value, then not all entities were returned according to the specified parameters. If the user continues to make additional requests after that, then search_in_subscribers must not be null, and search_in_all must be null.
If the number of entries in search_in_subscribers_result is less than search_in_subscribers.limit, it means that all entities with the specified parameters have already been returned. If after that the user continues to make additional requests, then search_in_subscribers should already be equal to null, and search_in_all should not be equal to null.
The idea is to first query all channels the user is subscribed to, and then query all public channels the user is subscribed to.

search_in_subscriptions.requery_channel_name and search_in_all.requery_channel_name - an alternative for offset. Used only for requering (make additioanl request) with persistent .channel_name. The value must be equal to the last channel_name of channel registry in received early response.

Incoming parameters validation rule:
- search_in_subscriptions and search_in_all can not be null at the same time.
- search_in_subscriptions.requery_channel_name:
    -- same as channel_name.
- search_in_subscriptions.limit:
    -- [1, 100] values.
- search_in_all.requery_channel_name:
    -- same as channel_name.
- search_in_all.limit:
    -- [1, 100] values.
```
```rust
Result data:
struct Outcoming {
    search_in_subscriptions_result: Option<Vec<Channel>>,
    search_in_all_result: Option<Vec<Channel>>
}

struct Channel {
    channel_id: i64,
    channel_name: String,
    channel_linked_name: String,
    channel_personalization_image_path: String
}
```
```
Communication codes:
- APPLICATION_USER_ACCESS_TOKEN__ALREADY_EXPIRED
- APPLICATION_USER_ACCESS_TOKEN__IN_APPLICATION_USER_ACCESS_TOKEN_BLACK_LIST
```
 - ## VERSION_1__CHANNEL__GET_MANY_BY_SUBSCRIPTION POST (GET functional)
```
Returns channels by subscription.
```
```rust
Request data:
struct Incoming {
    application_user_access_token_deserialized_form: String,
    requery_channel_id: Option<i64>,
    limit: i16
}
```
```
requery_channel_id - an alternative for offset. The value must be equal to the last channel_id of channel registry in received early response.

Incoming parameters validation rule:
- requery_channel_id:
    -- same as channel_id.
- limit:
    -- [1, 100] values.
```
```rust
Result data:
struct Outcoming {
    channel_registry: Vec<Channel>
}

struct Channel {
    channel_id: i64,
    channel_name: String,
    channel_linked_name: String,
    channel_personalization_image_path: String
}
```
```
Communication codes:
- APPLICATION_USER_ACCESS_TOKEN__ALREADY_EXPIRED
- APPLICATION_USER_ACCESS_TOKEN__IN_APPLICATION_USER_ACCESS_TOKEN_BLACK_LIST
```
<br/><br/>

# API for not authorized application user.
 - ## VERSION_1__APPLICATION_USER__CHECK_EMAIL_FOR_EXISTING POST (GET functional)
```
Checks application user email for existing.
```
```rust
Request data:
struct Incoming {
    application_user_email: String
}
```
```rust
Result data:
struct Outcoming {
    result: bool
}
```
```
Communication codes: absent.
```
 - ## VERSION_1__APPLICATION_USER__CHECK_NICKNAME_FOR_EXISTING POST (GET functional)
```
Checks application user nickname for existing.
```
```rust
Request data:
struct Incoming {
    application_user_nickname: String
}
```
```rust
Result data:
struct Outcoming {
    result: bool
}
```
```
Communication codes: absent.
```
 - ## VERSION_1__APPLICATION_USER__REGISTER_BY_FIRST_STEP POST
```
Registers application user for the first step and sends email to user.
```
```rust
Request data:
struct Incoming {
    application_user_email: String
}
```
```
Result data: absent.
```
```
Communication codes:
- APPLICATION_USER__EMAIL_ALREADY_EXIST
```
- ## VERSION_1__APPLICATION_USER__REGISTER_BY_SECOND_STEP POST
```
Registers application user for the second step through token value approving.
```
```rust
Request data:
struct Incoming {
    application_user_email: String,
    application_user_registration_token_value: String
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

 - ## VERSION_1__APPLICATION_USER__REGISTER_BY_LAST_STEP POST
```
Registers application user for the last step.
```
```rust
Request data:
struct Incoming {
    application_user_device_id: String,
    application_user_nickname: String,
    application_user_password: String,
    application_user_email: String,
    application_user_registration_token_value: String
}
```
```rust
Result data:
struct Outcoming {
    application_user_access_token_deserialized_form: String,
    application_user_access_refresh_token_deserialized_form: String
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
 - ## VERSION_1__APPLICATION_USER__SEND_EMAIL_FOR_REGISTER POST
```
Sends email for register. (Should be used only if the user does not receive an email.)
```
```rust
Request data:
struct Incoming {
    application_user_email: String
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
```
 - ## VERSION_1__APPLICATION_USER__AUTHORIZE_BY_FIRST_STEP POST
```
Authorizes application user for the firs step and send email to user.
```
```rust
Request data:
struct Incoming {
    application_user_device_id: String,
    application_user_email_or_application_user_nickname: String,
    application_user_password: String
}
```
```rust
Result data:
struct Outcoming {
    application_user_id: i64
}
```
```
Communication codes:
- APPLICATION_USER__WRONG_EMAIL_OR_NICKNAME_OR_PASSWORD
```
 - ## VERSION_1__APPLICATION_USER__AUTHORIZE_BY_LAST_STEP POST
```
Authorizes application user for the last step.
```
```rust
Request data:
struct Incoming {
    application_user_id: i64,
    application_user_device_id: String,
    application_user_authorization_token_value: String
}
```
```rust
Result data:
struct Outcoming {
    application_user_access_token_deserialized_form: String,
    application_user_access_refresh_token_deserialized_form: String
}
```
```
Communication codes:
- APPLICATION_USER_AUTHORIZATION_TOKEN__NOT_FOUND
- APPLICATION_USER_AUTHORIZATION_TOKEN__ALREADY_EXPIRED
- APPLICATION_USER_AUTHORIZATION_TOKEN__WRONG_VALUE
- APPLICATION_USER__NOT_FOUND
```
 - ## VERSION_1__APPLICATION_USER__SEND_EMAIL_FOR_AUTHORIZE POST
```
Sends email for authorization. (Should be used only if the user does not receive an email.)
```
```rust
Request data:
struct Incoming {
    application_user_device_id: String,
    application_user_id: i64
}
```
```
Result data: absent.
```
```
Communication codes:
- APPLICATION_USER__NOT_FOUND
- APPLICATION_USER_AUTHORIZATION_TOKEN__NOT_FOUND
- APPLICATION_USER_AUTHORIZATION_TOKEN__ALREADY_EXPIRED
```
 - ## VERSION_1__APPLICATION_USER__RESET_PASSWORD_BY_FIRST_STEP POST
```
Resets application user password for the first step and send email to user.
```
```rust
Request data:
struct Incoming {
    application_user_email: String
}
```
```rust
Result data:
struct Outcoming {
    application_user_id: i64
}
```
```
Communication codes:
- APPLICATION_USER__NOT_FOUND
```
 - ## VERSION_1__APPLICATION_USER__RESET_PASSWORD_BY_SECOND_STEP POST
```
Resets application user password for the second step through token value approving.
```
```rust
Request data:
struct Incoming {
    application_user_id: i64,
    application_user_reset_password_token_value: String
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
 - ## VERSION_1__APPLICATION_USER__RESET_PASSWORD_BY_LAST_STEP POST
```
Resets application user password for the last step.
```
```rust
Request data:
struct Incoming {
    application_user_id: i64,
    application_user_password: String,
    application_user_reset_password_token_value: String
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
 - ## VERSION_1__APPLICATION_USER__SEND_EMAIL_FOR_RESET_PASSWORD POST
```
Sends email for reset password.  (Should be used only if the user does not receive an email.)
```
```rust
Request data:
struct Incoming {
    application_user_id: i64
}
```
```
Result data: absent.
```
```
Communication codes:
- APPLICATION_USER__NOT_FOUND
- APPLICATION_USER_RESET_PASSWORD_TOKEN__NOT_FOUND
- APPLICATION_USER_RESET_PASSWORD_TOKEN__ALREADY_APPROVED
- APPLICATION_USER_RESET_PASSWORD_TOKEN__ALREADY_EXPIRED
```
 - ## VERSION_1__APPLICATION_USER__REFRESH_APPLICATION_USER_ACCESS_TOKEN POST
```
Refreshs application user access token.
```
```rust
Request data:
struct Incoming {
    application_user_access_token_deserialized_form: String,
    application_user_access_refresh_token_deserialized_form: String
}
```
```rust
Result data:
struct Outcoming {
    application_user_access_token_deserialized_form: String,
    application_user_access_refresh_token_deserialized_form: String
}
```
```
Communication codes:
- APPLICATION_USER_ACCESS_TOKEN__NOT_EXPIRED
- APPLICATION_USER_ACCESS_REFRESH_TOKEN__ALREADY_EXPIRED
- APPLICATION_USER_ACCESS_REFRESH_TOKEN__NOT_FOUND
```
# Incoming parameters validation rule.
 - ## application_user_registration_token_value
```
^[0-9]{6}$ - regular expression.
```
 - ## application_user_reset_password_token_value
```
^[0-9]{6}$ - regular expression.
```
 - ## application_user_authorization_token_value
```
^[0-9]{6}$ - regular expression.
```
 - ## application_user_id
```
>= 0
```
 - ## application_user_email
```
(?:[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*|"(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21\x23-\x5b\x5d-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])*")@(?:(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?|\[(?:(?:(2(5[0-5]|[0-4][0-9])|1[0-9][0-9]|[1-9]?[0-9]))\.){3}(?:(2(5[0-5]|[0-4][0-9])|1[0-9][0-9]|[1-9]?[0-9])|[a-z0-9-]*[a-z0-9]:(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21-\x5a\x53-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])+)\]) - regular expression.

320 - maximum number of characters.

```
 - ## application_user_nickname
```
55 - maximum number of characters.

@ - can not contain this character.

Can not contain whitespace character.

Can not be empty.

```
 - ## application_user_password
```
7 - minimum number of characters.

65 - maximum number of characters.

Can not contain whitespace character.
```
 - ## channel_id
```
>= 0
```
 - ## channel_name
```
75 - maximum number of characters.
```






<!--
## /v1/m/a/c/gmbca GET
Returns Channel registry by Channel Created_at.
### Request Quey parameters:
```
'cca': string; - 'channel_created_at', optional
```
```
'o': integer; - 'order'.
```
```
'l': integer; - 'limit'.

>0 && <=30
```
### Response parameters:
-PAYLOAD-:
```
'cr': array(object_1); - 'channel_registry', nullable.
```

object_1:
```
'ci': integer; - 'channel_id'.
```
```
'cn': string; - 'channel_name'.
```
```
'cpip': string; - 'channel_personalization_image_path'.
```
```
'csq': integer; - 'channel_subscribers_quantity'.
```
```
'cpmq': integer; - 'channel_public_marks_quantity'.
```
```
'chmq': integer; - 'channel_hidden_marks_quantity'.
```
```
'crq': integer; - 'channel_reactions_quantity'.
```
```
'cvq': integer; - 'channel_viewing_quantity'.
```
```
'cca': string; - 'channel_created_at'.
```
-ERROR_CODE-:
```
is absent.
``` -->
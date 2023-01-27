# Request standards
 - All data is transferred in `HTTP body` as `bytes` in encoded with `MessagePack protocol` form.
 - Every request should contain this `HTTP header`s:
 ```
 - user-agent: ... (https://www.scientiamobile.com/correctly-form-user-agents-for-mobile-apps/)
 - content-type: application/octet-stream
 - content-length: ...
 - x-content-type-options: nosniff
 ```
 - Values of variable for ``order``ing looks like:
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
- Existing values for `communication_code` can be founded here:
```
/project/application/source/presentation_layer/functionality/service/communication_code_registry.rs
```
- `HTTP status code` unequal to `200` (it is `400`, `401`, ... `500`) have not got `HTTP body`

<br/><br/>

# API for authorized application user.
```
 Every endpoint at this area requires an existing of access token.
 ```

 - ## /v1/m/au/dfod (deauthorize_from_one_device) POST                              БЫВШИЙ log_out_from_one_device. УДАЛИТЬ
```
Deauthorizes application user from one device.
```
```rust
Request data:
struct Incoming {
    aapplication_user_access_token_deserialized_form: String
}
```
```
Result data: absent.
```
```
Communication codes:
- APPLICATION_USER_ACCESS_TOKEN__ALREADY_EXPIRED
- APPLICATION_USER_ACCESS_TOKEN__IN_APPLICATION_USER_ACCESS_TOKEN_BLACK_LIST
- APPLICATION_USER_ACCESS_TOKEN__WRONG_DESERIALIZED_FORM
```
 - ## /v1/m/au/dfad (deauthorize_from_all_devices) POST                                 БЫВШИЙ log_out_from_all_device. УДАЛИТЬ
```
Deauthorizes application user from all devices.
```
```rust
Request data:
struct Incoming {
    aapplication_user_access_token_deserialized_form: String
}
```
```
Result data: absent.
```
```
Communication codes:
- APPLICATION_USER_ACCESS_TOKEN__ALREADY_EXPIRED
- APPLICATION_USER_ACCESS_TOKEN__IN_APPLICATION_USER_ACCESS_TOKEN_BLACK_LIST
- APPLICATION_USER_ACCESS_TOKEN__WRONG_DESERIALIZED_FORM
```
<br/><br/>

# API for not authorized application user.
 - ## /v1/m/au/cefe (check_email_for_existing) POST (GET functional)
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
Communication codes:
- APPLICATION_USER__INVALID_EMAIL
```
 - ## /v1/m/au/cnfe (check_nickname_for_existing) POST (GET functional)
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
Communication codes:
- APPLICATION_USER__INVALID_NICKNAME
```
 - ## /v1/m/au/rbfs (register_by_first_step) POST
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
- APPLICATION_USER__INVALID_EMAIL
- APPLICATION_USER__EMAIL_ALREADY_EXIST
```
- ## /v1/m/au/rbss (register_by_second_step) POST
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
- APPLICATION_USER__INVALID_EMAIL
- APPLICATION_USER_REGISTRATION_TOKEN__INVALID_VALUE
- APPLICATION_USER_REGISTRATION_TOKEN__NOT_FOUND
- APPLICATION_USER_REGISTRATION_TOKEN__ALREADY_APPROVED
- APPLICATION_USER_REGISTRATION_TOKEN__ALREADY_EXPIRED
- APPLICATION_USER_REGISTRATION_TOKEN__WRONG_VALUE
```

 - ## /v1/m/au/rbls (register_by_last_step) POST
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
    aapplication_user_access_token_deserialized_form: String,
    application_user_access_refresh_token_deserialized_form: String
}
```
```
Communication codes:
- APPLICATION_USER__INVALID_PASSWORD
- APPLICATION_USER__INVALID_NICKNAME
- APPLICATION_USER__INVALID_EMAIL
- APPLICATION_USER__NICKNAME_ALREADY_EXIST
- APPLICATION_USER__EMAIL_ALREADY_EXIST
- APPLICATION_USER_REGISTRATION_TOKEN__INVALID_VALUE
- APPLICATION_USER_REGISTRATION_TOKEN__NOT_FOUND
- APPLICATION_USER_REGISTRATION_TOKEN__IS_NOT_APPROVED
- APPLICATION_USER_REGISTRATION_TOKEN__WRONG_VALUE
- APPLICATION_USER_REGISTRATION_TOKEN__ALREADY_EXPIRED
```
 - ## /v1/m/au/sefr (send_email_for_register) POST
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
- APPLICATION_USER__INVALID_EMAIL
- APPLICATION_USER_REGISTRATION_TOKEN__NOT_FOUND
- APPLICATION_USER_REGISTRATION_TOKEN__ALREADY_APPROVED
- APPLICATION_USER_REGISTRATION_TOKEN__ALREADY_EXPIRED
```
 - ## /v1/m/au/abfs (authorize_by_first_step) POST                             БЫВШИЙ log_in_by_first_step. УДАЛИТЬ
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
 - ## /v1/m/au/abls (authorize_by_last_step) POST                       БЫВШИЙ (log_in_by_last_step). УДАЛИТЬ
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
    aapplication_user_access_token_deserialized_form: String,
    application_user_access_refresh_token_deserialized_form: String
}
```
```
Communication codes:
- APPLICATION_USER_AUTHORIZATION_TOKEN__INVALID_VALUE
- APPLICATION_USER_AUTHORIZATION_TOKEN__NOT_FOUND
- APPLICATION_USER_AUTHORIZATION_TOKEN__ALREADY_EXPIRED
- APPLICATION_USER_AUTHORIZATION_TOKEN__WRONG_VALUE
- APPLICATION_USER__NOT_FOUND
```
 - ## /v1/m/au/sefa (send_email_for_authorize) POST                                 БЫВШИЙ (send_email_for_log_in). УДАЛИТЬ
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
 - ## /v1/m/au/rpbfs (reset_password_by_first_step) POST
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
- APPLICATION_USER__INVALID_EMAIL
- APPLICATION_USER__NOT_FOUND
```
 - ## /v1/m/au/rpbss (reset_password_by_second_step) POST
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
- APPLICATION_USER_RESET_PASSWORD_TOKEN__INVALID_VALUE
- APPLICATION_USER_RESET_PASSWORD_TOKEN__NOT_FOUND
- APPLICATION_USER_RESET_PASSWORD_TOKEN__ALREADY_EXPIRED
- APPLICATION_USER_RESET_PASSWORD_TOKEN__ALREADY_APPROVED
- APPLICATION_USER_RESET_PASSWORD_TOKEN__WRONG_VALUE
```
 - ## /v1/m/au/rpbls (reset_password_by_last_step) POST
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
- APPLICATION_USER__INVALID_PASSWORD
- APPLICATION_USER__NOT_FOUND
- APPLICATION_USER_RESET_PASSWORD_TOKEN__INVALID_VALUE
- APPLICATION_USER_RESET_PASSWORD_TOKEN__NOT_FOUND
- APPLICATION_USER_RESET_PASSWORD_TOKEN__ALREADY_EXPIRED
- APPLICATION_USER_RESET_PASSWORD_TOKEN__IS_NOT_APPROVED
- APPLICATION_USER_RESET_PASSWORD_TOKEN__WRONG_VALUE
```
 - ## /v1/m/au/sefrp (send_email_for_reset_password) POST
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
 - ## /v1/m/au/rauat (refresh_application_user_access_token) POST
```
Refreshs application user access token.
```
```rust
Request data:
struct Incoming {
    aapplication_user_access_token_deserialized_form: String,
    application_user_access_refresh_token_deserialized_form: String
}
```
```rust
Result data:
struct Outcoming {
    aapplication_user_access_token_deserialized_form: String,
    application_user_access_refresh_token_deserialized_form: String
}
```
```
Communication codes:
- APPLICATION_USER_ACCESS_TOKEN__NOT_EXPIRED
- APPLICATION_USER_ACCESS_TOKEN__WRONG_DESERIALIZED_FORM
- APPLICATION_USER_ACCESS_REFRESH_TOKEN__ALREADY_EXPIRED
- APPLICATION_USER_ACCESS_REFRESH_TOKEN__NOT_FOUND
- APPLICATION_USER_ACCESS_REFRESH_TOKEN__WRONG_DESERIALIZED_FORM
```
# Incoming parameters validation rule.
 - ## application_user_registration_token_value
```
- "^[0-9]{6}$" - regular expression.
```
 - ## application_user_reset_password_token_value
```
- "^[0-9]{6}$" - regular expression.
```
 - ## application_user_authorization_token_value
```
- "^[0-9]{6}$" - regular expression.
```
 - ## application_user_email
```
- "\S+@\S+" - regular expression.
- 320 - maximum number of characters.

```
 - ## application_user_nickname
```
- 55 - maximum number of characters.
- '@' - can not contain this character.
- ' ' - can not contain this character (space character).
- can not be empty.

```
 - ## application_user_password
```
- 7 - minimum number of characters.
- 65 - maximum number of characters.
- ' ' - can not contain this character (space character).
```







<!-- ## /v1/m/a/c/gmbn GET
Returns Channel registry by Channel Name.
### Request Query parameters:
```
'cn': string; - 'channel_name'
```
```
'rcn': string; - 'requery_channel_name', optional.

An alternative for Offset. Used only for requering with persistent 'cn'. The value must be equal to the last Channel Name of Channel registry in
received early response.
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
```
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
# Request standards:
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

# Response standards:
 - All data is transferred in `HTTP body` as `bytes` in encoded with `MessagePack protocol` form.
 - Every response should contain this `HTTP header`s:
 ```
 - content-type: application/octet-stream
 - x-content-type-options: nosniff
 - content-length: ...
 ```
 - The permanent general structure of the each response with `HTTP status code` equal to `200` looks like:
```
struct UnifiedReport<S>
{
    data: Option<Data<S>>,
    communication_code: Option<String>
}

struct Data<S>
{
    data: Option<S>
}
```
- `Result data` structures written under each API endpoint will be nested in the `data` field in the `struct Data<S>`.
- Existing values for `communication_code` can be founded here:
```
/project/application/source/presentation_layer/functionality/service/communication_code_registry/mod.rs
```
- `HTTP status code` unequal to `200` (it is `400`, `401`, ... `500`) have not got `HTTP body`

<br/><br/>

# Area for authorized application user. API:
 - Every endpoint at this area requires an existing of `access token`
 - Response of EVERY endpoint at this area can contain `communication_code` equals to
 ```
  - ENTITY_APPLICATION_USER_ACCESS_TOKEN_ALREADY_EXPIRED
  - ENTITY_APPLICATION_USER_ACCESS_TOKEN_IN_APPLICATION_USER_ACCESS_TOKEN_BLACK_LIST
 ```
 - ## /v1/m/au/lofod (log_out_from_one_device) POST
```
Deauthorizes application user from one device.

Request data:
struct Base {
    application_user_access_token_web_form: String
}

Result data is absent.

Communication codes:
- ENTITY_APPLICATION_USER_ACCESS_REFRESH_TOKEN_NOT_FOUND

```
 - ## /v1/m/au/lofad (log_out_from_all_devices) POST
```
Deauthorizes application user from all devices.

Request data:
struct Base {
    application_user_access_token_web_form: String
}

Result data is absent.

Communication codes:
- ENTITY_APPLICATION_USER_ACCESS_REFRESH_TOKEN_NOT_FOUND
```
# Area for not authorized application user. API:
 - ## /v1/m/au/cefe (check_email_for_existing) POST (GET functional)
```
Checks application user email for existing.

Request data:
struct Base {
    application_user_email: String
}

Result data:
struct Base {
    result: bool
}

Communication codes:
- ENTITY_APPLICATION_USER_INVALID_EMAIL
```
 - ## /v1/m/au/cnfe (check_nickname_for_existing) POST (GET functional)
```
Checks application user nickname for existing.

Request data:
struct Base {
    application_user_nickname: String
}

Result data:
struct Base {
    result: bool
}

Communication codes:
- ENTITY_APPLICATION_USER_INVALID_NICKNAME
```
 - ## /v1/m/au/rbfs (register_by_first_step) POST
```
Registers application user for the first step and sends email to user.

Request data:
struct Base {
    application_user_email: String
}

Result data is absent.

Communication codes:
- ENTITY_APPLICATION_USER_INVALID_EMAIL
- ENTITY_APPLICATION_USER_EMAIL_ALREADY_EXIST
```
- ## /v1/m/au/rbss (register_by_second_step) POST
```
Registers application user for the second step through token value approving.

Request data:
struct Base {
    application_user_email: String,
    application_user_registration_confirmation_token_value: String
}

Result data:
struct Base {
    application_user_registration_confirmation_token_is_approved: bool
}

Communication codes:
- ENTITY_APPLICATION_USER_INVALID_EMAIL
- ENTITY_APPLICATION_USER_REGISTRATION_CONFIRMATION_TOKEN_INVALID_VALUE
- ENTITY_APPLICATION_USER_REGISTRATION_CONFIRMATION_TOKEN_NOT_FOUND
- ENTITY_APPLICATION_USER_REGISTRATION_CONFIRMATION_TOKEN_ALREADY_APPROVED
```

 - ## /v1/m/au/rbls (register_by_last_step) POST
```
Registers application user for the last step.

Request data:
struct Base {
    application_user_log_in_token_device_id: String,
    application_user_nickname: String,
    application_user_password: String,
    application_user_email: String,
    application_user_registration_confirmation_token_value: String
}

Result data:
struct Base {
    application_user_access_token_web_form: String,
    application_user_access_refresh_token_web_form: String
}

Communication codes:
- ENTITY_APPLICATION_USER_INVALID_PASSWORD
- ENTITY_APPLICATION_USER_INVALID_NICKNAME
- ENTITY_APPLICATION_USER_INVALID_EMAIL
- ENTITY_APPLICATION_USER_NICKNAME_ALREADY_EXIST
- ENTITY_APPLICATION_USER_EMAIL_ALREADY_EXIST
- ENTITY_APPLICATION_USER_REGISTRATION_CONFIRMATION_TOKEN_INVALID_VALUE
- ENTITY_APPLICATION_USER_REGISTRATION_CONFIRMATION_TOKEN_NOT_FOUND
- ENTITY_APPLICATION_USER_REGISTRATION_CONFIRMATION_TOKEN_IS_NOT_APPROVED
- ENTITY_APPLICATION_USER_REGISTRATION_CONFIRMATION_TOKEN_WRONG_VALUE
```
 - ## /v1/m/au/sefr (send_email_for_register) POST
```
Sends email for register. (Should be used only if the user does not receive an email.)

Request data:
struct Base {
    application_user_email: String
}

Result data is absent.

Communication codes:
- ENTITY_APPLICATION_USER_INVALID_EMAIL
- ENTITY_APPLICATION_USER_REGISTRATION_CONFIRMATION_TOKEN_NOT_FOUND
- ENTITY_APPLICATION_USER_REGISTRATION_CONFIRMATION_TOKEN_ALREADY_APPROVED
```
 - ## /v1/m/au/libfs (log_in_by_first_step) POST
```
Authorizes application user for the firs step and send email to user.

Request data:
struct Base {
    application_user_log_in_token_device_id: String,
    application_user_email_or_application_user_nickname: String,
    application_user_password: String
}

Result data:
struct Base {
    application_user_id: i64
}

Communication codes:
- ENTITY_APPLICATION_USER_WRONG_EMAIL_OR_NICKNAME_OR_PASSWORD
```
 - ## /v1/m/au/libls (log_in_by_last_step) POST
```
Authorizes application user for the last step.

Request data:
struct Base {
    application_user_id: i64,
    application_user_log_in_token_device_id: String,
    application_user_log_in_token_value: String
}

Result data:
struct Base {
    application_user_access_token_web_form: String,
    application_user_access_refresh_token_web_form: String
}

Communication codes:
- ENTITY_APPLICATION_USER_LOG_IN_TOKEN_INVALID_VALUE
- ENTITY_APPLICATION_USER_LOG_IN_TOKEN_NOT_FOUND
- ENTITY_APPLICATION_USER_LOG_IN_TOKEN_WRONG_VALUE
```
 - ## /v1/m/au/sefli (send_email_for_log_in) POST
```
Sends email for log in. (Should be used only if the user does not receive an email.)

Request data:
struct Base {
    application_user_log_in_token_device_id: String,
    application_user_id: i64
}

Result data is absent.

Communication codes:
- ENTITY_APPLICATION_USER_NOT_FOUND
- ENTITY_APPLICATION_USER_LOG_IN_TOKEN_NOT_FOUND
```
 - ## /v1/m/au/rpbfs (reset_password_by_first_step) POST
```
Resets application user password for the first step and send email to user.

Request data:
struct Base {
    application_user_email: String
}

Result data:
struct Base {
    application_user_id: i64
}

Communication codes:
- ENTITY_APPLICATION_USER_INVALID_EMAIL
- ENTITY_APPLICATION_USER_NOT_FOUND
```
 - ## /v1/m/au/rpbss (reset_password_by_second_step) POST
```
Resets application user password for the second step through token value approving.

Request data:
struct Base {
    application_user_id: i64,
    application_user_reset_password_token_value: String
}

Result data:
struct Base {
    application_user_reset_password_token_is_approved: bool
}

Communication codes:
- ENTITY_APPLICATION_USER_RESET_PASSWORD_TOKEN_INVALID_VALUE
- ENTITY_APPLICATION_USER_RESET_PASSWORD_TOKEN_NOT_FOUND
- ENTITY_APPLICATION_USER_RESET_PASSWORD_TOKEN_ALREADY_APPROVED
```
 - ## /v1/m/au/rpbls (reset_password_by_last_step) POST
```
Resets application user password for the last step.

Request data:
struct Base {
    application_user_id: i64,
    application_user_password: String,
    application_user_reset_password_token_value: String
}

Result data is absent.

Communication codes:
- ENTITY_APPLICATION_USER_INVALID_PASSWORD
- ENTITY_APPLICATION_USER_NOT_FOUND
- ENTITY_APPLICATION_USER_RESET_PASSWORD_TOKEN_INVALID_VALUE
- ENTITY_APPLICATION_USER_RESET_PASSWORD_TOKEN_NOT_FOUND
- ENTITY_APPLICATION_USER_RESET_PASSWORD_TOKEN_IS_NOT_APPROVED
- ENTITY_APPLICATION_USER_RESET_PASSWORD_TOKEN_WRONG_VALUE
```
 - ## /v1/m/au/sefrp (send_email_for_reset_password) POST
```
Sends email for reset password.  (Should be used only if the user does not receive an email.)

Request data:
struct Base {
    application_user_id: i64
}

Result data is absent.

Communication codes:
- ENTITY_APPLICATION_USER_NOT_FOUND
- ENTITY_APPLICATION_USER_RESET_PASSWORD_TOKEN_NOT_FOUND
- ENTITY_APPLICATION_USER_RESET_PASSWORD_TOKEN_ALREADY_APPROVED
```
 - ## /v1/m/au/rauat (refresh_application_user_access_token) POST
```
Refreshs application user access token.

Request data:
struct Base {
    application_user_access_token_web_form: String,
    application_user_access_refresh_token_web_form: String
}

Result data:
struct Base {
    application_user_access_token_web_form: String,
    application_user_access_refresh_token_web_form: String
}

Communication codes:
- ENTITY_APPLICATION_USER_ACCESS_TOKEN_NOT_EXPIRED
- ENTITY_APPLICATION_USER_ACCESS_REFRESH_TOKEN_NOT_FOUND
- ENTITY_APPLICATION_USER_ACCESS_REFRESH_TOKEN_ALREADY_EXPIRED
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
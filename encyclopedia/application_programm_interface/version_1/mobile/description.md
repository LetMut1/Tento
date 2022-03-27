# Request standards:
 - All data is transferred in `HTTP body` as `bytes` in encoded with `MessagePack protocol` form.
 - Values of variable for ``order``ing looks like:
```
0 - is equal to 'ASC'
1 - is equal to 'DESC'
```

# Response standards:
 - All data is transferred in `HTTP body` as `bytes` in encoded with `MessagePack protocol` form.
 - The permanent general structure of the each response with `HTTP status code` equal to `200` looks like:
```
enum EndpointResponse<S>
{
    Data {
        data: Data<S>
    },
    ErrorCode {
        error_code: String
    }
}

enum Data<S>
{
    Empty,
    Filled {
        data: S
    }
}
```
- `Result data` structures written under each API endpoint will be nested in the `data` field in the `enum Data<S>`.
- Existing values for `error_code` can be founded here:
```
/project/application/source/domain_layer/service/_in_context_for/domain_layer/error/_new_for_context/communication_code_storage/mod.rs
```
- `HTTP status code` unequal to `200` (it is `400`, `401`, ... `500`) have not got `HTTP body`

<br/><br/>

# Area for authorized application user. API:
 - Every endpoint at this area requires an existing of `json access webtoken`
 - Response of every endpoint at this area can contain `error_code` equals to `enjsacweto03`, `enjsacweto05`.
 - ## /v1/m/au/lo POST
```
Deauthorizes application user from one device.

Request data:
struct Base {
    json_access_web_token: String
}

Result data is absent.

Error codes:
- enjsreweto02

```
 - ## /v1/m/au/lofad POST
```
Deauthorizes application user from all devices.

Request data:
struct Base {
    json_access_web_token: String
}

Result data is absent.

Error codes:
- enjsreweto02
```
# Area for not authorized application user. API:
 - ## /v1/m/au/cnfe GET
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

Error codes:
- enapus06
```
 - ## /v1/m/au/cefe GET
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

Error codes:
- enapus05
```
 - ## /v1/m/au/rbfs POST
```
Registers application user for the first step and sends email to user.

Request data:
struct Base {
    application_user_email: String
}

Result data is absent.

Error codes:
- enapus01
- enapus05
```
 - ## /v1/m/au/rbls POST
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
    json_access_web_token: String,
    json_refresh_web_token: String
}

Error codes:
- enapus01
- enapus02
- enapus06
- enapus07
- enapusrecoto02
- enapusrecoto03
```
 - ## /v1/m/au/sefr POST
```
Sends email for register. (Should be used only if the user does not receive an email.)

Request data:
struct Base {
    application_user_email: String
}

Result data is absent.

Error codes:
- enapusrecoto02
```
 - ## /v1/m/au/libfs POST
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

Error codes:
- enapus04
```
 - ## /v1/m/au/libls POST
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
    json_access_web_token: String,
    json_refresh_web_token: String
}

Error codes:
- enapuslointo02
- enapuslointo03
```
 - ## /v1/m/au/sefli POST
```
Sends email for log in. (Should be used only if the user does not receive an email.)

Request data:
struct Base {
    application_user_log_in_token_device_id: String,
    application_user_id: i64
}

Result data is absent.

Error codes:
- enapus03
- enapuslointo02
```
 - ## /v1/m/au/rpbfs POST
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

Error codes:
- enapus03
```
 - ## /v1/m/au/rpbls POST
```
Resets application user password for the last step.

Request data:
struct Base {
    application_user_id: i64,
    application_user_password: String,
    application_user_reset_password_token_value: String
}

Result data is absent.

Error codes:
- enapusrepato02
- enapusrepato03
- enapus07
```
 - ## /v1/m/au/sefrp POST
```
Sends email for reset password.  (Should be used only if the user does not receive an email.)

Request data:
struct Base {
    application_user_id: i64
}

Result data is absent.

Error codes:
- enapus03
- enapusrepato02
```
 - ## /v1/m/au/rjawt POST
```
Refreshs json access web token.

Request data:
struct Base {
    json_access_web_token: String,
    json_refresh_web_token: String
}

Result data:
struct Base {
    json_access_web_token: String,
    json_refresh_web_token: String
}

Error codes:
- enjsacweto04
- enjsreweto02
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
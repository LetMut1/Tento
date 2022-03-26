# Request standards:
 - All payload (parameters) is transferred in `HTTP body` in encoded with `MessagePack protocol` form.
 - Values of variable for ``order``ing looks like:
```
0 - is equal to 'ASC'
1 - is equal to 'DESC'
```
# Response standards:
 - All payload (parameters) is transferred in `HTTP body` in encoded with `MessagePack protocol` form.
 - The permanent general structure of the each response with `HTTP status code` equal to `200` looks like:
```
struct WrappedResponseData<S> 
{
    success: bool,
    error_code: Option<&'static str>,
    data: Option<S>
}
```
- Structures written under each specific API point will be nested in the `data` field in the `WrappedResponseData`
- `HTTP status code` unequal to `200` (it is `400`, `401`, ... `500`) have not got `HTTP body`













Existing values for -ERROR_CODE- can be founded here:
```
application/source/source/domain_layer/service/_in_context_for/domain_layer/error/_new_for_context/communication_code_storage/mod.rs
```
# ApplicationUser authorized area:
Every endpoint at this area requires an existing of JsonAccessWebToken, wich should be sended as ``'X-Jawt'`` parameter
of HTTP/S Request Header. Response of every endpoint can contain -ERROR_CODE- equals ``'enjsacweto03'``
## /v1/m/a/c/gmbn GET
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
```
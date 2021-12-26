# Application programming interface description standard:
Existing parameter types:
```
string  
integer  
array()  
object_NUMBER
```
Existing parameter features:  
```
optional (exists/not exists)  
nullable (: null / : value)
```
Parameter description:
```
'name': type; - 'decryption/full name', features.

additional informaion.

description.

usecases.
```
# Request standard:
Values of variable for ``order``ing:
```
0 is equal to ASC
1 is equal to DESC
```
# Response standard:
```
200, '{"s":true}'
200, '{"s":true, "b": {-PAYLOAD-}}'
200, '{"s":false, "c":"-ERROR_CODE-"}'
```
```
400
401
```
```
500
```
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

Base64(URL_SAFE) encoded;
```
```
'rcn': string; - 'requery_channel_name', optional.

Base64(URL_SAFE) encoded;


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

Base64(URL_SAFE) encoded;
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
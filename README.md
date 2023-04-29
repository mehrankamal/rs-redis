# rs-redis

> A dummy excercise implementation of Redis (not to the teeth) in Rust following the [Build your Own Redis in C/C++](https://build-your-own.org/redis/) with some modifications

## Protocol:

### Message Structure:

#### Format: 

##### Requests:
```
<NO_OF_BYTES_LE_ENCODED_U64> <CR><LF>
<MESSAGE_TYPE> <CR><LF>
<CR><LF>
<MESSAGE_BODY>
```

##### Responses:
```
<NO_OF_BYTES_LE_ENCODED_U64> <CR><LF>
<RESPONSE_TYPE> <CR><LF>
<CR><LF>
<RESPONSE_BODY>
```
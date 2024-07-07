//*// Creates a new message. New messages are open in write-only mode for eventual sending.
//*// Getters of the message should not be called. Only fjage_msg_set_* and fjage_msg_add_*
//*// functions should be called on the message. If the message is eventually not
//*// sent, it may be destroyed using fjage_msg_destroy().
//*//
//*// Messages of class org.arl.fjage.GenericMessage are currently unsupported.
//*//
//*// @param clazz          Fully qualified message class
//*// @param perf           Performative of the message
//*// @return               Message open in write-only mode

use std::{
    cmp::min,
    ffi::{c_char, c_double, c_float, c_int, c_long},
    slice,
};

use serde_json::Value;

use crate::msg_get_array;

use super::util::{c_api_cstr_to_string, c_api_int_to_perf, c_api_perf_to_int, fjage_msg_t};

//fjage_msg_t fjage_msg_create(const char *clazz, fjage_perf_t perf);
#[no_mangle]
pub unsafe extern "C" fn fjage_msg_create(clazz: *mut c_char, perf: c_int) -> *mut fjage_msg_t {
    let msg = fjage_msg_t::alloc();
    let msg_ref = &mut msg.as_mut().unwrap().msg;
    msg_ref.clazz = c_api_cstr_to_string(clazz);
    msg_ref.data.perf = c_api_int_to_perf(perf);
    return msg;
}

//*// Destroy a message. Once destroyed, the message is considered invalid and should
//*// no longer be used.
//*//
//*// @param msg            Message to destroy

//void fjage_msg_destroy(fjage_msg_t msg);
#[no_mangle]
pub unsafe extern "C" fn fjage_msg_destroy(msg: *mut fjage_msg_t) {
    fjage_msg_t::free(msg);
}

//*// Set the recipient of a message.
//*//
//*// @param msg            Message in write-only mode
//*// @param aid            AgentID of the recipient

//void fjage_msg_set_recipient(fjage_msg_t msg, fjage_aid_t aid);
#[no_mangle]
pub unsafe extern "C" fn fjage_msg_set_recipient(msg: *mut fjage_msg_t, aid: *const c_char) {
    fjage_msg_t::strkey_set(msg, "recipient", Value::String(c_api_cstr_to_string(aid)));
}

//*// Set the message ID of the request which is being responded to.
//*//
//*// @param msg            Message in write-only mode
//*// @param id             Message ID of the request being responded to

//void fjage_msg_set_in_reply_to(fjage_msg_t msg, const char *id);
#[no_mangle]
pub unsafe extern "C" fn fjage_msg_set_in_reply_to(msg: *mut fjage_msg_t, id: *const c_char) {
    msg.as_mut().unwrap().msg.data.inReplyTo = Some(c_api_cstr_to_string(id));
}

//*// Add a string value to a message.
//*//
//*// @param msg            Message in write-only mode
//*// @param key            Key
//*// @param value          Value

//void fjage_msg_add_string(fjage_msg_t msg, const char *key, const char *value);
#[no_mangle]
pub unsafe extern "C" fn fjage_msg_add_string(
    msg: *mut fjage_msg_t,
    key: *const c_char,
    value: *const c_char,
) {
    fjage_msg_t::set(msg, key, Value::String(c_api_cstr_to_string(value)));
}

//*// Add an integer value to a message.
//*//
//*// @param msg            Message in write-only mode
//*// @param key            Key
//*// @param value          Value

//void fjage_msg_add_int(fjage_msg_t msg, const char *key, int value);
#[no_mangle]
pub unsafe extern "C" fn fjage_msg_add_int(
    msg: *mut fjage_msg_t,
    key: *const c_char,
    value: c_int,
) {
    fjage_msg_t::set(msg, key, Value::from(value));
}

//*// Add a long value to a message.
//*//
//*// @param msg            Message in write-only mode
//*// @param key            Key
//*// @param value          Value

//void fjage_msg_add_long(fjage_msg_t msg, const char *key, long value);
#[no_mangle]
pub unsafe extern "C" fn fjage_msg_add_long(
    msg: *mut fjage_msg_t,
    key: *const c_char,
    value: c_long,
) {
    fjage_msg_t::set(msg, key, Value::from(value));
}

//*// Add a floating point value to a message.
//*//
//*// @param msg            Message in write-only mode
//*// @param key            Key
//*// @param value          Value

//void fjage_msg_add_float(fjage_msg_t msg, const char *key, float value);
#[no_mangle]
pub unsafe extern "C" fn fjage_msg_add_float(
    msg: *mut fjage_msg_t,
    key: *const c_char,
    value: c_float,
) {
    fjage_msg_t::set(msg, key, Value::from(value));
}

/* NOT IN THE BASE C API */
#[no_mangle]
pub unsafe extern "C" fn fjage_msg_add_double(
    msg: *mut fjage_msg_t,
    key: *const c_char,
    value: c_double,
) {
    fjage_msg_t::set(msg, key, Value::from(value));
}

//*// Add a boolean value to a message.
//*//
//*// @param msg            Message in write-only mode
//*// @param key            Key
//*// @param value          Value

//void fjage_msg_add_bool(fjage_msg_t msg, const char *key, bool value);
#[no_mangle]
pub unsafe extern "C" fn fjage_msg_add_bool(
    msg: *mut fjage_msg_t,
    key: *const c_char,
    value: bool,
) {
    fjage_msg_t::set(msg, key, Value::Bool(value));
}

//*// Add a byte array value to a message.
//*//
//*// @param msg            Message in write-only mode
//*// @param key            Key
//*// @param value          Pointer to the byte array
//*// @param len            Length of the byte array

//void fjage_msg_add_byte_array(fjage_msg_t msg, const char *key, uint8_t *value, int len);
#[no_mangle]
pub unsafe extern "C" fn fjage_msg_add_byte_array(
    msg: *mut fjage_msg_t,
    key: *const c_char,
    value: *mut u8,
    len: c_int,
) {
    let arr = slice::from_raw_parts(value, len as usize);
    //let arr: Vec<u8> = Vec::from_raw_parts(value, len as usize, len as usize);
    fjage_msg_t::set(msg, key, Value::from(arr));
}

//*// Add an integer array value to a message.
//*//
//*// @param msg            Message in write-only mode
//*// @param key            Key
//*// @param value          Pointer to the int array
//*// @param len            Length of the int array (number of ints)

//void fjage_msg_add_int_array(fjage_msg_t msg, const char *key, int32_t *value, int len);
#[no_mangle]
pub unsafe extern "C" fn fjage_msg_add_int_array(
    msg: *mut fjage_msg_t,
    key: *const c_char,
    value: *mut i32,
    len: c_int,
) {
    let arr = slice::from_raw_parts(value, len as usize);
    fjage_msg_t::set(msg, key, Value::from(arr));
}

/* NOT IN BASE C API */
#[no_mangle]
pub unsafe extern "C" fn fjage_msg_add_long_array(
    msg: *mut fjage_msg_t,
    key: *const c_char,
    value: *const i64,
    len: c_int,
) {
    let arr = slice::from_raw_parts(value, len as usize);
    fjage_msg_t::set(msg, key, Value::from(arr));
}

//*// Add a floating point array value to a message.
//*//
//*// @param msg            Message in write-only mode
//*// @param key            Key
//*// @param value          Pointer to the floating point array
//*// @param len            Length of the array (in floats)

//void fjage_msg_add_float_array(fjage_msg_t msg, const char *key, float *value, int len);
#[no_mangle]
pub unsafe extern "C" fn fjage_msg_add_float_array(
    msg: *mut fjage_msg_t,
    key: *const c_char,
    value: *mut c_float,
    len: c_int,
) {
    let arr = slice::from_raw_parts(value, len as usize);
    fjage_msg_t::set(msg, key, Value::from(arr));
}

/* NOT IN BASE C API */

#[no_mangle]
pub unsafe extern "C" fn fjage_msg_add_double_array(
    msg: *mut fjage_msg_t,
    key: *const c_char,
    value: *const c_double,
    len: c_int,
) {
    let arr = slice::from_raw_parts(value, len as usize);
    fjage_msg_t::set(msg, key, Value::from(arr));
}

//*// Get the message ID. The string returned by this function should
//*// not be freed by the caller. However, it will be invalid after the message
//*// is destroyed.
//*//
//*// @param msg            Message in read-only mode
//*// @return               Message ID

//const char *fjage_msg_get_id(fjage_msg_t msg);
#[no_mangle]
pub unsafe extern "C" fn fjage_msg_get_id(msg: *mut fjage_msg_t) -> *const c_char {
    //let id = &msg.as_ref().unwrap().msg.data.msgID;
    let id = fjage_msg_t::strkey_get(msg, "msgID")
        .as_str()
        .unwrap()
        .to_string();
    let id = msg.as_mut().unwrap().alloc_str(id);
    return id;
}

//*// Get the message class. The string returned by this function should
//*// not be freed by the caller. However, it will be invalid after the message
//*// is destroyed.
//*//
//*// @param msg            Message in read-only mode
//*// @return               Fully qualified message class name

//const char *fjage_msg_get_clazz(fjage_msg_t msg);
#[no_mangle]
pub unsafe extern "C" fn fjage_msg_get_clazz(msg: *mut fjage_msg_t) -> *const c_char {
    let clazz = &msg.as_ref().unwrap().msg.clazz;
    let clazz = msg.as_mut().unwrap().alloc_str(clazz.clone());
    return clazz;
}

//*// Get the message performative.
//*//
//*// @param msg            Message in read-only mode
//*// @return               Message performative

//fjage_perf_t fjage_msg_get_performative(fjage_msg_t msg);
#[no_mangle]
pub unsafe extern "C" fn fjage_msg_get_performative(msg: *mut fjage_msg_t) -> c_int {
    return c_api_perf_to_int(&msg.as_ref().unwrap().msg.data.perf);
}

//*// Get the message recipient. The AgentID returned by this function should
//*// not be freed by the caller. However, it will be invalid after the message
//*// is destroyed.
//*//
//*// @param msg            Message in read-only mode
//*// @return               AgentID of the recipient

//fjage_aid_t fjage_msg_get_recipient(fjage_msg_t msg);
#[no_mangle]
pub unsafe extern "C" fn fjage_msg_get_recipient(msg: *mut fjage_msg_t) -> *const c_char {
    let recipient = &msg.as_ref().unwrap().msg.data.recipient;
    if recipient.is_empty() {
        return std::ptr::null();
    }
    let recipient = msg.as_mut().unwrap().alloc_str(recipient.clone());
    return recipient;
}

//*// Get the message sender. The AgentID returned by this function should
//*// not be freed by the caller. However, it will be invalid after the message
//*// is destroyed.
//*//
//*// @param msg            Message in read-only mode
//*// @return               AgentID of the sender

//fjage_aid_t fjage_msg_get_sender(fjage_msg_t msg);
#[no_mangle]
pub unsafe extern "C" fn fjage_msg_get_sender(msg: *mut fjage_msg_t) -> *const c_char {
    let sender = &msg.as_ref().unwrap().msg.data.sender;
    let sender = msg.as_mut().unwrap().alloc_str(sender.clone());
    return sender;
}

//*// Get the message ID of the request corresponding to this response.
//*// The string returned by this function should not be freed by the caller.
//*// However, it will be invalid after the message is destroyed.
//*//
//*// @param msg            Message in read-only mode
//*// @return               Message ID of the corresponding request

//const char *fjage_msg_get_in_reply_to(fjage_msg_t msg);
#[no_mangle]
pub unsafe extern "C" fn fjage_msg_get_in_reply_to(msg: *mut fjage_msg_t) -> *const c_char {
    let irp = &msg.as_ref().unwrap().msg.data.inReplyTo;
    if irp.is_none() {
        return std::ptr::null();
    }
    let irp = irp.clone().unwrap();
    let irp = msg.as_mut().unwrap().alloc_str(irp);
    return irp;
}

//*// Get a string value. The string returned by this function should not
//*// be freed by the caller. However, it will be invalid after the message
//*// is destroyed.
//*//
//*// @param msg            Message in read-only mode
//*// @param key            Key
//*// @return               String value

// NOTE: We need to decide if we care about this only getting generic fields
//const char *fjage_msg_get_string(fjage_msg_t msg, const char *key);
#[no_mangle]
pub unsafe extern "C" fn fjage_msg_get_string(
    msg: *mut fjage_msg_t,
    key: *const c_char,
) -> *const c_char {
    let x = fjage_msg_t::get(msg, key);
    if x.is_null() {
        return std::ptr::null();
    }
    return fjage_msg_t::alloc_str_s(msg, x.as_str().unwrap_or("").to_string());
}

//*// Get an integer value.
//*//
//*// @param msg            Message in read-only mode
//*// @param key            Key
//*// @param defval         Default value, if value unavailable
//*// @return               Integer value

//int fjage_msg_get_int(fjage_msg_t msg, const char *key, int defval);
#[no_mangle]
pub unsafe extern "C" fn fjage_msg_get_int(
    msg: *mut fjage_msg_t,
    key: *const c_char,
    defval: c_int,
) -> c_int {
    return fjage_msg_t::get(msg, key).as_i64().unwrap_or(defval as i64) as c_int;
}

//*// Get a long value.
//*//
//*// @param msg            Message in read-only mode
//*// @param key            Key
//*// @param defval         Default value, if value unavailable
//*// @return               Long value

//long fjage_msg_get_long(fjage_msg_t msg, const char *key, long defval);
#[no_mangle]
pub unsafe extern "C" fn fjage_msg_get_long(
    msg: *mut fjage_msg_t,
    key: *const c_char,
    defval: c_long,
) -> c_long {
    return fjage_msg_t::get(msg, key).as_i64().unwrap_or(defval as i64) as c_long;
}

//*// Get a floating point value.
//*//
//*// @param msg            Message in read-only mode
//*// @param key            Key
//*// @param defval         Default value, if value unavailable
//*// @return               Floating point value

//float fjage_msg_get_float(fjage_msg_t msg, const char *key, float defval);
#[no_mangle]
pub unsafe extern "C" fn fjage_msg_get_float(
    msg: *mut fjage_msg_t,
    key: *const c_char,
    defval: c_float,
) -> c_float {
    return fjage_msg_t::get(msg, key).as_f64().unwrap_or(defval as f64) as c_float;
}

/* NOT IN BASE C API */
#[no_mangle]
pub unsafe extern "C" fn fjage_msg_get_double(
    msg: *mut fjage_msg_t,
    key: *const c_char,
    defval: c_double,
) -> c_double {
    return fjage_msg_t::get(msg, key).as_f64().unwrap_or(defval as f64) as c_double;
}

//*// Get a boolean value.
//*//
//*// @param msg            Message in read-only mode
//*// @param key            Key
//*// @param defval         Default value, if value unavailable
//*// @return               Boolean value

//bool fjage_msg_get_bool(fjage_msg_t msg, const char *key, bool defval);
#[no_mangle]
pub unsafe extern "C" fn fjage_msg_get_bool(
    msg: *mut fjage_msg_t,
    key: *const c_char,
    defval: bool,
) -> bool {
    return fjage_msg_t::get(msg, key).as_bool().unwrap_or(defval);
}

//*// Get a byte array value. If only the length of the array is desired (so that
//*// an array can be allocated), passing NULL as value and 0 as maxlen returns
//*// the array length.
//*//
//*// @param msg            Message in read-only mode
//*// @param key            Key
//*// @param value          Pointer to a byte array to receive data, or NULL
//*// @param maxlen         The maximum number of bytes to receive, or 0 if value is NULL
//*// @return               Number of bytes in the byte array

//int fjage_msg_get_byte_array(fjage_msg_t msg, const char *key, uint8_t *value, int maxlen);
#[no_mangle]
pub unsafe extern "C" fn fjage_msg_get_byte_array(
    msg: *mut fjage_msg_t,
    key: *const c_char,
    value: *mut u8,
    maxlen: c_int,
) -> c_int {
    return msg_get_array!(msg, key, value, maxlen, u8, |x| x.as_u64().unwrap_or(0)
        as u8);
}

//*// Get an integer array value. If only the length of the array is desired (so that
//*// an array can be allocated), passing NULL as value and 0 as maxlen returns
//*// the array length.
//*//
//*// @param msg            Message in read-only mode
//*// @param key            Key
//*// @param value          Pointer to an int array to receive data, or NULL
//*// @param maxlen         The maximum number of ints to receive, or 0 if value is NULL
//*// @return               Number of ints in the byte array

//int fjage_msg_get_int_array(fjage_msg_t msg, const char *key, int32_t *value, int maxlen);
#[no_mangle]
pub unsafe extern "C" fn fjage_msg_get_int_array(
    msg: *mut fjage_msg_t,
    key: *const c_char,
    value: *mut i32,
    maxlen: c_int,
) -> c_int {
    return msg_get_array!(msg, key, value, maxlen, i32, |x| x.as_i64().unwrap_or(0)
        as i32);
}

/* NOT IN BASE C API */
#[no_mangle]
pub unsafe extern "C" fn fjage_msg_get_long_array(
    msg: *mut fjage_msg_t,
    key: *const c_char,
    value: *mut i64,
    maxlen: c_int,
) -> c_int {
    return msg_get_array!(msg, key, value, maxlen, i64, |x| x.as_i64().unwrap_or(0)
        as i64);
}

//*// Get a floating point array value. If only the length of the array is desired (so that
//*// an array can be allocated), passing NULL as value and 0 as maxlen returns
//*// the array length.
//*//
//*// @param msg            Message in read-only mode
//*// @param key            Key
//*// @param value          Pointer to a floating point array to receive data, or NULL
//*// @param maxlen         The maximum number of floats to receive, or 0 if value is NULL
//*// @return               Number of floats in the array

//int fjage_msg_get_float_array(fjage_msg_t msg, const char *key, float *value, int maxlen);
#[no_mangle]
pub unsafe extern "C" fn fjage_msg_get_float_array(
    msg: *mut fjage_msg_t,
    key: *const c_char,
    value: *mut f32,
    maxlen: c_int,
) -> c_int {
    return msg_get_array!(msg, key, value, maxlen, f32, |x| x.as_f64().unwrap_or(0.0)
        as f32);
}

/* NOT IN BASE C API */
#[no_mangle]
pub unsafe extern "C" fn fjage_msg_get_double_array(
    msg: *mut fjage_msg_t,
    key: *const c_char,
    value: *mut f64,
    maxlen: c_int,
) -> c_int {
    return msg_get_array!(msg, key, value, maxlen, f64, |x| x.as_f64().unwrap_or(0.0)
        as f64);
}

/// Get an integer parameter from an agent. This is a utility function that sends a ParameterReq to an
/// agent, and returns the value from the agent's response.
///
/// @param aid            AgentID of the target agent
/// @param param          Name of the parameter
/// @param ndx            Index of the parameter (-1 for non-indexed parameters)
/// @param defval         Default value, if value unavailable
/// @return               Parameter value, NULL
use std::{
    cmp::min,
    ffi::{c_char, c_double, c_float, c_int, c_long},
    slice,
};

use serde_json::Value;

use crate::{core::param::ParameterManipulation, remote::gateway::Gateway};

use super::util::{c_api_alloc_cstr, c_api_cstr_to_string, c_api_exec_timeout_ms, c_api_set_param};

//int fjage_param_get_int(fjage_gw_t gw, fjage_aid_t aid, const char *param, int ndx, int defval);
#[no_mangle]
pub unsafe extern "C" fn fjage_param_get_int(
    gw: *mut Gateway,
    aid: *const c_char,
    param: *const c_char,
    ndx: c_int,
    defval: c_int,
) -> c_int {
    //let val = c_api_get_param(gw, aid, param, ndx);
    /*let val = gw
    .as_mut()
    .unwrap()
    .get_param_blocking(&cstr_to_String(aid), &cstr_to_String(param));
    if val.is_null() {
        return defval;
    } else {
        return val.as_i64().unwrap() as c_int;
    }*/
    let val = c_api_exec_timeout_ms(
        gw.as_mut().unwrap().get_int(
            &c_api_cstr_to_string(aid),
            &c_api_cstr_to_string(param),
            ndx as i64,
        ),
        1000,
    );
    if val.is_ok() {
        return val.unwrap().unwrap_or(defval);
    } else {
        return defval;
    }
}

/// Get a long parameter from an agent. This is a utility function that sends a ParameterReq to an
/// agent, and returns the value from the agent's response.
///
/// @param gw             Gateway
/// @param aid            AgentID of the target agent
/// @param param          Name of the parameter
/// @param ndx            Index of the parameter (-1 for non-indexed parameters)
/// @param defval         Default value, if value unavailable
/// @return               Parameter value

//long fjage_param_get_long(fjage_gw_t gw, fjage_aid_t aid, const char *param, int ndx, long defval);
#[no_mangle]
pub unsafe extern "C" fn fjage_param_get_long(
    gw: *mut Gateway,
    aid: *const c_char,
    param: *const c_char,
    ndx: c_int,
    defval: c_long,
) -> c_long {
    let val = c_api_exec_timeout_ms(
        gw.as_mut().unwrap().get_long(
            &c_api_cstr_to_string(aid),
            &c_api_cstr_to_string(param),
            ndx as i64,
        ),
        1000,
    );
    if val.is_ok() {
        return val.unwrap().unwrap_or(defval);
    } else {
        return defval;
    }
}

/// Get a float parameter from an agent. This is a utility function that sends a ParameterReq to an
/// agent, and returns the value from the agent's response.
///
/// @param gw             Gateway
/// @param aid            AgentID of the target agent
/// @param param          Name of the parameter
/// @param ndx            Index of the parameter (-1 for non-indexed parameters)
/// @param defval         Default value, if value unavailable
/// @return               Parameter value

//float fjage_param_get_float(fjage_gw_t gw, fjage_aid_t aid, const char *param, int ndx, float defval);
#[no_mangle]
pub unsafe extern "C" fn fjage_param_get_float(
    gw: *mut Gateway,
    aid: *const c_char,
    param: *const c_char,
    ndx: c_int,
    defval: c_float,
) -> c_float {
    let val = c_api_exec_timeout_ms(
        gw.as_mut().unwrap().get_float(
            &c_api_cstr_to_string(aid),
            &c_api_cstr_to_string(param),
            ndx as i64,
        ),
        1000,
    );
    if val.is_ok() {
        return val.unwrap().unwrap_or(defval);
    } else {
        return defval;
    }
}

//double fjage_param_get_double(fjage_gw_t gw, fjage_aid_t aid, const char *param, int ndx, double defval);
#[no_mangle]
pub unsafe extern "C" fn fjage_param_get_double(
    gw: *mut Gateway,
    aid: *const c_char,
    param: *const c_char,
    ndx: c_int,
    defval: c_double,
) -> c_double {
    let val = c_api_exec_timeout_ms(
        gw.as_mut().unwrap().get_double(
            &c_api_cstr_to_string(aid),
            &c_api_cstr_to_string(param),
            ndx as i64,
        ),
        1000,
    );
    if val.is_ok() {
        return val.unwrap().unwrap_or(defval);
    } else {
        return defval;
    }
}

/// Get a boolean parameter from an agent. This is a utility function that sends a ParameterReq to an
/// agent, and returns the value from the agent's response.
///
/// @param gw             Gateway
/// @param aid            AgentID of the target agent
/// @param param          Name of the parameter
/// @param ndx            Index of the parameter (-1 for non-indexed parameters)
/// @param defval         Default value, if value unavailable
/// @return               Parameter value

//bool fjage_param_get_bool(fjage_gw_t gw, fjage_aid_t aid, const char *param, int ndx, bool defval);
#[no_mangle]
pub unsafe extern "C" fn fjage_param_get_bool(
    gw: *mut Gateway,
    aid: *const c_char,
    param: *const c_char,
    ndx: c_int,
    defval: c_int,
) -> bool {
    // Rust does not let us treat integers as booleans
    let defval = defval == 1;
    let val = c_api_exec_timeout_ms(
        gw.as_mut().unwrap().get_bool(
            &c_api_cstr_to_string(aid),
            &c_api_cstr_to_string(param),
            ndx as i64,
        ),
        1000,
    );
    if val.is_ok() {
        return val.unwrap().unwrap_or(defval);
    } else {
        return defval;
    }
}

/// Get a string parameter from an agent. This is a utility function that sends a ParameterReq to an
/// agent, and returns the value from the agent's response. The returned pointer should be freed by
/// the caller after use.
///
/// @param gw             Gateway
/// @param aid            AgentID of the target agent
/// @param param          Name of the parameter
/// @param ndx            Index of the parameter (-1 for non-indexed parameters)
/// @param strval         Pointer to a string to receive data, or NULL
/// @param len            Size of the buffer, or 0 if strval is NULL
/// @return               Length of the string copied into the buffer, or length of the string returned by the agent if strval is NULL, or -1 on error.

//int fjage_param_get_string(fjage_gw_t gw, fjage_aid_t aid, const char *param, int ndx, const char *strval, int len);
#[no_mangle]
pub unsafe extern "C" fn fjage_param_get_string(
    gw: *mut Gateway,
    aid: *const c_char,
    param: *const c_char,
    ndx: c_int,
    strval: *mut c_char,
    len: c_int,
) -> c_int {
    let val = c_api_exec_timeout_ms(
        gw.as_mut().unwrap().get_string(
            &c_api_cstr_to_string(aid),
            &c_api_cstr_to_string(param),
            ndx as i64,
        ),
        1000,
    );
    if val.is_err() {
        return -1;
    }
    let val = val.unwrap();
    if val.is_none() {
        return -1;
    }
    let val = val.unwrap();
    let str_len = val.len();
    if !strval.is_null() {
        let copy_len = std::cmp::min(len.try_into().unwrap(), str_len);
        val.as_str().as_ptr().copy_to(strval.cast(), copy_len);
        return copy_len as c_int;
    }
    return str_len as c_int;
}

// not in base C API
// returns number of elements, or -1 on error

//int fjage_param_get_int_array(fjage_gw_t gw, fjage_aid_t aid, const char *param, int *value, int maxlen, int ndx);
#[no_mangle]
pub unsafe extern "C" fn fjage_param_get_int_array(
    gw: *mut Gateway,
    aid: *const c_char,
    param: *const c_char,
    value: *mut c_int,
    maxlen: c_int,
    ndx: c_int,
) -> c_int {
    let val = c_api_exec_timeout_ms(
        gw.as_mut().unwrap().get_int_array(
            &c_api_cstr_to_string(aid),
            &c_api_cstr_to_string(param),
            ndx as i64,
        ),
        1000,
    );
    if val.is_err() {
        return -1;
    }
    let val = val.unwrap();
    if val.is_none() {
        return -1;
    }
    let val: Vec<c_int> = val.unwrap().iter().map(|x| *x as c_int).collect();
    // Bounds checking copy from constructed vector
    let copy_len = std::cmp::min(val.len(), maxlen as usize);
    value.copy_from(val.as_ptr(), copy_len);
    return copy_len as c_int;
}

//int fjage_param_get_long_array(fjage_gw_t gw, fjage_aid_t aid, const char *param, long *value, int maxlen, int ndx);
#[no_mangle]
pub unsafe extern "C" fn fjage_param_get_long_array(
    gw: *mut Gateway,
    aid: *const c_char,
    param: *const c_char,
    value: *mut c_long,
    maxlen: c_int,
    ndx: c_int,
) -> c_int {
    let val = c_api_exec_timeout_ms(
        gw.as_mut().unwrap().get_long_array(
            &c_api_cstr_to_string(aid),
            &c_api_cstr_to_string(param),
            ndx as i64,
        ),
        1000,
    );
    if val.is_err() {
        return -1;
    }
    let val = val.unwrap();
    if val.is_none() {
        return -1;
    }
    let val: Vec<c_long> = val.unwrap().iter().map(|x| *x as c_long).collect();
    // Bounds checking copy from constructed vector
    let copy_len = std::cmp::min(val.len(), maxlen as usize);
    value.copy_from(val.as_ptr(), copy_len);
    return copy_len as c_int;
}

//int fjage_param_get_float_array(fjage_gw_t gw, fjage_aid_t aid, const char *param, float *value, int maxlen, int ndx);
#[no_mangle]
pub unsafe extern "C" fn fjage_param_get_float_array(
    gw: *mut Gateway,
    aid: *const c_char,
    param: *const c_char,
    value: *mut c_float,
    maxlen: c_int,
    ndx: c_int,
) -> c_int {
    let val = c_api_exec_timeout_ms(
        gw.as_mut().unwrap().get_float_array(
            &c_api_cstr_to_string(aid),
            &c_api_cstr_to_string(param),
            ndx as i64,
        ),
        1000,
    );
    if val.is_err() {
        return -1;
    }
    let val = val.unwrap();
    if val.is_none() {
        return -1;
    }
    let val: Vec<c_float> = val.unwrap().iter().map(|x| *x as c_float).collect();
    // Bounds checking copy from constructed vector
    let copy_len = std::cmp::min(val.len(), maxlen as usize);
    value.copy_from(val.as_ptr(), copy_len);
    return copy_len as c_int;
}

//int fjage_param_get_double_array(fjage_gw_t gw, fjage_aid_t aid, const char *param, double *value, int maxlen, int ndx);
#[no_mangle]
pub unsafe extern "C" fn fjage_param_get_double_array(
    gw: *mut Gateway,
    aid: *const c_char,
    param: *const c_char,
    value: *mut c_double,
    maxlen: c_int,
    ndx: c_int,
) -> c_int {
    let val = c_api_exec_timeout_ms(
        gw.as_mut().unwrap().get_double_array(
            &c_api_cstr_to_string(aid),
            &c_api_cstr_to_string(param),
            ndx as i64,
        ),
        1000,
    );
    if val.is_err() {
        return -1;
    }
    let val = val.unwrap();
    if val.is_none() {
        return -1;
    }
    let val: Vec<c_double> = val.unwrap().iter().map(|x| *x as c_double).collect();
    // Bounds checking copy from constructed vector
    let copy_len = std::cmp::min(val.len(), maxlen as usize);
    value.copy_from(val.as_ptr(), copy_len);
    return copy_len as c_int;
}

// User will need to free these later
//int fjage_param_get_string_array(fjage_gw_t gw, fjage_aid_t aid, const char *param, char **value,int maxlen, int ndx);
#[no_mangle]
pub unsafe extern "C" fn fjage_param_get_string_array(
    gw: *mut Gateway,
    aid: *const c_char,
    param: *const c_char,
    value: *mut *const c_char,
    maxlen: c_int,
    ndx: c_int,
) -> c_int {
    let val = c_api_exec_timeout_ms(
        gw.as_mut().unwrap().get_string_array(
            &c_api_cstr_to_string(aid),
            &c_api_cstr_to_string(param),
            ndx as i64,
        ),
        1000,
    );
    if val.is_err() {
        return -1;
    }
    let val = val.unwrap();
    if val.is_none() {
        return -1;
    }
    let val = val.unwrap();

    //

    let len = min(val.len(), maxlen as usize);
    let mut allocated_strings: Vec<*const c_char> = Vec::new();
    for i in 0..len {
        let str = val.get(i).unwrap();
        allocated_strings.push(c_api_alloc_cstr(str.to_string()));
        //let str_ptr = (value.add(i)).as_mut().unwrap();
        //str_ptr = str.cast();
        //str_ptr = str;
        //let str: Vec<c_char> = str.as_bytes().iter().map(|x| *x as c_char).collect();

        //str.as_ptr().copy_to(value.add(i).cast(), str.len());
    }
    //let val: Vec<String> = val
    //    .iter()
    //    .map(|x| x.as_str().unwrap().to_string())
    //    .collect();
    // Bounds checking copy from constructed vector (bounds check performed above)
    value.copy_from(allocated_strings.as_ptr(), len);
    return len as c_int;
}

/// Set an integer parameter on an agent. This is a utility function that sends a ParameterReq to an
/// agent, and returns the value from the agent's response.
///
/// @param gw             Gateway
/// @param aid            AgentID of the target agent
/// @param param          Name of the parameter
/// @param value          Value of the parameter
/// @param ndx            Index of the parameter (-1 for non-indexed parameters)
/// @return               0 on success, error code otherwise

//int fjage_param_set_int(fjage_gw_t gw, fjage_aid_t aid, const char *param, int value, int ndx);
#[no_mangle]
pub unsafe extern "C" fn fjage_param_set_int(
    gw: *mut Gateway,
    aid: *const c_char,
    param: *const c_char,
    value: c_int,
    ndx: c_int,
) -> c_int {
    // implies 1000ms timeout
    return c_api_set_param(gw, aid, param, Value::from(value), ndx);
}

/// Set a long parameter on an agent. This is a utility function that sends a ParameterReq to an
/// agent, and returns the value from the agent's response.
///
/// @param gw             Gateway
/// @param aid            AgentID of the target agent
/// @param param          Name of the parameter
/// @param value          Value of the parameter
/// @param ndx            Index of the parameter (-1 for non-indexed parameters)
/// @return               0 on success, error code otherwise

//int fjage_param_set_long(fjage_gw_t gw, fjage_aid_t aid, const char *param, long value, int ndx);
#[no_mangle]
pub unsafe extern "C" fn fjage_param_set_long(
    gw: *mut Gateway,
    aid: *const c_char,
    param: *const c_char,
    value: c_long,
    ndx: c_int,
) -> c_int {
    //unimplemented!();
    return c_api_set_param(gw, aid, param, Value::from(value), ndx);
}

/// Set a float parameter on an agent. This is a utility function that sends a ParameterReq to an
/// agent, and returns the value from the agent's response.
///
/// @param gw             Gateway
/// @param aid            AgentID of the target agent
/// @param param          Name of the parameter
/// @param value          Value of the parameter
/// @param ndx            Index of the parameter (-1 for non-indexed parameters)
/// @return               0 on success, error code otherwise

//int fjage_param_set_float(fjage_gw_t gw, fjage_aid_t aid, const char *param, float value, int ndx);
#[no_mangle]
pub unsafe extern "C" fn fjage_param_set_float(
    gw: *mut Gateway,
    aid: *const c_char,
    param: *const c_char,
    value: c_float,
    ndx: c_int,
) -> c_int {
    //unimplemented!();
    return c_api_set_param(gw, aid, param, Value::from(value), ndx);
}

// NOT IN BASE API

//int fjage_param_set_double(fjage_gw_t gw, fjage_aid_t aid, const char *param, double value, int ndx);
#[no_mangle]
pub unsafe extern "C" fn fjage_param_set_double(
    gw: *mut Gateway,
    aid: *const c_char,
    param: *const c_char,
    value: c_double,
    ndx: c_int,
) -> c_int {
    //unimplemented!();
    return c_api_set_param(gw, aid, param, Value::from(value), ndx);
}

/// Set a boolean parameter on an agent. This is a utility function that sends a ParameterReq to an
/// agent, and returns the value from the agent's response.
///
/// @param gw             Gateway
/// @param aid            AgentID of the target agent
/// @param param          Name of the parameter
/// @param value          Value of the parameter
/// @param ndx            Index of the parameter (-1 for non-indexed parameters)
/// @return               0 on success, error code otherwise

//int fjage_param_set_bool(fjage_gw_t gw, fjage_aid_t aid, const char *param, bool value, int ndx);
#[no_mangle]
pub unsafe extern "C" fn fjage_param_set_bool(
    gw: *mut Gateway,
    aid: *const c_char,
    param: *const c_char,
    value: bool,
    ndx: c_int,
) -> c_int {
    //unimplemented!();
    return c_api_set_param(gw, aid, param, Value::from(value), ndx);
}

/// Set a string parameter on an agent. This is a utility function that sends a ParameterReq to an
/// agent, and returns the value from the agent's response.
///
/// @param gw             Gateway
/// @param aid            AgentID of the target agent
/// @param param          Name of the parameter
/// @param value          Value of the parameter
/// @param ndx            Index of the parameter (-1 for non-indexed parameters)
/// @return               0 on success, error code otherwise

//int fjage_param_set_string(fjage_gw_t gw, fjage_aid_t aid, const char *param, const char *value, int ndx);
#[no_mangle]
pub unsafe extern "C" fn fjage_param_set_string(
    gw: *mut Gateway,
    aid: *const c_char,
    param: *const c_char,
    value: *const c_char,
    ndx: c_int,
) -> c_int {
    //unimplemented!();
    return c_api_set_param(
        gw,
        aid,
        param,
        Value::from(c_api_cstr_to_string(value)),
        ndx,
    );
}

// Experimental, not in base C api
//int fjage_param_set_bool_array(fjage_gw_t gw, fjage_aid_t aid, const char *param, bool *value, int len, int ndx);
#[no_mangle]
pub unsafe extern "C" fn fjage_param_set_bool_array(
    gw: *mut Gateway,
    aid: *const c_char,
    param: *const c_char,
    value: *const bool,
    len: c_int,
    ndx: c_int,
) -> c_int {
    //unimplemented!();
    return c_api_set_param(
        gw,
        aid,
        param,
        Value::from(slice::from_raw_parts(value, len as usize)),
        ndx,
    );
}
// Experimental, not in base C api
//int fjage_param_set_int_array(fjage_gw_t gw, fjage_aid_t aid, const char *param, int *value, int len, int ndx);
#[no_mangle]
pub unsafe extern "C" fn fjage_param_set_int_array(
    gw: *mut Gateway,
    aid: *const c_char,
    param: *const c_char,
    value: *const c_int,
    len: c_int,
    ndx: c_int,
) -> c_int {
    let result = c_api_exec_timeout_ms(
        gw.as_mut().unwrap().set_int_array(
            &c_api_cstr_to_string(aid),
            &c_api_cstr_to_string(param),
            Vec::from(slice::from_raw_parts(value, len as usize)),
            ndx as i64,
        ),
        1000,
    );
    if result.is_err() {
        return -1;
    }
    let result = result.unwrap();
    if result.is_ok() {
        return 0;
    } else {
        return -1;
    }
}

// Experimental, not in base C api
//int fjage_param_set_long_array(fjage_gw_t gw, fjage_aid_t aid, const char *param, long *value, int len, int ndx);
#[no_mangle]
pub unsafe extern "C" fn fjage_param_set_long_array(
    gw: *mut Gateway,
    aid: *const c_char,
    param: *const c_char,
    value: *const c_long,
    len: c_int,
    ndx: c_int,
) -> c_int {
    let result = c_api_exec_timeout_ms(
        gw.as_mut().unwrap().set_long_array(
            &c_api_cstr_to_string(aid),
            &c_api_cstr_to_string(param),
            Vec::from(slice::from_raw_parts(value, len as usize)),
            ndx as i64,
        ),
        1000,
    );
    if result.is_err() {
        return -1;
    }
    let result = result.unwrap();
    if result.is_ok() {
        return 0;
    } else {
        return -1;
    }
}

// Experimental, not in base C api
//int fjage_param_set_float_array(fjage_gw_t gw, fjage_aid_t aid, const char *param, float *value, int len, int ndx);
#[no_mangle]
pub unsafe extern "C" fn fjage_param_set_float_array(
    gw: *mut Gateway,
    aid: *const c_char,
    param: *const c_char,
    value: *const c_float,
    len: c_int,
    ndx: c_int,
) -> c_int {
    let result = c_api_exec_timeout_ms(
        gw.as_mut().unwrap().set_float_array(
            &c_api_cstr_to_string(aid),
            &c_api_cstr_to_string(param),
            Vec::from(slice::from_raw_parts(value, len as usize)),
            ndx as i64,
        ),
        1000,
    );
    if result.is_err() {
        return -1;
    }
    let result = result.unwrap();
    if result.is_ok() {
        return 0;
    } else {
        return -1;
    }
}

// Experimental, not in base C api
//int fjage_param_set_double_array(fjage_gw_t gw, fjage_aid_t aid, const char *param, double *value, int len, int ndx);
#[no_mangle]
pub unsafe extern "C" fn fjage_param_set_double_array(
    gw: *mut Gateway,
    aid: *const c_char,
    param: *const c_char,
    value: *const c_double,
    len: c_int,
    ndx: c_int,
) -> c_int {
    //unimplemented!();
    let result = c_api_exec_timeout_ms(
        gw.as_mut().unwrap().set_double_array(
            &c_api_cstr_to_string(aid),
            &c_api_cstr_to_string(param),
            Vec::from(slice::from_raw_parts(value, len as usize)),
            ndx as i64,
        ),
        1000,
    );
    if result.is_err() {
        return -1;
    }
    let result = result.unwrap();
    if result.is_ok() {
        return 0;
    } else {
        return -1;
    }
}

// Experimental, not in base C api
//int fjage_param_set_string_array(fjage_gw_t gw, fjage_aid_t aid, const char *param, const char **value, int len, int ndx);
#[no_mangle]
pub unsafe extern "C" fn fjage_param_set_string_array(
    gw: *mut Gateway,
    aid: *const c_char,
    param: *const c_char,
    value: *const *const c_char,
    len: c_int,
    ndx: c_int,
) -> c_int {
    //unimplemented!();

    //println!(
    //    "{:?}",
    //    Vec::<c_char>::from_raw_parts(value.cast_mut().cast(), 5, 5)
    //);
    let mut str_vec: Vec<String> = Vec::new();
    for i in 0..len {
        str_vec.push(c_api_cstr_to_string(*value.add(i as usize)));
    }
    return c_api_set_param(gw, aid, param, Value::from(str_vec), ndx);
}

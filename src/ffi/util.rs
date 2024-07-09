use std::{
    ffi::{c_char, c_int, c_long, CStr, CString},
    time::Duration,
};

use serde_json::Value;

use crate::{
    api::gateway::Gateway,
    core::{
        message::{Message, Performative},
        param::ParameterManipulation,
    },
};

#[macro_export]
macro_rules! msg_get_array {
    ($msg:expr, $key:expr, $value:expr, $maxlen:expr, $type:ty, $map_fn:expr) => {
        'inner: {
            let val = fjage_msg_t::get($msg, $key);
            if !val.is_array() {
                break 'inner -1;
            }
            let val = val.as_array().unwrap();
            let val: Vec<$type> = val.iter().map($map_fn).collect();
            let len = val.len();
            if $value.is_null() {
                break 'inner len as c_int;
            }
            let copy_len = min(len, $maxlen as usize);
            $value.copy_from(val.as_ptr(), copy_len);
            copy_len as c_int
        }
    };
}

pub unsafe fn c_api_cstr_to_string(str: *const c_char) -> String {
    let str = CStr::from_ptr(str);
    return String::from_utf8_lossy(str.to_bytes()).to_string();
}

pub unsafe fn c_api_alloc_cstr(aid: String) -> *const c_char {
    let aid_ptr = CString::new(aid).unwrap();
    let aid_ptr = aid_ptr.into_boxed_c_str();
    let aid_ptr = Box::into_raw(aid_ptr);
    return aid_ptr.cast();
}
pub unsafe fn c_api_free_cstr(aid: *mut c_char) {
    let _aid_ptr: Box<&[u8]> = Box::from_raw(aid.cast());
}

pub unsafe fn c_api_alloc_msg() -> *mut Message {
    let msg = Box::new(Message::new());
    println!("Allocated a message");
    return Box::into_raw(msg);
}
pub unsafe fn c_api_free_msg(msg: *mut Message) {
    let _msg = Box::from_raw(msg);
    println!("Deallocated a message");
}

pub unsafe fn c_api_perf_to_int(perf: &Performative) -> c_int {
    return match perf {
        Performative::NONE => 0,
        Performative::REQUEST => 1,
        Performative::AGREE => 2,
        Performative::REFUSE => 3,
        Performative::FAILURE => 4,
        Performative::INFORM => 5,
        Performative::CONFIRM => 6,
        Performative::DISCONFIRM => 7,
        Performative::QUERY_IF => 8,
        Performative::NOT_UNDERSTOOD => 9,
        Performative::CPF => 10,
        Performative::PROPOSE => 11,
        Performative::CANCEL => 12,
    };
}
pub unsafe fn c_api_int_to_perf(perf: c_int) -> Performative {
    match perf {
        1 => Performative::REQUEST,
        2 => Performative::AGREE,
        3 => Performative::REFUSE,
        4 => Performative::FAILURE,
        5 => Performative::INFORM,
        6 => Performative::CONFIRM,
        7 => Performative::DISCONFIRM,
        8 => Performative::QUERY_IF,
        9 => Performative::NOT_UNDERSTOOD,
        10 => Performative::CPF,
        11 => Performative::PROPOSE,
        12 => Performative::CANCEL,
        _ => Performative::NONE,
    }
}

#[allow(non_camel_case_types)]
pub struct fjage_msg_t {
    //msg: GenericMessage,
    pub msg: Message,
    heap: Vec<*const c_char>,
}
impl fjage_msg_t {
    pub unsafe fn alloc() -> *mut fjage_msg_t {
        Box::into_raw(Box::new(fjage_msg_t {
            msg: Message::new(),
            heap: Vec::new(),
        }))
    }
    pub unsafe fn alloc_str(&mut self, str: String) -> *const c_char {
        let str_ptr = c_api_alloc_cstr(str);
        self.heap.push(str_ptr);
        return str_ptr;
    }
    pub unsafe fn alloc_str_s(msg: *mut fjage_msg_t, str: String) -> *const c_char {
        let str_ptr = c_api_alloc_cstr(str);
        msg.as_mut().unwrap().heap.push(str_ptr);
        return str_ptr;
    }
    pub unsafe fn free(msg: *mut fjage_msg_t) {
        if msg.is_null() {
            return;
        }
        for str in msg.as_ref().unwrap().heap.iter() {
            c_api_free_cstr(str.cast_mut());
        }
        let _msg: Box<fjage_msg_t> = Box::from_raw(msg);
    }
    pub unsafe fn send(gw: *mut Gateway, msg: *mut fjage_msg_t) {
        gw.as_mut().unwrap().send(
            fjage_msg_t::strkey_get(msg, "recipient")
                .as_str()
                .unwrap_or(""),
            msg.as_mut().unwrap().msg.clone(),
        );
        fjage_msg_t::free(msg);
    }
    pub unsafe fn request(
        gw: *mut Gateway,
        msg: *mut fjage_msg_t,
        timeout: c_long,
    ) -> *const fjage_msg_t {
        let req = msg.as_mut().unwrap().msg.clone();
        let rsp = gw.as_mut().unwrap().request_timeout(
            &req.data.recipient,
            req.clone(),
            Duration::from_millis(timeout as u64),
        );
        fjage_msg_t::free(msg);

        if rsp.is_none() {
            return std::ptr::null();
        }
        let boxed_msg = fjage_msg_t::alloc();
        boxed_msg.as_mut().unwrap().msg = rsp.unwrap();
        return boxed_msg;
    }
    pub unsafe fn set(msg: *mut fjage_msg_t, key: *const c_char, value: Value) {
        let msg = msg.as_mut().unwrap();
        let msg = &mut msg.msg;
        let data = &mut msg.data;
        let key = c_api_cstr_to_string(key);

        match key.as_str() {
            "msgID" => {
                data.msgID = value.as_str().unwrap().to_string();
            }
            "perf" => {
                data.perf = c_api_int_to_perf(value.as_i64().unwrap() as c_int);
            }
            "recipient" => {
                data.recipient = value.as_str().unwrap().to_string();
            }
            "inReplyTo" => {
                data.inReplyTo = Some(value.as_str().unwrap().to_string());
            }
            "sender" => {
                data.sender = value.as_str().unwrap().to_string();
            }
            "data" => {
                data.fields.insert("data".to_string(), value);
            }
            "signal" => {
                data.fields.insert("signal".to_string(), value);
            }
            _ => {
                data.fields.insert(key, value);
            }
        }
    }
    pub unsafe fn strkey_get(msg: *mut fjage_msg_t, key: &str) -> Value {
        let key = CString::new(key).unwrap();
        return fjage_msg_t::get(msg, key.as_ptr());
    }
    pub unsafe fn strkey_set(msg: *mut fjage_msg_t, key: &str, value: Value) {
        let key = CString::new(key).unwrap();
        return fjage_msg_t::set(msg, key.as_ptr(), value);
    }
    pub unsafe fn get(msg: *mut fjage_msg_t, key: *const c_char) -> Value {
        let msg = msg.as_mut().unwrap();
        let msg = &mut msg.msg;
        let data = &mut msg.data;
        let key = c_api_cstr_to_string(key);

        println!("getting: {}", key);

        match key.as_str() {
            "msgID" => Value::String(data.msgID.clone()),
            "perf" => Value::Number(c_api_perf_to_int(&data.perf).into()),
            "recipient" => {
                if data.recipient.is_empty() {
                    Value::Null
                } else {
                    Value::String(data.recipient.clone())
                }
            }
            "inReplyTo" => {
                if data.inReplyTo.is_some() {
                    return Value::String(data.inReplyTo.clone().unwrap());
                } else {
                    return Value::Null;
                }
            }
            "sender" => Value::String(data.sender.clone()),
            "data" => data.fields.get("data").unwrap().clone(),
            "signal" => data.fields.get("signal").unwrap().clone(),
            _ => {
                let value = data.fields.get_mut(&key);
                if value.is_none() {
                    return Value::Null;
                }
                let value = value.unwrap();
                return value.clone();
            }
        }
    }
}

pub unsafe fn c_api_set_param(
    gw: *mut Gateway,
    aid: *const c_char,
    param: *const c_char,
    value: Value,
    ndx: c_int,
) -> c_int {
    let val = gw.as_mut().unwrap().set_param(
        &c_api_cstr_to_string(aid),
        &c_api_cstr_to_string(param),
        value.clone(),
        ndx as i64,
    );

    if val.is_ok() {
        return 0;
    } else {
        return -1;
    }
}
pub unsafe fn c_api_get_param(
    gw: *mut Gateway,
    aid: *const c_char,
    param: *const c_char,
    ndx: c_int,
) -> Value {
    //let rt = super::runtime.as_mut().unwrap();
    //let val = rt.block_on(async {
    //    tokio::time::timeout(
    //        Duration::from_millis(1000),
    let val = gw.as_mut().unwrap().get_param(
        &c_api_cstr_to_string(aid),
        &c_api_cstr_to_string(param),
        ndx as i64,
    );
    //    )
    //    .await
    //});

    if val.is_none() {
        return Value::Null;
    }
    return val.unwrap();
}

/*
pub unsafe fn c_api_exec_timeout_ms<F: Future>(
    future: F,
    _timeout: c_int,
) -> Result<<F as std::future::Future>::Output, Elapsed> {
    let rt = super::RUNTIME.as_mut().unwrap();
    return rt.block_on(async { tokio::time::timeout(Duration::from_millis(1000), future).await });
}

pub unsafe fn c_api_exec<F: Future>(future: F) -> <F as std::future::Future>::Output {
    let rt = super::RUNTIME.as_mut().unwrap();
    return rt.block_on(async { future.await });
}*/

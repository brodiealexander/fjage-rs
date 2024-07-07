use std::{
    ffi::{c_char, c_int, c_long, CStr, CString},
    future::Future,
    time::Duration,
};

use serde_json::Value;
use tokio::time::error::Elapsed;

use crate::{
    core::{
        message::{Message, Performative},
        param::ParameterManipulation,
    },
    remote::gateway::Gateway,
};

pub unsafe fn cstr_to_String(str: *const c_char) -> String {
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
    let aid_ptr: Box<&[u8]> = Box::from_raw(aid.cast());
}

pub unsafe fn c_api_alloc_msg() -> *mut Message {
    let msg = Box::new(Message::new());
    println!("Allocated a message");
    return Box::into_raw(msg);
}
pub unsafe fn c_api_free_msg(msg: *mut Message) {
    let msg = Box::from_raw(msg);
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

pub struct GenericMessage {
    //msg: GenericMessage,
    pub msg: Message,
    heap: Vec<*const c_char>,
}
impl GenericMessage {
    pub unsafe fn alloc() -> *mut GenericMessage {
        Box::into_raw(Box::new(GenericMessage {
            msg: Message::new(),
            heap: Vec::new(),
        }))
    }
    pub unsafe fn alloc_str(&mut self, str: String) -> *const c_char {
        let str_ptr = c_api_alloc_cstr(str);
        self.heap.push(str_ptr);
        return str_ptr;
    }
    pub unsafe fn alloc_str_s(msg: *mut GenericMessage, str: String) -> *const c_char {
        let str_ptr = c_api_alloc_cstr(str);
        msg.as_mut().unwrap().heap.push(str_ptr);
        return str_ptr;
    }
    pub unsafe fn free(msg: *mut GenericMessage) {
        if msg.is_null() {
            return;
        }
        for str in msg.as_ref().unwrap().heap.iter() {
            c_api_free_cstr(str.cast_mut());
        }
        let msg: Box<GenericMessage> = Box::from_raw(msg);
    }
    pub unsafe fn send(gw: *mut Gateway, msg: *mut GenericMessage) {
        gw.as_mut()
            .unwrap()
            .send_raw(msg.as_mut().unwrap().msg.clone());
        GenericMessage::free(msg);
    }
    pub unsafe fn request(
        gw: *mut Gateway,
        msg: *mut GenericMessage,
        timeout: c_long,
    ) -> *const GenericMessage {
        let rt = super::runtime.as_mut().unwrap();
        let req = msg.as_mut().unwrap().msg.clone();
        let rsp = rt.block_on(async {
            tokio::time::timeout(
                Duration::from_millis(timeout as u64),
                gw.as_mut()
                    .unwrap()
                    .request(&req.data.recipient, req.clone()),
            )
            .await
        });
        GenericMessage::free(msg);
        if rsp.is_ok() {
            let rsp = rsp.unwrap();
            if rsp.is_none() {
                return std::ptr::null();
            }
            let boxed_msg = GenericMessage::alloc();
            boxed_msg.as_mut().unwrap().msg = rsp.unwrap();
            return boxed_msg;
        } else {
            return std::ptr::null();
        }
    }
    pub unsafe fn set(msg: *mut GenericMessage, key: *const c_char, value: Value) {
        let msg = msg.as_mut().unwrap();
        let msg = &mut msg.msg;
        let data = &mut msg.data;
        let key = cstr_to_String(key);

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
    pub unsafe fn strkey_get(msg: *mut GenericMessage, key: &str) -> Value {
        let key = CString::new(key).unwrap();
        return GenericMessage::get(msg, key.as_ptr());
    }
    pub unsafe fn strkey_set(msg: *mut GenericMessage, key: &str, value: Value) {
        let key = CString::new(key).unwrap();
        return GenericMessage::set(msg, key.as_ptr(), value);
    }
    pub unsafe fn get(msg: *mut GenericMessage, key: *const c_char) -> Value {
        let msg = msg.as_mut().unwrap();
        let msg = &mut msg.msg;
        let data = &mut msg.data;
        let key = cstr_to_String(key);

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
    let val = c_api_exec_timeout_ms(
        gw.as_mut().unwrap().set_param(
            &cstr_to_String(aid),
            &cstr_to_String(param),
            value.clone(),
            ndx as i64,
        ),
        1000,
    );
    if val.is_err() {
        return -1;
    }
    let val = val.unwrap();
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
    let val = c_api_exec_timeout_ms(
        gw.as_mut()
            .unwrap()
            .get_param(&cstr_to_String(aid), &cstr_to_String(param), ndx as i64),
        1000 as c_int,
    );
    //    )
    //    .await
    //});
    if val.is_err() {
        return Value::Null;
    }
    let val = val.unwrap();
    if val.is_none() {
        return Value::Null;
    }
    return val.unwrap();
}

pub unsafe fn c_api_exec_timeout_ms<F: Future>(
    future: F,
    timeout: c_int,
) -> Result<<F as std::future::Future>::Output, Elapsed> {
    let rt = super::runtime.as_mut().unwrap();
    return rt.block_on(async { tokio::time::timeout(Duration::from_millis(1000), future).await });
}

pub unsafe fn c_api_exec<F: Future>(future: F) -> <F as std::future::Future>::Output {
    let rt = super::runtime.as_mut().unwrap();
    return rt.block_on(async { future.await });
}

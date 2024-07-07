use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::core::message::{Message, Performative};

#[allow(non_camel_case_types, non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetFileReq {
    pub msgID: String,
    pub perf: Performative,
    pub recipient: String,
    pub inReplyTo: Option<String>,
    pub sender: String,
    pub sentAt: i64,
    pub filename: String,
    pub ofs: u64,
    pub len: u64,
}
impl GetFileReq {
    pub fn new(filename: &str) -> GetFileReq {
        return GetFileReq {
            msgID: Uuid::new_v4().to_string(),
            perf: Performative::REQUEST,
            recipient: String::new(),
            inReplyTo: None,
            sender: String::new(),
            sentAt: 0,
            filename: filename.to_string(),
            ofs: 0,
            len: 0,
        };
    }
    pub fn from_msg(msg: Message) -> GetFileReq {
        return serde_json::from_value(serde_json::to_value(msg.data).unwrap()).unwrap();
    }
    pub fn to_msg(&mut self) -> Message {
        return Message {
            clazz: "org.arl.fjage.shell.GetFileReq".to_string(),
            data: serde_json::from_value(serde_json::to_value(self).unwrap()).unwrap(),
        };
    }
}

#[allow(non_camel_case_types, non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetFileRsp {
    pub msgID: String,
    pub perf: Performative,
    pub recipient: String,
    pub inReplyTo: Option<String>,
    pub sender: String,
    pub sentAt: i64,
    pub filename: String,
    pub dir: bool,
    pub contents: Vec<u8>,
    pub ofs: u64,
}
impl GetFileRsp {
    pub fn new(filename: &str) -> GetFileRsp {
        return GetFileRsp {
            msgID: Uuid::new_v4().to_string(),
            perf: Performative::REQUEST,
            recipient: String::new(),
            inReplyTo: None,
            sender: String::new(),
            sentAt: 0,
            filename: filename.to_string(),
            dir: false,
            contents: Vec::new(),
            ofs: 0,
        };
    }
    pub fn from_msg(msg: Message) -> GetFileRsp {
        return serde_json::from_value(serde_json::to_value(msg.data).unwrap()).unwrap();
    }
    pub fn to_msg(&mut self) -> Message {
        return Message {
            clazz: "org.arl.fjage.shell.GetFileRsp".to_string(),
            data: serde_json::from_value(serde_json::to_value(self).unwrap()).unwrap(),
        };
    }
}

#[allow(non_camel_case_types, non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PutFileReq {
    pub msgID: String,
    pub perf: Performative,
    pub recipient: String,
    pub inReplyTo: Option<String>,
    pub sender: String,
    pub sentAt: i64,
    pub filename: String,
    pub contents: Option<Vec<u8>>,
    pub ofs: u64,
}
impl PutFileReq {
    pub fn new(filename: &str) -> PutFileReq {
        return PutFileReq {
            msgID: Uuid::new_v4().to_string(),
            perf: Performative::REQUEST,
            recipient: String::new(),
            inReplyTo: None,
            sender: String::new(),
            sentAt: 0,
            filename: filename.to_string(),
            contents: Some(Vec::new()),
            ofs: 0,
        };
    }
    pub fn new_contents(filename: &str, contents: &str) -> PutFileReq {
        let mut req = PutFileReq::new(filename);
        req.contents = Some(contents.as_bytes().to_vec());
        return req;
    }
    pub fn from_msg(msg: Message) -> PutFileReq {
        return serde_json::from_value(serde_json::to_value(msg.data).unwrap()).unwrap();
    }
    pub fn to_msg(&mut self) -> Message {
        return Message {
            clazz: "org.arl.fjage.shell.PutFileReq".to_string(),
            data: serde_json::from_value(serde_json::to_value(self).unwrap()).unwrap(),
        };
    }
}

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::core::message::{Message, Performative};

#[allow(non_camel_case_types, non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ShellExecReq {
    pub msgID: String,
    pub perf: Performative,
    pub recipient: String,
    pub inReplyTo: Option<String>,
    pub sender: String,
    pub sentAt: i64,
    pub ans: bool,
    pub cmd: String,
}
impl ShellExecReq {
    pub fn new(cmd: &str) -> ShellExecReq {
        return ShellExecReq {
            msgID: Uuid::new_v4().to_string(),
            perf: Performative::REQUEST,
            recipient: String::new(),
            inReplyTo: None,
            sender: String::new(),
            sentAt: 0,
            cmd: cmd.to_string(),
            ans: true,
        };
    }
    pub fn from_msg(msg: Message) -> ShellExecReq {
        return serde_json::from_value(serde_json::to_value(msg.data).unwrap()).unwrap();
    }
    pub fn to_msg(&mut self) -> Message {
        return Message {
            clazz: "org.arl.fjage.shell.ShellExecReq".to_string(),
            data: serde_json::from_value(serde_json::to_value(self).unwrap()).unwrap(),
        };
    }
}

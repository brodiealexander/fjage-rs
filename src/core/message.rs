use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use uuid::Uuid;

use crate::protocol::base64::*;

#[allow(non_camel_case_types, non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]

pub struct Message {
    pub clazz: String,
    pub data: Payload,
}
impl Message {
    pub fn new() -> Message {
        Message {
            clazz: "org.arl.fjage.GenericMessage".to_string(),
            data: Payload::new(),
        }
    }
    pub fn decode_java_classes(&mut self) {
        for field in self.data.fields.clone().keys() {
            let msg_data = self.data.fields.get(field).unwrap();
            // Make sure it is an object
            //println!("CHECK FOR OBJECT: ");
            if !msg_data.is_object() {
                //println!("FAIL");
                continue;
            }
            let msg_data = msg_data.as_object().unwrap();

            // Check for clazz
            //println!("CHECK FOR CLAZZ: ");
            if !msg_data.contains_key("clazz") {
                //println!("FAIL");
                continue;
            }

            // Check for the two possible keys
            //println!("CHECK FOR DATA/SIG: ");
            let key = if msg_data.contains_key("data") {
                "data"
            } else if msg_data.contains_key("signal") {
                "signal"
            } else {
                //println!("FAIL");
                continue;
            };
            let val: Value = match msg_data.get("clazz").unwrap().as_str().unwrap() {
                "[F" => {
                    //println!("Type: Float32");
                    base64_to_f32(msg_data.get(key).unwrap().as_str().unwrap()).into()
                }
                "[I" => {
                    //println!("Type: Int32");
                    base64_to_i32(msg_data.get(key).unwrap().as_str().unwrap()).into()
                }
                "[D" => {
                    //println!("Type: Double(F64)");
                    base64_to_f64(msg_data.get(key).unwrap().as_str().unwrap()).into()
                }
                "[J" => {
                    //println!("Type: Long int(I64)");
                    base64_to_i64(msg_data.get(key).unwrap().as_str().unwrap()).into()
                }
                "[B" => {
                    //println!("Type: Bytearray(Vec<u8>");
                    base64_to_u8(msg_data.get(key).unwrap().as_str().unwrap()).into()
                }
                "[Ljava.lang.String;" => msg_data.get("data").unwrap().to_owned(),
                _ => {
                    //println!("Unsupported B64 type");
                    Value::Null
                }
            };
            // Replace with decoded
            self.data.fields.insert(field.to_string(), val);
        }
        //println!("POST_DECODE: {:?}", self);
    }
}

#[allow(non_camel_case_types, non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Payload {
    pub msgID: String,
    pub perf: Performative,
    pub recipient: String,
    pub inReplyTo: Option<String>,
    pub sender: String,
    //#[serde(default)]
    pub sentAt: Option<i64>,
    #[serde(flatten)]
    pub fields: HashMap<String, Value>,
}
impl Payload {
    pub fn new() -> Payload {
        Payload {
            msgID: Uuid::new_v4().to_string(),
            perf: Performative::REQUEST,
            recipient: String::new(),
            inReplyTo: None,
            sender: String::new(),
            sentAt: None,
            fields: HashMap::new(),
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Performative {
    NONE = 0, //The C API defines this, but the spec does not.
    REQUEST = 1,
    AGREE = 2,
    REFUSE = 3,
    FAILURE = 4,
    INFORM = 5,
    CONFIRM = 6,
    DISCONFIRM = 7,
    QUERY_IF = 8,
    NOT_UNDERSTOOD = 9,
    CPF = 10,
    PROPOSE = 11,
    CANCEL = 12,
}

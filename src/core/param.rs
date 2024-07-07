use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

use crate::protocol::base64::*;

use super::message::{Message, Performative};

/*fn skip_if_requests_null(val: Option<HashMap<String,Value>>) {
    if val.is_none()
}*/

#[allow(non_camel_case_types, non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ParameterReq {
    pub msgID: String,
    pub perf: Performative,
    pub recipient: String,
    pub inReplyTo: Option<String>,
    pub sender: String,
    pub sentAt: i64,
    pub index: i64,
    pub param: Option<String>,
    pub value: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requests: Option<HashMap<String, Value>>,
}
impl ParameterReq {
    pub fn new() -> ParameterReq {
        return ParameterReq {
            msgID: Uuid::new_v4().to_string(),
            perf: Performative::REQUEST,
            recipient: String::new(),
            inReplyTo: None,
            sender: String::new(),
            sentAt: 0,
            index: -1,
            param: None,
            value: Value::Null,
            requests: None,
        };
    }
    pub fn set(param: &str, value: Value, index: i64) -> ParameterReq {
        let mut req = ParameterReq::new();
        req.param = Some(param.to_string());
        req.value = value;
        req.index = index;
        return req;
    }
    pub fn set_many(map: HashMap<String, Value>) -> ParameterReq {
        let mut req = ParameterReq::new();
        req.requests = Some(map);
        return req;
    }
    pub fn get(param: &str, index: i64) -> ParameterReq {
        let mut req = ParameterReq::new();
        req.param = Some(param.to_string());
        req.index = index;
        return req;
    }
    pub fn get_many(param: &Vec<String>) -> ParameterReq {
        let mut req = ParameterReq::new();
        let mut map = HashMap::new();
        for entry in param.iter() {
            map.insert(entry.clone(), Value::Null);
        }
        req.requests = Some(map);
        return req;
    }
    pub fn from_msg(msg: Message) -> ParameterReq {
        return serde_json::from_value(serde_json::to_value(msg.data).unwrap()).unwrap();
    }
    pub fn to_msg(&mut self) -> Message {
        return Message {
            clazz: "org.arl.fjage.param.ParameterReq".to_string(),
            data: serde_json::from_value(serde_json::to_value(self).unwrap()).unwrap(),
        };
    }
}

#[allow(non_camel_case_types, non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ParameterRsp {
    pub msgID: String,
    pub perf: Performative,
    pub recipient: String,
    pub inReplyTo: Option<String>,
    pub sender: String,
    pub sentAt: i64,
    pub index: i64,
    pub param: Option<String>,
    pub value: Value,
    pub values: Option<HashMap<String, Value>>,
    pub readonly: Option<Vec<String>>,
}
impl ParameterRsp {
    pub fn new() -> ParameterRsp {
        return ParameterRsp {
            msgID: Uuid::new_v4().to_string(),
            perf: Performative::REQUEST,
            recipient: String::new(),
            inReplyTo: None,
            sender: String::new(),
            sentAt: 0,
            index: -1,
            param: None,
            value: Value::Null,
            values: None,
            readonly: None,
        };
    }
    pub fn from_msg(msg: Message) -> ParameterRsp {
        return serde_json::from_value(serde_json::to_value(msg.data).unwrap()).unwrap();
    }
    pub fn to_msg(&mut self) -> Message {
        return Message {
            clazz: "org.arl.fjage.param.ParameterRsp".to_string(),
            data: serde_json::from_value(serde_json::to_value(self).unwrap()).unwrap(),
        };
    }
}

pub trait ParameterManipulation {
    async fn param_req(&mut self, aid: &str, req: ParameterReq) -> Option<ParameterRsp>;
    async fn get_param(&mut self, aid: &str, param: &str, index: i64) -> Option<Value> {
        let value = self.param_req(aid, ParameterReq::get(param, index)).await;
        if value.is_none() {
            return None;
        } else {
            return Some(value.unwrap().value);
        }
    }
    async fn set_param(
        &mut self,
        aid: &str,
        param: &str,
        value: Value,
        index: i64,
    ) -> Result<Value, i32> {
        let rsp = self
            .param_req(aid, ParameterReq::set(param, value.clone(), index))
            .await;
        if rsp.is_none() {
            return Err(-1);
        }
        let rsp = rsp.unwrap().value;
        println!("GOT BACK: {:?}", rsp);
        println!("AGAINST: {:?}", rsp);
        if value == rsp {
            println!("PASSED EQ CHECK");
            return Ok(value);
        } else {
            println!("FAILED EQ CHECK");
            return Err(-1);
        }
    }
    //
    async fn get_bool(&mut self, aid: &str, param: &str, index: i64) -> Option<bool> {
        let req = ParameterReq::get(param, index);
        let rsp = self.param_req(aid, req).await;
        if rsp.is_none() {
            return None;
        }
        let rsp = rsp.unwrap().value;
        // Optimization note: if as_bool is called on a non-boolean or null JSON value, serde-json will cause us to return 'None' here as intended.
        return rsp.as_bool();
    }
    // NOTE: serde_json interprets all numerical values as either long or double. The get_int and get_float methods are only here for convenience.
    async fn get_int(&mut self, aid: &str, param: &str, index: i64) -> Option<i32> {
        let rsp = self.get_long(aid, param, index).await;
        if rsp.is_none() {
            return None;
        }
        return Some(rsp.unwrap() as i32);
    }
    async fn get_long(&mut self, aid: &str, param: &str, index: i64) -> Option<i64> {
        let req = ParameterReq::get(param, index);
        let rsp = self.param_req(aid, req).await;
        if rsp.is_none() {
            return None;
        }
        let rsp = rsp.unwrap().value;
        return rsp.as_i64();
    }
    async fn get_float(&mut self, aid: &str, param: &str, index: i64) -> Option<f32> {
        let rsp = self.get_double(aid, param, index).await;
        if rsp.is_none() {
            return None;
        }
        return Some(rsp.unwrap() as f32);
    }
    async fn get_double(&mut self, aid: &str, param: &str, index: i64) -> Option<f64> {
        let req = ParameterReq::get(param, index);
        let rsp = self.param_req(aid, req).await;
        if rsp.is_none() {
            return None;
        }
        let rsp = rsp.unwrap().value;
        return rsp.as_f64();
    }
    async fn get_string(&mut self, aid: &str, param: &str, index: i64) -> Option<String> {
        let req = ParameterReq::get(param, index);
        let rsp = self.param_req(aid, req).await;
        if rsp.is_none() {
            return None;
        }
        let rsp = rsp.unwrap().value;
        if !rsp.is_string() {
            return None;
        }
        return Some(rsp.as_str().unwrap().to_string());
    }

    //
    async fn set_bool(
        &mut self,
        aid: &str,
        param: &str,
        value: bool,
        index: i64,
    ) -> Result<bool, i32> {
        let req = ParameterReq::set(param, Value::from(value), index);
        let rsp = self.param_req(aid, req).await;
        if rsp.is_none() {
            return Err(-1);
        }
        let rsp = rsp.unwrap().value;
        if !rsp.is_boolean() {
            return Err(-1);
        }
        let rsp = rsp.as_bool().unwrap();
        if rsp == value {
            return Ok(rsp);
        } else {
            return Err(-1);
        }
    }
    async fn set_int(
        &mut self,
        aid: &str,
        param: &str,
        value: i32,
        index: i64,
    ) -> Result<i32, i32> {
        let rsp = self.set_long(aid, param, value as i64, index).await;
        if rsp.is_ok() {
            return Ok(rsp.unwrap() as i32);
        } else {
            return Err(rsp.unwrap_err());
        }
    }
    async fn set_long(
        &mut self,
        aid: &str,
        param: &str,
        value: i64,
        index: i64,
    ) -> Result<i64, i32> {
        let req = ParameterReq::set(param, Value::from(value), index);
        let rsp = self.param_req(aid, req).await;
        if rsp.is_none() {
            return Err(-1);
        }
        let rsp = rsp.unwrap().value;
        if !rsp.is_i64() {
            return Err(-1);
        }
        let rsp = rsp.as_i64().unwrap();
        if rsp == value {
            return Ok(rsp);
        } else {
            return Err(-1);
        }
    }
    async fn set_float(
        &mut self,
        aid: &str,
        param: &str,
        value: f32,
        index: i64,
    ) -> Result<f32, i32> {
        let rsp = self.set_double(aid, param, value as f64, index).await;
        if rsp.is_ok() {
            return Ok(rsp.unwrap() as f32);
        } else {
            return Err(rsp.unwrap_err());
        }
    }
    async fn set_double(
        &mut self,
        aid: &str,
        param: &str,
        value: f64,
        index: i64,
    ) -> Result<f64, i32> {
        let req = ParameterReq::set(param, Value::from(value), index);
        let rsp = self.param_req(aid, req).await;
        if rsp.is_none() {
            return Err(-1);
        }
        let rsp = rsp.unwrap().value;
        if !rsp.is_f64() {
            return Err(-1);
        }
        let rsp = rsp.as_f64().unwrap();
        if rsp == value {
            return Ok(rsp);
        } else {
            return Err(-1);
        }
    }
    async fn set_string(
        &mut self,
        aid: &str,
        param: &str,
        value: &str,
        index: i64,
    ) -> Result<String, i32> {
        let req = ParameterReq::set(param, Value::from(value), index);
        let rsp = self.param_req(aid, req).await;
        if rsp.is_none() {
            return Err(-1);
        }
        let rsp = rsp.unwrap().value;
        if !rsp.is_string() {
            return Err(-1);
        }
        let rsp = rsp.as_str().unwrap();
        if rsp == value {
            return Ok(rsp.to_string());
        } else {
            return Err(-1);
        }
    }

    // array getters
    async fn get_int_array(&mut self, aid: &str, param: &str, index: i64) -> Option<Vec<i32>> {
        let rsp = self.get_long_array(aid, param, index).await;
        if rsp.is_none() {
            return None;
        }
        let rsp: Vec<i32> = rsp.unwrap().iter().map(|v| *v as i32).collect();
        return Some(rsp);
    }
    async fn get_long_array(&mut self, aid: &str, param: &str, index: i64) -> Option<Vec<i64>> {
        let req = ParameterReq::get(param, index);
        let rsp = self.param_req(aid, req).await;
        if rsp.is_none() {
            return None;
        }
        let rsp = rsp.unwrap().value;
        if !rsp.is_array() {
            return None;
        }
        let rsp = rsp.as_array().unwrap();
        if rsp.is_empty() {
            return Some(Vec::new());
        }
        if !rsp.first().is_some_and(|v| v.is_i64()) {
            return None;
        }
        let rsp: Vec<i64> = rsp.iter().map(|v| v.as_i64().unwrap_or(0)).collect();
        return Some(rsp);
    }
    async fn get_float_array(&mut self, aid: &str, param: &str, index: i64) -> Option<Vec<f32>> {
        {
            let rsp = self.get_double_array(aid, param, index).await;
            if rsp.is_none() {
                return None;
            }
            let rsp: Vec<f32> = rsp.unwrap().iter().map(|v| *v as f32).collect();
            return Some(rsp);
        }
    }
    async fn get_double_array(&mut self, aid: &str, param: &str, index: i64) -> Option<Vec<f64>> {
        let req = ParameterReq::get(param, index);
        let rsp = self.param_req(aid, req).await;
        if rsp.is_none() {
            return None;
        }
        let rsp = rsp.unwrap().value;
        if !rsp.is_array() {
            return None;
        }
        let rsp = rsp.as_array().unwrap();
        if rsp.is_empty() {
            return Some(Vec::new());
        }
        if !rsp.first().is_some_and(|v| v.is_f64()) {
            return None;
        }
        let rsp: Vec<f64> = rsp.iter().map(|v| v.as_f64().unwrap_or(0.0)).collect();
        return Some(rsp);
    }
    async fn get_string_array(
        &mut self,
        aid: &str,
        param: &str,
        index: i64,
    ) -> Option<Vec<String>> {
        {
            let req = ParameterReq::get(param, index);
            let rsp = self.param_req(aid, req).await;
            if rsp.is_none() {
                return None;
            }
            let rsp = rsp.unwrap().value;
            if !rsp.is_array() {
                return None;
            }
            let rsp = rsp.as_array().unwrap();
            if rsp.is_empty() {
                return Some(Vec::new());
            }
            if !rsp.first().is_some_and(|v| v.is_string()) {
                return None;
            }
            let rsp: Vec<String> = rsp
                .iter()
                .map(|v| v.as_str().unwrap_or("").to_string())
                .collect();
            return Some(rsp);
        }
    }
    // array setters
    async fn set_int_array(
        &mut self,
        aid: &str,
        param: &str,
        value: Vec<i32>,
        index: i64,
    ) -> Result<usize, i32> {
        let req = ParameterReq::set(param, b64_obj_from_i32(value.clone()), index);
        let rsp = self.param_req(aid, req).await;
        if rsp.is_none() {
            return Err(-1); // Timeout (most likely)
        }
        let rsp = rsp.unwrap().value;
        if !rsp.is_array() {
            return Err(-1); // Response was not an array
        }
        let rsp = rsp.as_array().unwrap();
        if rsp.is_empty() {
            if value.is_empty() {
                return Ok(0); // Response vector and request vector were empty
            } else {
                return Err(-1); // Response vector was empty, but request vector was not
            }
        }
        if !rsp.first().is_some_and(|v| v.is_i64()) {
            return Err(-1); // First element of response vector is not an integer
        }
        let rsp: Vec<i32> = rsp.iter().map(|v| v.as_i64().unwrap_or(0) as i32).collect();
        if rsp == value {
            return Ok(rsp.len());
        } else {
            return Err(-1); // Fails equality check
        }
    }
    async fn set_long_array(
        &mut self,
        aid: &str,
        param: &str,
        value: Vec<i64>,
        index: i64,
    ) -> Result<usize, i32> {
        let req = ParameterReq::set(param, b64_obj_from_i64(value.clone()), index);
        let rsp = self.param_req(aid, req).await;
        if rsp.is_none() {
            return Err(-1); // Timeout (most likely)
        }
        let rsp = rsp.unwrap().value;
        if !rsp.is_array() {
            return Err(-1); // Response was not an array
        }
        let rsp = rsp.as_array().unwrap();
        if rsp.is_empty() {
            if value.is_empty() {
                return Ok(0); // Response vector and request vector were empty
            } else {
                return Err(-1); // Response vector was empty, but request vector was not
            }
        }
        if !rsp.first().is_some_and(|v| v.is_i64()) {
            return Err(-1); // First element of response vector is not an integer
        }
        let rsp: Vec<i64> = rsp.iter().map(|v| v.as_i64().unwrap_or(0)).collect();
        if rsp == value {
            return Ok(rsp.len());
        } else {
            return Err(-1); // Fails equality check
        }
    }
    async fn set_float_array(
        &mut self,
        aid: &str,
        param: &str,
        value: Vec<f32>,
        index: i64,
    ) -> Result<usize, i32> {
        let req = ParameterReq::set(param, b64_obj_from_f32(value.clone()), index);
        let rsp = self.param_req(aid, req).await;
        if rsp.is_none() {
            return Err(-1); // Timeout (most likely)
        }
        let rsp = rsp.unwrap().value;
        if !rsp.is_array() {
            return Err(-1); // Response was not an array
        }
        let rsp = rsp.as_array().unwrap();
        if rsp.is_empty() {
            if value.is_empty() {
                return Ok(0); // Response vector and request vector were empty
            } else {
                return Err(-1); // Response vector was empty, but request vector was not
            }
        }
        if !rsp.first().is_some_and(|v| v.is_f64()) {
            return Err(-1); // First element of response vector is not a float
        }
        let rsp: Vec<f32> = rsp
            .iter()
            .map(|v| v.as_f64().unwrap_or(0.0) as f32)
            .collect();
        if rsp == value {
            return Ok(rsp.len());
        } else {
            return Err(-1); // Fails equality check
        }
    }
    async fn set_double_array(
        &mut self,
        aid: &str,
        param: &str,
        value: Vec<f64>,
        index: i64,
    ) -> Result<usize, i32> {
        let req = ParameterReq::set(param, b64_obj_from_f64(value.clone()), index);
        let rsp = self.param_req(aid, req).await;
        if rsp.is_none() {
            return Err(-1); // Timeout (most likely)
        }
        let rsp = rsp.unwrap().value;
        if !rsp.is_array() {
            return Err(-1); // Response was not an array
        }
        let rsp = rsp.as_array().unwrap();
        if rsp.is_empty() {
            if value.is_empty() {
                return Ok(0); // Response vector and request vector were empty
            } else {
                return Err(-1); // Response vector was empty, but request vector was not
            }
        }
        if !rsp.first().is_some_and(|v| v.is_f64()) {
            return Err(-1); // First element of response vector is not a float
        }
        let rsp: Vec<f64> = rsp.iter().map(|v| v.as_f64().unwrap_or(0.0)).collect();
        if rsp == value {
            return Ok(rsp.len());
        } else {
            return Err(-1); // Fails equality check
        }
    }
    async fn set_string_array(
        &mut self,
        aid: &str,
        param: &str,
        value: Vec<String>,
        index: i64,
    ) -> Result<usize, i32> {
        let req = ParameterReq::set(param, Value::from(value.clone()), index);
        let rsp = self.param_req(aid, req).await;
        if rsp.is_none() {
            return Err(-1); // Timeout (most likely)
        }
        let rsp = rsp.unwrap().value;
        if !rsp.is_array() {
            return Err(-1); // Response was not an array
        }
        let rsp = rsp.as_array().unwrap();
        if rsp.is_empty() {
            if value.is_empty() {
                return Ok(0); // Response vector and request vector were empty
            } else {
                return Err(-1); // Response vector was empty, but request vector was not
            }
        }
        if !rsp.first().is_some_and(|v| v.is_string()) {
            return Err(-1); // First element of response vector is not a string
        }
        let rsp: Vec<String> = rsp
            .iter()
            .map(|v| v.as_str().unwrap_or("").to_string())
            .collect();
        if rsp == value {
            return Ok(rsp.len());
        } else {
            return Err(-1); // Fails equality check
        }
    }
}

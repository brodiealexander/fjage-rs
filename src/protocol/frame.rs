use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use uuid::Uuid;

use crate::core::message::Message;

//use crate::protocol::message::*;

#[allow(non_camel_case_types, non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]

pub enum Frame {
    #[serde(rename = "alive")]
    Alive(bool),
    #[serde(untagged)]
    Request(RequestFrame),
    #[serde(untagged)]
    Response(ResponseFrame),
}
impl Frame {
    pub fn from_json(frame: &str) -> Option<Frame> {
        let frame: Result<Value, _> = serde_json::from_str(frame);
        if frame.is_err() {
            return None;
        }
        let frame = frame.unwrap();
        if frame.get("action").is_some() {
            return Some(Frame::Request(serde_json::from_value(frame).unwrap()));
        } else if frame.get("inResponseTo").is_some() {
            return Some(Frame::Response(serde_json::from_value(frame).unwrap()));
        } else if frame.get("alive").is_some() {
            return Some(Frame::Alive(frame.get("alive").unwrap().as_bool().unwrap()));
        } else {
            return None;
        }
    }
    pub fn to_json(&mut self) -> String {
        return serde_json::to_string(&self).unwrap();
    }
    pub fn as_req(&mut self) -> Option<RequestFrame> {
        match self {
            Frame::Request(req) => Some(req.to_owned()),
            _ => None,
        }
    }
    pub fn as_rsp(&mut self) -> Option<ResponseFrame> {
        match self {
            Frame::Response(rsp) => Some(rsp.to_owned()),
            _ => None,
        }
    }
    pub fn id(&self) -> Option<&String> {
        match self {
            Frame::Alive(_) => None,
            Frame::Request(req) => req.id(),
            Frame::Response(rsp) => Some(rsp.id()),
        }
    }
}

#[allow(non_camel_case_types, non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "action")]
pub enum RequestFrame {
    agents {
        id: String,
    },
    containsAgent {
        id: String,
        agentID: String,
    },
    services {
        id: String,
    },
    agentForService {
        id: String,
        service: String,
    },
    agentsForService {
        id: String,
        service: String,
    },
    send {
        message: Message,
        #[serde(default)]
        relay: bool,
    },
    // shutdown,
    wantsMessagesFor {
        agentIDs: Vec<String>,
    },
}
impl RequestFrame {
    pub fn id(&self) -> Option<&String> {
        match self {
            RequestFrame::agents { id } => Some(id),
            RequestFrame::containsAgent { id, agentID: _ } => Some(id),
            RequestFrame::services { id } => Some(id),
            RequestFrame::agentForService { id, service: _ } => Some(id),
            RequestFrame::agentsForService { id, service: _ } => Some(id),

            _ => None,
        }
    }
}
#[allow(non_camel_case_types, non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "inResponseTo")]
/* If a message contains the "inResponseTo" field, we define it as a response and match against outstanding requests. */
pub enum ResponseFrame {
    agents { id: String, agentIDs: Vec<String> }, // Need to ask Chinmay about this and services
    containsAgent { id: String, answer: bool },
    services { id: String, services: Vec<String> },
    agentForService { id: String, agentID: Option<String> }, // Error case?
    agentsForService { id: String, agentIDs: Vec<String> },
    // shutdown ?
}
impl ResponseFrame {
    pub fn id(&self) -> &String {
        match self {
            ResponseFrame::agents { id, agentIDs: _ } => id,
            ResponseFrame::containsAgent { id, answer: _ } => id,
            ResponseFrame::services { id, services: _ } => id,
            ResponseFrame::agentForService { id, agentID: _ } => id,
            ResponseFrame::agentsForService { id, agentIDs: _ } => id,
        }
    }
    pub fn get_agentID(&self) -> Option<String> {
        match self {
            ResponseFrame::agents { id: _, agentIDs } => Some(agentIDs.first().unwrap().clone()),
            ResponseFrame::agentForService { id: _, agentID } => {
                if agentID.is_some() {
                    Some(agentID.clone().unwrap())
                } else {
                    None
                }
            }
            ResponseFrame::agentsForService { id: _, agentIDs } => {
                Some(agentIDs.first().unwrap().clone())
            }
            _ => None,
        }
    }
    pub fn get_agentIDs(&self) -> Option<Vec<String>> {
        match self {
            ResponseFrame::agents { id: _, agentIDs } => Some(agentIDs.clone()),
            ResponseFrame::agentForService { id: _, agentID } => {
                if agentID.is_some() {
                    Some(vec![agentID.clone().unwrap()])
                } else {
                    None
                }
            }
            ResponseFrame::agentsForService { id: _, agentIDs } => Some(agentIDs.clone()),
            _ => None,
        }
    }
    pub fn get_services(&self) -> Option<Vec<String>> {
        match self {
            ResponseFrame::services { id: _, services } => Some(services.clone()),
            _ => None,
        }
    }
    pub fn get_contains_agent(&self) -> Option<bool> {
        match self {
            ResponseFrame::containsAgent { id: _, answer } => Some(answer.clone()),
            _ => None,
        }
    }
}

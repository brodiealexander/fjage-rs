use std::collections::HashMap;

use base64::prelude::*;
use serde_json::{Map, Value};

fn u8_to_f32_vec(v: &[u8]) -> Vec<f32> {
    v.chunks_exact(4)
        .map(TryInto::try_into)
        .map(Result::unwrap)
        .map(f32::from_le_bytes)
        .collect()
}
fn u8_to_i32_vec(v: &[u8]) -> Vec<i32> {
    v.chunks_exact(4)
        .map(TryInto::try_into)
        .map(Result::unwrap)
        .map(i32::from_le_bytes)
        .collect()
}
fn u8_to_f64_vec(v: &[u8]) -> Vec<f64> {
    v.chunks_exact(8)
        .map(TryInto::try_into)
        .map(Result::unwrap)
        .map(f64::from_le_bytes)
        .collect()
}
fn u8_to_i64_vec(v: &[u8]) -> Vec<i64> {
    v.chunks_exact(8)
        .map(TryInto::try_into)
        .map(Result::unwrap)
        .map(i64::from_le_bytes)
        .collect()
}

pub fn base64_to_f32(s: &str) -> Vec<f32> {
    let decoded = BASE64_STANDARD.decode(s).expect("Decode Failure");
    u8_to_f32_vec(&decoded)
}
pub fn base64_to_i32(s: &str) -> Vec<i32> {
    let decoded = BASE64_STANDARD.decode(s).expect("Decode Failure");
    u8_to_i32_vec(&decoded)
}
pub fn base64_to_f64(s: &str) -> Vec<f64> {
    let decoded = BASE64_STANDARD.decode(s).expect("Decode Failure");
    u8_to_f64_vec(&decoded)
}
pub fn base64_to_i64(s: &str) -> Vec<i64> {
    let decoded = BASE64_STANDARD.decode(s).expect("Decode Failure");
    u8_to_i64_vec(&decoded)
}
pub fn base64_to_u8(s: &str) -> Vec<u8> {
    BASE64_STANDARD.decode(s).expect("Decode Failure")
}

fn f32_to_u8_vec(v: &[f32]) -> Vec<u8> {
    let x = v.into_iter();
    let x = x.map(|j| j.to_le_bytes()).flatten();
    x.collect()
}
fn i32_to_u8_vec(v: &[i32]) -> Vec<u8> {
    let x = v.into_iter();
    let x = x.map(|j| j.to_le_bytes()).flatten();
    x.collect()
}
fn f64_to_u8_vec(v: &[f64]) -> Vec<u8> {
    let x = v.into_iter();
    let x = x.map(|j| j.to_le_bytes()).flatten();
    x.collect()
}
fn i64_to_u8_vec(v: &[i64]) -> Vec<u8> {
    let x = v.into_iter();
    let x = x.map(|j| j.to_le_bytes()).flatten();
    x.collect()
}

pub fn base64_from_f32(v: Vec<f32>) -> String {
    BASE64_STANDARD.encode(f32_to_u8_vec(&v))
}
pub fn base64_from_i32(v: Vec<i32>) -> String {
    BASE64_STANDARD.encode(i32_to_u8_vec(&v))
}
pub fn base64_from_f64(v: Vec<f64>) -> String {
    BASE64_STANDARD.encode(f64_to_u8_vec(&v))
}
pub fn base64_from_i64(v: Vec<i64>) -> String {
    BASE64_STANDARD.encode(i64_to_u8_vec(&v))
}

pub fn b64_obj_from_u8(v: Vec<u8>) -> Value {
    let mut obj: Map<String, Value> = Map::<String, Value>::new();
    obj.insert("clazz".to_string(), Value::from("[B"));
    obj.insert("data".to_string(), Value::from(BASE64_STANDARD.encode(&v)));
    return Value::from(obj);
}
pub fn b64_obj_from_i32(v: Vec<i32>) -> Value {
    let mut obj: Map<String, Value> = Map::<String, Value>::new();
    obj.insert("clazz".to_string(), Value::from("[I"));
    obj.insert("data".to_string(), Value::from(base64_from_i32(v)));
    return Value::from(obj);
}
pub fn b64_obj_from_i64(v: Vec<i64>) -> Value {
    let mut obj: Map<String, Value> = Map::<String, Value>::new();
    obj.insert("clazz".to_string(), Value::from("[J"));
    obj.insert("data".to_string(), Value::from(base64_from_i64(v)));
    return Value::from(obj);
}
pub fn b64_obj_from_f32(v: Vec<f32>) -> Value {
    let mut obj: Map<String, Value> = Map::<String, Value>::new();
    obj.insert("clazz".to_string(), Value::from("[F"));
    obj.insert("data".to_string(), Value::from(base64_from_f32(v)));
    return Value::from(obj);
}
pub fn b64_obj_from_f64(v: Vec<f64>) -> Value {
    let mut obj: Map<String, Value> = Map::<String, Value>::new();
    obj.insert("clazz".to_string(), Value::from("[D"));
    obj.insert("data".to_string(), Value::from(base64_from_f64(v)));
    return Value::from(obj);
}

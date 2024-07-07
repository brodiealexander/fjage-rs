use std::ffi::{c_char, c_int, c_long, CStr, CString};
use tokio::runtime::Runtime;

use util::{
    c_api_alloc_cstr, c_api_cstr_to_string, c_api_exec, c_api_exec_timeout_ms, c_api_free_cstr,
    fjage_msg_t,
};

use crate::remote::gateway::Gateway;

pub mod message;
pub mod param;
pub mod util;

pub static mut RUNTIME: Option<Runtime> = None;

//fjage_gw_t fjage_tcp_open(const char *hostname, int port);
#[no_mangle]
pub unsafe extern "C" fn fjage_tcp_open(hostname: *const c_char, port: c_int) -> *mut Gateway {
    RUNTIME = Some(Runtime::new().unwrap());

    let hostname = CStr::from_ptr(hostname);
    let hostname = String::from_utf8_lossy(hostname.to_bytes()).to_string();
    let port: i32 = i32::from(port);
    let gw = Box::new(
        RUNTIME
            .as_mut()
            .unwrap()
            .block_on(async { Gateway::new_tcp(&hostname, port.try_into().unwrap()).await }),
    );
    let val = Box::into_raw(gw);
    return val;
}

/// Open a gateway to a fjåge master container via RS232.
///
/// @param devname        Device name
/// @param baud           Baud rate
/// @param settings       RS232 settings (NULL or "N81")
/// @return               Gateway

//fjage_gw_t fjage_rs232_open(const char *devname, int baud, const char *settings);
#[no_mangle]
pub unsafe extern "C" fn fjage_rs232_open(
    devname: *const c_char,
    baud: c_int,
    settings: *const c_char,
) -> *mut Gateway {
    unimplemented!()
}

/// Wakeup a device running fjåge master container via RS232.
///
/// @param devname        Device name
/// @param baud           Baud rate
/// @param settings       RS232 settings (NULL or "N81")
/// @return               0 on success, error code otherwise

//int fjage_rs232_wakeup(const char *devname, int baud, const char *settings);
#[no_mangle]
pub unsafe extern "C" fn fjage_rs232_wakeup(
    devname: *const c_char,
    baud: c_int,
    settings: *const c_char,
) -> *mut Gateway {
    unimplemented!();
}

/// Close a gateway to a fjåge master container. Once a gateway is closed,
/// the gateway object is invalid and should no longer be used.
///
/// @param gw             Gateway
/// @return               0 on success, error code otherwise

//int fjage_close(fjage_gw_t gw);
#[no_mangle]
pub unsafe extern "C" fn fjage_close(gw: *mut Gateway) -> c_int {
    let mut _gw = Box::from_raw(gw);
    return 0;
}

/// Get the AgentID of the gateway. A gateway appears as a single agent in
/// a fjåge slave container. The AgentID returned by this function should not be
/// freed by the caller.
///
/// @param gw             Gateway
/// @return               The AgentID of the gateway agent

//fjage_aid_t fjage_get_agent_id(fjage_gw_t gw);
#[no_mangle]
pub unsafe extern "C" fn fjage_get_agent_id(gw: *mut Gateway) -> *const c_char {
    let aid = gw.as_mut().unwrap();
    let aid = aid.get_agent_id();
    return c_api_alloc_cstr(aid);
}

/// Subscribe to a topic.
///
/// @param gw             Gateway
/// @param topic          Topic to subscribe to, usually generated using fjage_aid_topic()
/// @return               0 on success, error code otherwise

//int fjage_subscribe(fjage_gw_t gw, const fjage_aid_t topic);
#[no_mangle]
pub unsafe extern "C" fn fjage_subscribe(gw: *mut Gateway, topic: *const c_char) -> c_int {
    RUNTIME
        .as_mut()
        .unwrap()
        .block_on(gw.as_mut().unwrap().subscribe(&c_api_cstr_to_string(topic)));
    return 0;
}

/// Subscribe to an agent's default topic.
///
/// @param gw             Gateway
/// @param topic          AgentID
/// @return               0 on success, error code otherwise

//int fjage_subscribe_agent(fjage_gw_t gw, const fjage_aid_t aid);
#[no_mangle]
pub unsafe extern "C" fn fjage_subscribe_agent(gw: *mut Gateway, aid: *const c_char) -> c_int {
    RUNTIME.as_mut().unwrap().block_on(
        gw.as_mut()
            .unwrap()
            .subscribe_agent(&c_api_cstr_to_string(aid)),
    );
    return 0;
}

/// Unsubscribe from a topic.
///
/// @param gw             Gateway
/// @param topic          Topic to subscribe to, usually generated using fjage_aid_topic()
/// @return               0 on success, error code otherwise

//int fjage_unsubscribe(fjage_gw_t gw, const fjage_aid_t topic);
#[no_mangle]
pub unsafe extern "C" fn fjage_unsubscribe(gw: *mut Gateway, topic: *const c_char) -> c_int {
    RUNTIME.as_mut().unwrap().block_on(
        gw.as_mut()
            .unwrap()
            .unsubscribe(&c_api_cstr_to_string(topic)),
    );
    return 0;
}

/// Check if a topic is subscribed to.
///
/// @param gw             Gateway
/// @param topic          Topic to check, usually generated using fjage_aid_topic()
/// @return               true if subscribed to, false otherwise

//bool fjage_is_subscribed(fjage_gw_t gw, const fjage_aid_t topic);
#[no_mangle]
pub unsafe extern "C" fn fjage_is_subscribed(gw: *mut Gateway, topic: *const c_char) -> bool {
    return RUNTIME.as_mut().unwrap().block_on(
        gw.as_mut()
            .unwrap()
            .is_subscribed(&c_api_cstr_to_string(topic)),
    );
}

/// Find an agent providing a specified service. The AgentID returned by this function
/// should be freed by the caller using fjage_aid_destroy().
///
/// @param gw             Gateway
/// @param service        Fully qualified name of a service
/// @return               AgentID of an agent providing the service, NULL if none found

//fjage_aid_t fjage_agent_for_service(fjage_gw_t gw, const char *service);
#[no_mangle]
pub unsafe extern "C" fn fjage_agent_for_service(
    gw: *mut Gateway,
    service: *const c_char,
) -> *const c_char {
    let result = c_api_exec(
        gw.as_mut()
            .unwrap()
            .agent_for_service(&c_api_cstr_to_string(service)),
    );
    if result.is_none() {
        return std::ptr::null(); // C will recognize this as returning NULL (I hope)
    }
    return c_api_alloc_cstr(result.unwrap());
}

/// Find all agents providing a specified service. The list of agents is populated in an
/// array provided by the caller. If only the number of agents is desired, a NULL may be
/// passed in instead of an array, and max can be set to 0. All AgentIDs returned by this
/// function should be freed by the caller using fjage_aid_destroy().
///
/// @param gw             Gateway
/// @param service        Fully qualified name of a service
/// @param agents         An array of AgentIDs for the function to fill, or NULL
/// @param max            Size of the agents array, or 0 if agents is NULL
/// @return               Number of agents providing the service

//int fjage_agents_for_service(fjage_gw_t gw, const char *service, fjage_aid_t *agents, int max);
#[no_mangle]
pub unsafe extern "C" fn fjage_agents_for_service(
    gw: *mut Gateway,
    service: *const c_char,
    agents: *mut *mut c_char,
    max: c_int,
) -> c_int {
    let result = c_api_exec(
        gw.as_mut()
            .unwrap()
            .agents_for_service(&c_api_cstr_to_string(service)),
    );
    if result.is_empty() {
        return 0;
    }

    for n in 0..std::cmp::min(max, result.len().try_into().unwrap()) {
        let str = result.get(n as usize).unwrap();
        if agents.is_null() {
            return result.len() as c_int;
        }

        let curr_aid_ptr = agents;

        let curr_aid_ptr = curr_aid_ptr.add((n) as usize);

        (*curr_aid_ptr) = c_api_alloc_cstr(String::from(str)).cast_mut();
    }

    return result.len() as c_int;
}

/// Send a message. The message object passed in is considered consumed after this call,
/// and should not be used or freed by the caller.
///
/// @param gw             Gateway
/// @param msg            Message to send
/// @return               0 on success, error code otherwise

//int fjage_send(fjage_gw_t gw, const fjage_msg_t msg);
#[no_mangle]
pub unsafe extern "C" fn fjage_send(gw: *mut Gateway, msg: *mut fjage_msg_t) -> c_int {
    fjage_msg_t::send(gw, msg); //auto-frees

    return 0 as c_int;
}

/// Receive a message. The received message should be freed by the caller using fjage_msg_destroy().
/// If clazz is not NULL, only the first message of a specified message class is received. If id is not
/// NULL, only the first message that is in response to the message specified by the id is received.
///
/// Received message are open in read-only mode, where the getter fjage_msg_get_* functions may
/// be called, but not the setters. Messages of class org.arl.fjage.GenericMessage are currently
/// unsupported.
///
/// @param gw             Gateway
/// @param clazz          Fully qualified name of message class, or NULL
/// @param id             MessageID of the message being responded to, or NULL
/// @param timeout        Timeout in milliseconds
/// @return               The received message in read-only mode, or NULL on timeout

//fjage_msg_t fjage_receive(fjage_gw_t gw, const char *clazz, const char *id, long timeout);
#[no_mangle]
pub unsafe extern "C" fn fjage_receive(
    gw: *mut Gateway,
    clazz: *const c_char,
    id: *const c_char,
    timeout: c_long,
) -> *const fjage_msg_t {
    let msg = c_api_exec_timeout_ms(
        gw.as_mut().unwrap().recv(
            if clazz.is_null() {
                None
            } else {
                Some(vec![c_api_cstr_to_string(clazz)])
            },
            if id.is_null() {
                None
            } else {
                Some(c_api_cstr_to_string(id))
            },
        ),
        timeout as i32,
    );

    if msg.is_ok() {
        let msg = msg.unwrap();
        if msg.is_none() {
            return std::ptr::null();
        }
        let boxed_msg = fjage_msg_t::alloc();
        boxed_msg.as_mut().unwrap().msg = msg.unwrap();
        return boxed_msg;
    } else {
        return std::ptr::null();
    }
}

/// Receive any message whose name is contained in the array 'clazzes'. The first message whose name
/// matches any of the message names in the array will be returned. If the array is NULL or it's
/// length is less than 1, no message will be returned. The received message should be freed by
/// the caller using fjage_msg_destroy().
///
///
/// Received message is open in read-only mode, where the getter fjage_msg_get_* functions may
/// be called, but not the setters. Messages of class org.arl.fjage.GenericMessage are currently
/// unsupported.
///
/// @param gw             Gateway
/// @param clazzes        An array of fully qualified name of message class.
/// @param clazzlen       Length of the array of fully qualified name of message class.
/// @param timeout        Timeout in milliseconds
/// @return               The received message in read-only mode, or NULL on timeout

//fjage_msg_t fjage_receive_any(fjage_gw_t gw, const char **clazzes, int clazzlen, long timeout);
#[no_mangle]
pub unsafe extern "C" fn fjage_receive_any(
    gw: *mut Gateway,
    clazzes: *const *const c_char,
    clazzlen: c_int,
    timeout: c_long,
) -> *const fjage_msg_t {
    let mut clazzlist: Vec<String> = Vec::new();
    for i in 0..clazzlen {
        clazzlist.push(c_api_cstr_to_string(*clazzes.add(i as usize)));
    }

    let msg = c_api_exec_timeout_ms(
        gw.as_mut().unwrap().recv(Some(clazzlist), None),
        timeout as i32,
    );

    if msg.is_ok() {
        let msg = msg.unwrap();
        if msg.is_none() {
            return std::ptr::null();
        }
        let boxed_msg = fjage_msg_t::alloc();
        boxed_msg.as_mut().unwrap().msg = msg.unwrap();
        return boxed_msg;
    } else {
        return std::ptr::null();
    }
}

/// Send a message and wait for a response. The message object passed in is considered consumed
/// after the call, and should not be used or freed by the caller. If a response is returned,
/// it should be freed by the caller using fjage_msg_destroy().
///
/// Received messages are open in read-only mode, where the getter fjage_msg_get_* functions may
/// be called, but not the setters. Messages of class org.arl.fjage.GenericMessage are currently
/// unsupported.
///
/// @param gw             Gateway
/// @param request        Request message to send
/// @param timeout        Timeout in milliseconds
/// @return               Response message in read-only mode, or NULL on timeout

//fjage_msg_t fjage_request(fjage_gw_t gw, const fjage_msg_t request, long timeout);
#[no_mangle]
pub unsafe extern "C" fn fjage_request(
    gw: *mut Gateway,
    request: *mut fjage_msg_t,
    timeout: c_long,
) -> *const fjage_msg_t {
    return fjage_msg_t::request(gw, request, timeout);
}

/// Abort a fjage_receive() or fjage_request() operation before the timeout. This function may be
/// called from another thread to abort an ongoing blocking reception.
///
/// @param gw             Gateway
/// @return               0 on success, error code otherwise

//int fjage_interrupt(fjage_gw_t gw);
#[no_mangle]
pub unsafe extern "C" fn fjage_interrupt(gw: *mut Gateway) -> c_int {
    gw.as_mut().unwrap().interrupt();
    return 0;
}

/// Create an AgentID. The AgentID created using this function should be freed using
/// fjage_aid_destroy().
///
/// @param name           Name of the agent
/// @return               AgentID

//fjage_aid_t fjage_aid_create(const char *name);
#[no_mangle]
pub unsafe extern "C" fn fjage_aid_create(name: *const c_char) -> *const c_char {
    return c_api_alloc_cstr(c_api_cstr_to_string(name));
}

/// Create an topic. The topic AgentID created using this function should be freed using
/// fjage_aid_destroy().
///
/// @param name           Name of the topic
/// @return               AgentID for the specified topic

//fjage_aid_t fjage_aid_topic(const char *topic);
#[no_mangle]
pub unsafe extern "C" fn fjage_aid_topic(topic: *const c_char) -> *const c_char {
    let mut aid_topic = String::from("#");
    aid_topic.push_str(&c_api_cstr_to_string(topic));
    return c_api_alloc_cstr(aid_topic);
}

/// Destroy an AgentID. Once destroyed, the AgentID is considered invalid and should no
/// longer be used.
///
/// @param aid            AgentID to destroy

//void fjage_aid_destroy(fjage_aid_t aid);
#[no_mangle]
pub unsafe extern "C" fn fjage_aid_destroy(aid: *mut c_char) {
    c_api_free_cstr(aid);
}

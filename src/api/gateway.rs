use std::{sync::Arc, time::Duration};

use crate::{
    core::{
        message::Message,
        param::{ParameterManipulation, ParameterReq, ParameterRsp},
    },
    remote::container::*,
};
use tokio::runtime::Runtime;

pub struct Gateway {
    container: RemoteContainer,
    runtime: Arc<Box<Runtime>>,
}
impl Gateway {
    pub fn new_tcp(hostname: &str, port: u16) -> Gateway {
        let runtime = Box::new(Runtime::new().unwrap());
        let gw = runtime.block_on(RemoteContainer::new_tcp(hostname, port));
        return Gateway {
            container: gw,
            runtime: Arc::new(runtime),
        };
    }
    pub fn is_subscribed(&self, aid: &str) -> bool {
        return self.runtime.block_on(self.container.is_subscribed(aid));
    }
    pub fn subscribe(&mut self, aid: &str) {
        return self.runtime.block_on(self.container.subscribe(aid));
    }
    pub fn unsubscribe(&mut self, aid: &str) {
        return self.runtime.block_on(self.container.unsubscribe(aid));
    }
    // Move to a separate trait related to local agent representation?
    pub fn add_agent(&mut self, aid: &str) {
        return self.runtime.block_on(self.container.add_agent(aid));
    }
    pub fn subscribe_agent(&mut self, aid: &str) {
        return self.runtime.block_on(self.container.subscribe_agent(aid));
    }
    pub fn unsubscribe_agent(&mut self, aid: &str) {
        return self.runtime.block_on(self.container.unsubscribe_agent(aid));
    }
    pub fn get_agent_id(&self) -> String {
        return self.container.get_agent_id();
    }
    /// Get list of agents running in fjåge
    pub fn agents(&mut self) -> Vec<String> {
        return self.runtime.block_on(self.container.agents());
    }
    /// Get list of services running in fjåge
    pub fn services(&mut self) -> Vec<String> {
        return self.runtime.block_on(self.container.services());
    }
    /// Ask if upstream container contains a specific agent
    pub fn contains_agent(&mut self, aid: &str) -> bool {
        return self.runtime.block_on(self.container.contains_agent(aid));
    }
    /// Find an agent which advertises the requested service
    pub fn agent_for_service(&mut self, service: &str) -> Option<String> {
        return self
            .runtime
            .block_on(self.container.agent_for_service(service));
    }
    /// Find all agents which advertise the requested service
    pub fn agents_for_service(&mut self, service: &str) -> Vec<String> {
        return self
            .runtime
            .block_on(self.container.agents_for_service(service));
    }

    /// Send a message to the specified agent or topic. If "sender" is empty, it will be filled with the AgentID of the Gateway
    pub fn send(&mut self, to: &str, msg: Message) {
        return self.container.send(to, msg);
    }
    /// Send a message to the specified agent, then waits for a message with an inReplyTo marker matching the sent message's UUID.
    pub fn request(&mut self, to: &str, msg: Message) -> Option<Message> {
        return self.runtime.block_on(self.container.request(to, msg));
    }
    pub fn request_timeout(
        &mut self,
        to: &str,
        msg: Message,
        timeout: Duration,
    ) -> Option<Message> {
        return self
            .runtime
            .block_on(async {
                tokio::time::timeout(timeout, self.container.request(to, msg)).await
            })
            .unwrap_or(None);
    }
    /// Receive a message. If clazzes, id, or both are specified, then only messages matching those parameters will be returned.
    pub fn recv(&mut self, clazzes: Option<Vec<String>>, id: Option<String>) -> Option<Message> {
        return self.runtime.block_on(self.container.recv(clazzes, id));
    }
    pub fn recv_timeout(
        &mut self,
        clazzes: Option<Vec<String>>,
        id: Option<String>,
        timeout: Duration,
    ) -> Option<Message> {
        return self
            .runtime
            .block_on(async {
                tokio::time::timeout(timeout, self.container.recv(clazzes, id)).await
            })
            .unwrap_or(None);
    }
    /// Interrupt an ongoing reception
    pub fn interrupt(&mut self) {
        self.container.interrupt();
    }
}
impl ParameterManipulation for Gateway {
    /// Send a [ParameterReq](https://org-arl.github.io/fjage/javadoc/org/arl/fjage/param/ParameterReq.html) message to an agent in the upstream container and return the [ParameterRsp]((https://org-arl.github.io/fjage/javadoc/org/arl/fjage/param/ParameterRsp.html)).
    fn param_req(&mut self, aid: &str, mut req: ParameterReq) -> Option<ParameterRsp> {
        let rsp = self.request(aid, req.to_msg());
        if rsp.is_none() {
            return None;
        }
        let rsp = ParameterRsp::from_msg(rsp.unwrap());
        return Some(rsp);
    }

    fn param_req_timeout(
        &mut self,
        aid: &str,
        mut req: ParameterReq,
        timeout: Duration,
    ) -> Option<ParameterRsp> {
        let rsp = self.request_timeout(aid, req.to_msg(), timeout);
        if rsp.is_none() {
            return None;
        }
        let rsp = ParameterRsp::from_msg(rsp.unwrap());
        return Some(rsp);
    }
}

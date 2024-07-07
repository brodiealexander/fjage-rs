use std::sync::Arc;

use tokio::sync::{broadcast, Mutex};

use crate::core::{
    agent::{Agent, AgentID},
    //behavior::BehaviorExecutor,
    container::Container,
    message::Message,
};

pub struct EchoAgent {
    mailbox: Option<broadcast::Sender<Message>>,
    owner: Option<Container>,
}
impl EchoAgent {
    pub fn new() -> AgentID {
        return AgentID::new(EchoAgent {
            mailbox: None,
            owner: None,
        });
    }
}
impl Agent for EchoAgent {
    fn init(&mut self, owner: Container) {
        self.mailbox = Some(owner.get_broadcast_channel());
        self.owner = Some(owner);
        //BehaviorExecutor::
    }

    /*fn get_mailbox(&mut self) -> tokio::sync::broadcast::Sender<crate::core::message::Message> {
        if self.mailbox.is_some() {
            return self.mailbox.as_ref().unwrap().clone();
        } else {
            panic!("No mailbox defined for agent! Did you add it to a container?");
        }
    }*/

    //fn get_filter(&self) -> Box<dyn Fn(&Message) -> bool> {
    //    return Box::new(|msg| msg.data.inReplyTo.is_some());
    //}

    fn on_message(&mut self, msg: Message) {
        println!("ECHO AGENT: {:?}", msg);
    }

    fn get_name(&self) -> String {
        todo!()
    }

    fn get_clazz(&self) -> String {
        todo!()
    }
}

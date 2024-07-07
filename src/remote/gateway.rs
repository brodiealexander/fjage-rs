use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};
use tokio::sync::{broadcast, Mutex};
use uuid::Uuid;

//use crate::core::agent::AgentID;
//use crate::core::container::Container;
use crate::core::message::Message;
use crate::core::param::{ParameterManipulation, ParameterReq, ParameterRsp};
use crate::protocol::connector::TcpConnector;
use crate::protocol::frame::Frame;

use crate::protocol::{connector::Connector, frame::*};

#[derive(Clone)]
pub enum GatewayReceiveInterrupt {
    MESSAGE,
    CANCEL,
}

#[derive(Clone)]
pub struct Gateway {
    sender: UnboundedSender<Frame>,
    rsp_frame_broadcast: broadcast::Sender<ResponseFrame>,
    agents: Arc<Mutex<Vec<String>>>,
    subscriptions: Arc<Mutex<Vec<String>>>,
    agent_id: String,
    msg_queue: Arc<Mutex<Vec<Message>>>, //parent: Box<Container>,
    msg_interrupt_listener: Arc<Mutex<mpsc::Receiver<GatewayReceiveInterrupt>>>,
    msg_interrupt_sender: mpsc::Sender<GatewayReceiveInterrupt>,
}
impl Gateway {
    pub async fn new_tcp(hostname: &str, port: u16) -> Gateway {
        let conn = TcpConnector::new(hostname, port);
        return Gateway::new(&conn).await;
    }
    pub async fn new<T: Connector>(connector: &T) -> Gateway {
        let (tx, rx): (UnboundedSender<Frame>, UnboundedReceiver<Frame>) =
            connector.connect().await;
        let (rsp_frame_broadcast, _rsp_frame_listen): (
            broadcast::Sender<ResponseFrame>,
            broadcast::Receiver<ResponseFrame>,
        ) = broadcast::channel(64);

        let (tx_interrupt, rx_interrupt) = mpsc::channel::<GatewayReceiveInterrupt>(64);

        let mut agent_id = String::from("RustGW-");
        agent_id.push_str(&Uuid::new_v4().to_string());

        let mut gateway = Gateway {
            agent_id: agent_id.clone(),
            sender: tx.clone(),
            rsp_frame_broadcast: rsp_frame_broadcast.clone(),
            agents: Arc::new(Mutex::new(Vec::new())),
            subscriptions: Arc::new(Mutex::new(Vec::new())),
            msg_interrupt_listener: Arc::new(Mutex::new(rx_interrupt)),
            msg_queue: Arc::new(Mutex::new(Vec::new())),
            msg_interrupt_sender: tx_interrupt,
        };
        gateway.add_agent(&agent_id).await;
        gateway.receive_task(rx);
        gateway.subscribe(&agent_id).await;

        return gateway;
    }

    async fn update_watch(&mut self) {
        let subs = self.subscriptions.lock().await;
        self.sender
            .send(Frame::Request(RequestFrame::wantsMessagesFor {
                agentIDs: subs.clone(),
            }))
            .unwrap();
    }

    pub async fn is_subscribed(&self, aid: &str) -> bool {
        return self.subscriptions.lock().await.contains(&aid.to_string());
    }
    pub async fn subscribe(&mut self, aid: &str) {
        self.subscriptions.lock().await.push(aid.to_string());
        self.update_watch().await;
    }
    pub async fn unsubscribe(&mut self, aid: &str) {
        self.subscriptions.lock().await.retain(|x| *x != aid);
        self.update_watch().await;
    }
    pub async fn add_agent(&mut self, agent_id: &str) {
        self.agents.lock().await.push(agent_id.to_string());
    }
    pub async fn subscribe_agent(&mut self, aid: &str) {
        let mut topic_aid = String::from("#");
        topic_aid.push_str(aid);
        topic_aid.push_str("__ntf");
        self.subscribe(&topic_aid).await;
    }
    pub async fn unsubscribe_agent(&mut self, aid: &str) {
        let mut topic_aid = String::from("#");
        topic_aid.push_str(aid);
        topic_aid.push_str("__ntf");
        self.unsubscribe(&topic_aid).await;
    }
    fn receive_task(&mut self, mut receiver: UnboundedReceiver<Frame>) {
        let mut container = self.clone();
        tokio::spawn(async move {
            container.sender.send(Frame::Alive(true)).unwrap();
            loop {
                let frame = receiver.recv().await.unwrap();
                let response = match frame {
                    Frame::Alive(_) => None,
                    Frame::Request(req) => container.process_request(req).await,
                    Frame::Response(rsp) => {
                        container.rsp_frame_broadcast.send(rsp).unwrap();
                        None
                    }
                };
                if response.is_some() {
                    container
                        .sender
                        .send(Frame::Response(response.unwrap()))
                        .unwrap();
                }
            }
        });
    }
    async fn query(&mut self, frame: Frame) -> ResponseFrame {
        let mut listener = self.rsp_frame_broadcast.subscribe();
        let id = frame.id().unwrap().clone();
        self.sender.send(frame).unwrap();
        loop {
            let msg = listener.recv().await.unwrap();
            if msg.id() == &id {
                return msg;
            }
        }
    }
    pub fn get_agent_id(&self) -> String {
        return self.agent_id.clone();
    }

    pub async fn agents(&mut self) -> Vec<String> {
        let id = Uuid::new_v4().to_string();
        let rsp = self
            .query(Frame::Request(RequestFrame::agents { id: id.clone() }))
            .await
            .get_agentIDs();
        return rsp.unwrap();
    }
    pub async fn services(&mut self) -> Vec<String> {
        let rsp = self
            .query(Frame::Request(RequestFrame::services {
                id: Uuid::new_v4().to_string(),
            }))
            .await;
        return rsp.get_services().unwrap();
    }
    pub async fn contains_agent(&mut self, agent_id: &str) -> bool {
        let rsp = self
            .query(Frame::Request(RequestFrame::containsAgent {
                id: Uuid::new_v4().to_string(),
                agentID: agent_id.to_string(),
            }))
            .await;
        return rsp.get_contains_agent().unwrap();
    }
    pub async fn agent_for_service(&mut self, service: &str) -> Option<String> {
        let rsp = self
            .query(Frame::Request(RequestFrame::agentForService {
                id: Uuid::new_v4().to_string(),
                service: service.to_string(),
            }))
            .await;
        return rsp.get_agentID();
    }
    pub async fn agents_for_service(&mut self, service: &str) -> Vec<String> {
        let rsp = self
            .query(Frame::Request(RequestFrame::agentsForService {
                id: Uuid::new_v4().to_string(),
                service: service.to_string(),
            }))
            .await;
        return rsp.get_agentIDs().unwrap();
    }
    pub fn send_raw(&mut self, mut msg: Message) {
        if msg.data.sender.is_empty() {
            msg.data.sender = self.agent_id.clone();
        }
        msg.data.sentAt = Some(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as i64,
        );
        self.sender
            .send(Frame::Request(RequestFrame::send {
                message: msg,
                relay: true,
            }))
            .unwrap();
    }
    pub fn send(&mut self, to: &str, mut msg: Message) {
        msg.data.sender = self.agent_id.clone();
        msg.data.recipient = to.to_string();
        msg.data.sentAt = Some(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as i64,
        );
        self.sender
            .send(Frame::Request(RequestFrame::send {
                message: msg,
                relay: true,
            }))
            .unwrap();
    }
    pub async fn request(&mut self, to: &str, mut msg: Message) -> Option<Message> {
        let id = msg.data.msgID.clone();
        msg.data.sentAt = Some(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as i64,
        );
        self.clear_interrupt().await;
        self.send(to, msg);

        loop {
            let msg = {
                let mut msg = None;
                let mut msg_pos = None;
                let mut queue = self.msg_queue.lock().await;
                for (pos, msg) in queue.iter().enumerate() {
                    if msg
                        .data
                        .inReplyTo
                        .as_ref()
                        .is_some_and(|in_reply_to| in_reply_to == &id)
                    {
                        msg_pos = Some(pos);
                    }
                }
                if msg_pos.is_some() {
                    msg = Some(queue.remove(msg_pos.unwrap()));
                }
                msg
            };
            if msg.is_some() {
                return Some(msg.unwrap());
            }

            let int = self
                .msg_interrupt_listener
                .lock()
                .await
                .recv()
                .await
                .unwrap();

            match int {
                GatewayReceiveInterrupt::CANCEL => return None,
                _ => (),
            };
        }
    }

    pub async fn clear_interrupt(&mut self) {
        while self.msg_interrupt_listener.lock().await.try_recv().is_ok() {}
    }

    pub async fn recv(&mut self) -> Option<Message> {
        self.clear_interrupt().await;
        loop {
            let msg = {
                let mut queue = self.msg_queue.lock().await;
                if queue.is_empty() {
                    None
                } else {
                    Some(queue.pop().unwrap())
                }
            };
            if msg.is_some() {
                return Some(msg.unwrap());
            }
            let int = self
                .msg_interrupt_listener
                .lock()
                .await
                .recv()
                .await
                .unwrap();

            match int {
                GatewayReceiveInterrupt::CANCEL => return None,
                _ => (),
            };
        }
    }
    pub async fn recv_any(&mut self, clazzes: Vec<String>) -> Option<Message> {
        self.clear_interrupt().await;
        loop {
            let msg = {
                let mut msg = None;
                let mut msg_pos = None;
                let mut queue = self.msg_queue.lock().await;
                for (pos, msg) in queue.iter().enumerate() {
                    if clazzes.contains(&msg.clazz) {
                        msg_pos = Some(pos);
                    }
                }
                if msg_pos.is_some() {
                    msg = Some(queue.remove(msg_pos.unwrap()));
                }
                msg
            };
            if msg.is_some() {
                return Some(msg.unwrap());
            }
            let int = self
                .msg_interrupt_listener
                .lock()
                .await
                .recv()
                .await
                .unwrap();

            match int {
                GatewayReceiveInterrupt::CANCEL => return None,
                _ => (),
            };
        }
    }

    pub fn interrupt(&mut self) {
        self.msg_interrupt_sender
            .blocking_send(GatewayReceiveInterrupt::CANCEL)
            .unwrap();
    }
    pub async fn process_request(&mut self, req: RequestFrame) -> Option<ResponseFrame> {
        return match req {
            RequestFrame::agents { id } => Some(ResponseFrame::agents {
                id: id,
                agentIDs: self.agents.lock().await.clone(),
            }),
            RequestFrame::containsAgent { id, agentID } => Some(ResponseFrame::containsAgent {
                id: id,
                answer: self.agents.lock().await.contains(&agentID),
            }),
            RequestFrame::services { id } => Some(ResponseFrame::services {
                // TODO: investigate possibility of advertising services
                id: id,
                services: Vec::new(),
            }),
            RequestFrame::agentForService { id, service: _ } => {
                Some(ResponseFrame::agentForService {
                    // This will always respond "None" because we have not implemented this functionality
                    id: id,
                    agentID: None,
                })
            }
            RequestFrame::agentsForService { id, service: _ } => {
                Some(ResponseFrame::agentsForService {
                    // This will always respond with an empty vector because we have not implemented this functionality
                    id: id,
                    agentIDs: Vec::new(),
                })
            }
            RequestFrame::send {
                mut message,
                relay: _,
            } => {
                message.decode_java_classes();
                let mut queue = self.msg_queue.lock().await;
                queue.push(message);
                self.msg_interrupt_sender
                    .send(GatewayReceiveInterrupt::MESSAGE)
                    .await
                    .unwrap();

                None
            }
            RequestFrame::wantsMessagesFor { agentIDs: _ } => None,
        };
    }
}
impl ParameterManipulation for Gateway {
    async fn param_req(&mut self, aid: &str, mut req: ParameterReq) -> Option<ParameterRsp> {
        let rsp = self.request(aid, req.to_msg()).await;
        if rsp.is_none() {
            return None;
        }
        let rsp = ParameterRsp::from_msg(rsp.unwrap());
        return Some(rsp);
    }
}

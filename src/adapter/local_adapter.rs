use std::collections::HashMap;
use actix::{Actor, Addr, Message, Recipient};
use actix::dev::channel::AddressSender;
use actix_web::Handler;
use serde_json::json;
use crate::log::Log;
use crate::message::PusherApiMessage;
use crate::namespace::{Namespace, BroadcastMessage, Channel};
use crate::WS;
use crate::ws_message::{OnPusherMessage};

pub struct LocalAdapter {
    pub namespaces: HashMap<String, Addr<Namespace>>,
}

impl Actor for LocalAdapter {
    type Context = actix::Context<Self>;
    
    fn started(&mut self, _: &mut Self::Context) {
        println!("LocalAdapter started");
        let namespace1 = Namespace {
            channels: HashMap::new(),
            users: HashMap::new(),
            app_id: "app1".to_string(),
            sockets: HashMap::new(),
        }.start();
        let namespace2 = Namespace {
            channels: HashMap::new(),
            users: HashMap::new(),
            app_id: "app2".to_string(),
            sockets: HashMap::new(),
        }.start();
        self.namespaces.insert("app1".to_string(), namespace1);
        self.namespaces.insert("app2".to_string(), namespace2);
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct AddSocket {
    pub(crate) app_id: String,
    pub(crate) socket_id: String,
    pub(crate) socket_addr: Addr<WS>,
}

impl actix::Handler<AddSocket> for LocalAdapter {
    type Result = ();

    fn handle(&mut self, msg: AddSocket, _: &mut Self::Context) {
        Log::websocket_title(format!("Adding socket {} to app {}", msg.socket_id, msg.app_id).as_str());
        self.namespaces.get(&msg.app_id.clone()).unwrap().do_send(crate::namespace::AddSocket {
            socket_id: msg.socket_id.clone(),
            socket_addr: msg.socket_addr.clone(),
        });
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct SendMessage {
    pub(crate) app_id: String,
    pub(crate) message: PusherApiMessage,
}

impl actix::Handler<SendMessage> for LocalAdapter {
    type Result = ();

    fn handle(&mut self, msg: SendMessage, _: &mut Self::Context) {
        let namespace = self.namespaces.get(&msg.app_id.clone()).unwrap();
        Log::websocket_title(format!("Sending message to app {}", msg.app_id).as_str());
        for ch in msg.message.channels.unwrap() {
            let msg = PusherApiMessage {
                name: msg.message.name.clone(),
                data: msg.message.data.clone(),
                channel: Some(ch.clone()),
                channels: Some(vec![ch.clone()]),
                socket_id: None,
                info: None,
            };
            println!("Broadcasting message: {:?}", msg);
            namespace.do_send(BroadcastMessage(msg));
        }
    }
}

#[derive(Message)]
#[rtype(result = "Addr<Namespace>")]
pub struct GetNamespace {
    pub(crate) app_id: String,
}

impl actix::Handler<GetNamespace> for LocalAdapter {
    type Result = Addr<Namespace>;

    fn handle(&mut self, msg: GetNamespace, _: &mut Self::Context) -> Self::Result {
        self.namespaces.get(&msg.app_id.clone()).unwrap().clone()
    }
}


#[derive(Message)]
#[rtype(result = "()")]
pub struct AddToChannel {
    pub(crate) app_id: String,
    pub(crate) channel: String,
    pub(crate) socket_id: String,
}

impl actix::Handler<AddToChannel> for LocalAdapter {
    type Result = ();

    fn handle(&mut self, msg: AddToChannel, _: &mut Self::Context) {
        Log::websocket_title(format!("Adding socket {} to channel {} in app {}", msg.socket_id, msg.channel, msg.app_id).as_str());
        self.namespaces.get(&msg.app_id.clone()).unwrap().do_send(crate::namespace::AddToChannel {
            socket_id: msg.socket_id.clone(),
            channel: msg.channel.clone(),
        });
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct RemoveFromChannel {
    pub(crate) app_id: String,
    pub(crate) channel: String,
    pub(crate) socket_id: String,
}

impl actix::Handler<RemoveFromChannel> for LocalAdapter {
    type Result = ();

    fn handle(&mut self, msg: RemoveFromChannel, _: &mut Self::Context) {
        Log::websocket_title(format!("Removing socket {} from channel {} in app {}", msg.socket_id, msg.channel, msg.app_id).as_str());
        self.namespaces.get(&msg.app_id.clone()).unwrap().do_send(crate::namespace::RemoveFromChannel {
            socket_id: msg.socket_id.clone(),
            channel: Channel::Ch(msg.channel.clone()),
        });
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct RemoveSocket {
    pub(crate) app_id: String,
    pub(crate) socket_id: String,
    pub(crate) socket_addr: Addr<WS>,
}

impl actix::Handler<RemoveSocket> for LocalAdapter {
    type Result = ();

    fn handle(&mut self, msg: RemoveSocket, _: &mut Self::Context) {
        Log::websocket_title(format!("Removing socket {} from app {}", msg.socket_id, msg.app_id).as_str());
        self.namespaces.get(&msg.app_id.clone()).unwrap().do_send(crate::namespace::RemoveSocket {
            socket_id: msg.socket_id.clone(),
            socket_addr: msg.socket_addr.clone()
        });
    }
}
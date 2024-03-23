use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use actix::{Actor, ActorFutureExt, Addr, AsyncContext, Context, Handler, Message, WrapFuture};
use actix::dev::channel::AddressSender;
use serde_json::json;
use crate::log::Log;
use crate::message::PusherApiMessage;
use crate::WS;
use crate::ws_message::{OnMessage, OnPusherMessage};

pub struct Namespace {
    pub channels: HashMap<String, HashSet<String>>,
    pub users: HashMap<String, HashSet<String>>,
    pub app_id: String,
    pub sockets: HashMap<String, Addr<WS>>,
}

impl Actor for Namespace {
    type Context = actix::Context<Self>;
    fn started(&mut self, ctx: &mut Self::Context) {
        Log::websocket_title(format!("Namespace for app {}", self.app_id).as_str());
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        Log::websocket_title(format!("Namespace for app {} stopped", self.app_id).as_str());
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct AddSocket {
    pub(crate) socket_id: String,
    pub(crate) socket_addr: Addr<WS>,
}

impl Handler<AddSocket> for Namespace {
    type Result = ();

    fn handle(&mut self, msg: AddSocket, _: &mut Self::Context) {
        // Add the socket to the hashmap
        self.sockets.insert(msg.socket_id.clone(), msg.socket_addr);
    }
}

#[derive(Message)]
#[rtype(result = "usize")]
pub(crate) struct AddToChannel {
    pub(crate) socket_id: String,
    pub(crate) channel: String,
}

impl Handler<AddToChannel> for Namespace {
    type Result = usize;

    fn handle(&mut self, msg: AddToChannel, _ctx: &mut Self::Context) -> Self::Result {
        let socket_id = msg.socket_id.clone();
        let channel = msg.channel.clone();
        self.channels.entry(channel.clone()).or_default().insert(socket_id.clone());
        self.channels.get(&channel).unwrap().len()
    }
}

pub enum Channel {
    Ch(String),
    Vec(Vec<String>),
}

#[derive(Message)]
#[rtype(result = "usize")]
pub struct RemoveFromChannel {
    pub socket_id: String,
    pub channel: Channel,
}

impl Handler<RemoveFromChannel> for Namespace {
    type Result = usize;

    fn handle(&mut self, msg: RemoveFromChannel, _ctx: &mut Self::Context) -> Self::Result {
        match msg.channel {
            Channel::Ch(channel) => {
                self.channels.entry(channel.clone()).or_default().remove(&msg.socket_id);
                self.channels.get(&channel).unwrap().len()
            }
            Channel::Vec(channels) => {
                for channel in channels {
                    self.channels.entry(channel.clone()).or_default().remove(&msg.socket_id);
                }
                self.channels.values().map(|x| x.len()).sum()
            }
        }
    }
}

#[derive(Message)]
#[rtype(result = "Vec<String>")]
pub struct GetSockets;

impl Handler<GetSockets> for Namespace {
    type Result = Vec<String>;

    fn handle(&mut self, _msg: GetSockets, _ctx: &mut Self::Context) -> Self::Result {
        self.sockets.keys().cloned().collect()
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct BroadcastMessage(pub PusherApiMessage); // Message to be broadcasted

impl Handler<BroadcastMessage> for Namespace {
    type Result = ();

    fn handle(&mut self, msg: BroadcastMessage, _: &mut Self::Context) {
        let message = json!({
            "data": msg.0.data,
            "channel": msg.0.channel,
            "event": msg.0.name,
        });
        for socket_addr in self.sockets.values() {
            socket_addr.do_send(OnPusherMessage {
                message: message.clone(),
            });
        }
    }
}

#[derive(Message)]
#[rtype(result = "usize")]
pub struct RemoveSocket {
    pub(crate) socket_id: String,
    pub socket_addr: Addr<WS>,
}

impl Handler<RemoveSocket> for Namespace {
    type Result = usize;

    fn handle(&mut self, msg: RemoveSocket, _: &mut Self::Context) -> Self::Result {
        self.sockets.remove(&msg.socket_id);
        self.channels.values_mut().for_each(|x| {
            x.remove(&msg.socket_id);
        });
        self.sockets.len()
    }
}

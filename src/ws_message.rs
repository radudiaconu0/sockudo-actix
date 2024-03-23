use actix::{Actor, AsyncContext, Context, Handler, Message};
use serde_json::Value;
use crate::log::Log;
use crate::message::{PusherApiMessage, PusherMessage};
use crate::{utils, WS};
use crate::adapter::local_adapter::{AddSocket, AddToChannel};


#[derive(Message, Debug)]
#[rtype(result = "()")]
pub struct OnMessage {
    pub(crate) message: PusherMessage,
}

impl Handler<OnMessage> for WS {
    type Result = ();

    fn handle(&mut self, msg: OnMessage, ctx: &mut Self::Context) -> Self::Result {
        let message = msg.message.clone();
        match msg.message.data {
            Some(data) => {
                Log::websocket_title(format!("Received message: {:?}", data).as_str());
            }
            None => {
                Log::websocket_title("Received message without data");
            }
        }
        match msg.message.event.as_str() {
            "pusher:ping" => {
                let pong = PusherMessage {
                    event: "pusher:pong".to_string(),
                    data: None,
                    channel: None,
                    name: None,
                };
                ctx.text(serde_json::to_string(&pong).unwrap());
            }
            "pusher:ping" => {
                let pong = PusherMessage {
                    event: "pusher:pong".to_string(),
                    data: None,
                    channel: None,
                    name: None,
                };
                ctx.text(serde_json::to_string(&pong).unwrap());
            }
            "pusher:subscribe" => {
                Log::websocket_title("Subscribing to a channel");
                let subscription = PusherMessage {
                    event: "pusher_internal:subscription_succeeded".to_string(),
                    data: None,
                    channel: None,
                    name: None,
                };
                Log::websocket_title(format!("Sending subscription message: {:?}", message).as_str());
                ctx.text(serde_json::to_string(&subscription).unwrap());
                let message_data = message.data.unwrap();
                self.local_adapter.do_send(AddSocket {
                    app_id: self.app_id.clone().unwrap(),
                    socket_id: self.id.clone().unwrap(),
                    socket_addr: ctx.address(),
                });
                self.local_adapter.do_send(AddToChannel {
                    app_id: self.app_id.clone().unwrap(),
                    socket_id: self.id.clone().unwrap(),
                    channel: message_data.channel.unwrap(),
                });
            }
            "pusher:unsubscribe" => {
                Log::websocket_title("Unsubscribing from a channel");
                let unsubscription = PusherMessage {
                    event: "pusher_internal:unsubscribed".to_string(),
                    data: None,
                    channel: None,
                    name: None,
                };
                ctx.text(serde_json::to_string(&unsubscription).unwrap());
            }
            _ => {
                Log::websocket_title("Unknown event");
            }
        }
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct OnPusherMessage {
    pub(crate) message: Value,
}

impl Handler<OnPusherMessage> for WS {
    type Result = ();

    fn handle(&mut self, msg: OnPusherMessage, ctx: &mut Self::Context) -> Self::Result {
        ctx.text(serde_json::to_string(&msg.message).unwrap());
    }
}
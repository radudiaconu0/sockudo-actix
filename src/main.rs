mod server;
mod ws_message;
mod utils;
mod message;
mod log;
mod namespace;
mod adapter;
mod channel_managers;
mod app;
mod config;

use std::collections::HashMap;
use actix::{Actor, Addr, AsyncContext, StreamHandler};
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, HttpRequest, Error};
use actix_web::web::Path;
use actix_web_actors::ws;
use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::adapter::local_adapter::{GetNamespace, LocalAdapter, SendMessage};
use crate::log::Log;
use crate::message::PusherApiMessage;
use crate::namespace::{Namespace};

/// Define HTTP actor
#[derive(Debug)]
struct WS {
    id: Option<String>,
    app_id: Option<String>,
    local_adapter: Addr<LocalAdapter>,
}

impl Actor for WS {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        Log::websocket_title("Connection opened");
        let id = utils::generate_socket_id();
        self.id = Some(id.clone());
        let broadcast_message = json!({
            "event": "pusher:connection_established",
            "data": {
                "socket_id": id,
                "activity_timeout": 120,
            },
        });
        println!("{:?}", broadcast_message);
        ctx.text(broadcast_message.to_string());
    }
}

impl WS {
    pub fn new(local_adapter: Addr<LocalAdapter>, app_id: String) -> Self {
        WS {
            id: None,
            app_id: Some(app_id),
            local_adapter,
        }
    }
}

/// Handler for ws::Message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WS {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => {
                println!("Received a text message: {}", text);
                let message = ws_message::OnMessage {
                    message: serde_json::from_str(text.to_string().as_str()).unwrap(),
                };
                ctx.address().do_send(message);
            }
            Ok(ws::Message::Binary(bin)) => {
                ctx.binary(bin);
                println!("Received a binary message")
            }
            Ok(ws::Message::Pong(_)) => println!("Received a pong"),
            Ok(ws::Message::Close(reason)) => {
                println!("Received a close message: {:?}", reason);
                ctx.close(reason);
            }
            Ok(ws::Message::Continuation(_)) => println!("Received a continuation message"),
            Ok(ws::Message::Nop) => println!("Received a nop message"),
            Err(e) => {
                println!("An error occurred: {:?}", e);
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct PusherQuery {
    protocol: String,
    client: String,
    version: String,
    flash: String
}

#[get("/app/{app_id}")]
async fn ws_handler(app_id: Path<String>,
                    query: web::Query<PusherQuery>,
                    req: HttpRequest, 
                    stream: web::Payload, 
                    local_adapter: web::Data<Addr<LocalAdapter>>,
                    
) -> Result<HttpResponse, Error> {
    let app_id = app_id.into_inner();
    let local_adapter = local_adapter.get_ref().clone();
    let resp = ws::start(WS::new(local_adapter, app_id), &req, stream);
    resp
}

#[post("/apps/{app_id}/events")]
async fn pusher_event(app_id: Path<String>, info: web::Json<PusherApiMessage>, local_adapter: web::Data<Addr<LocalAdapter>>) -> impl Responder {
    let message = info.into_inner();
    local_adapter.do_send(SendMessage {
        app_id: app_id.into_inner(),
        message,
    });
    HttpResponse::Ok().body("Event sent")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let local_adapter = LocalAdapter {
        namespaces: HashMap::new(),
    }.start();
    Log::info_title("Starting server");
    HttpServer::new(move || {
        App::new()
            .service(ws_handler)
            .service(pusher_event)
            .app_data(web::Data::new(local_adapter.clone()))
    })
        .bind(("127.0.0.1", 6001))?
        .workers(32)
        .run()
        .await
}
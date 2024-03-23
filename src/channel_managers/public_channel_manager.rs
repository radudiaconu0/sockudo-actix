use actix::{Actor, Addr, Message};
use crate::adapter::local_adapter::LocalAdapter;

struct PublicChannelManager {
    local_adapter: Addr<LocalAdapter>,
}

impl Actor for PublicChannelManager {
    type Context = actix::Context<Self>;
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Join {
    pub(crate) app_id: String,
    pub(crate) channel: String,
    pub(crate) socket_id: String,
}

impl actix::Handler<Join> for PublicChannelManager {
    type Result = ();

    fn handle(&mut self, msg: Join, _: &mut Self::Context) {
        self.local_adapter.do_send(crate::adapter::local_adapter::AddToChannel {
            app_id: msg.app_id.clone(),
            channel: msg.channel.clone(),
            socket_id: msg.socket_id.clone(),
        });
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Leave {
    pub(crate) app_id: String,
    pub(crate) channel: String,
    pub(crate) socket_id: String,
}






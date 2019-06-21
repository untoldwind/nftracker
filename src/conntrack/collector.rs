use crate::config::Config;
use actix::{Actor, AsyncContext, Context, Handler, Message};
use std::time::Duration;

pub struct ConntrackCollector {
    config: Config,
}

#[derive(Message)]
struct Ping;

impl ConntrackCollector {
    pub fn new(config: Config) -> ConntrackCollector {
        ConntrackCollector { config }
    }
}

impl Handler<Ping> for ConntrackCollector {
    type Result = ();

    fn handle(&mut self, msg: Ping, ctx: &mut Context<ConntrackCollector>) {
        ctx.notify_later(Ping, Duration::from_millis(500));
    }
}

impl Actor for ConntrackCollector {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        ctx.notify(Ping);
    }
}

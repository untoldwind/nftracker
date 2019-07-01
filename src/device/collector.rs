use crate::common::Timeseries;
use crate::config::Config;
use actix::{Actor, AsyncContext, Context, Handler, Message};
use std::time::Duration;

pub struct DeviceCollector {
    config: Config,
    timeseries: Timeseries,
}

#[derive(Message)]
struct Ping;

impl DeviceCollector {
    pub fn new(config: Config) -> DeviceCollector {
        DeviceCollector {
            config,
            timeseries: Default::default(),
        }
    }
}

impl Handler<Ping> for DeviceCollector {
    type Result = ();

    fn handle(&mut self, msg: Ping, ctx: &mut Context<DeviceCollector>) {
        ctx.notify_later(Ping, Duration::from_millis(500));
    }
}
impl Actor for DeviceCollector {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        ctx.notify(Ping);
    }
}
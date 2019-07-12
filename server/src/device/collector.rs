use super::parse;
use crate::common::Trafic;
use crate::config::Config;
use actix::{Actor, AsyncContext, Context, Handler, Message};
use chrono::{NaiveDateTime, Utc};
use log::error;
use std::fs::File;
use std::io::{self, Read};
use std::time::Duration;

pub struct DeviceCollector {
    config: Config,
    traffic: Trafic,
}

#[derive(Message)]
struct Ping;

struct SeriesCollector<'a> {
    now: NaiveDateTime,
    interface: &'a str,
    traffic: &'a mut Trafic,
}

impl<'a> SeriesCollector<'a> {
    fn process<I: Read>(
        traffic: &mut Trafic,
        interface: &str,
        input: I,
    ) -> io::Result<()> {
        let collector = SeriesCollector {
            now: Utc::now().naive_utc(),
            interface,
            traffic,
        };
        parse::parse(input, collector, SeriesCollector::collect)?;

        Ok(())
    }

    fn collect(self, stats: &parse::InterfaceStats) -> Self {
        if stats.interface == self.interface {
            self.traffic
                .put_in(self.now, stats.receive_bytes, stats.receive_packets);
            self.traffic
                .put_out(self.now, stats.transmit_bytes, stats.transmit_packets);
        }
        self
    }
}

impl DeviceCollector {
    pub fn new(config: Config) -> DeviceCollector {
        DeviceCollector {
            traffic: Trafic::new(config.retain_data),
            config,
        }
    }

    fn process_device_file(&mut self) -> io::Result<()> {
        let file = File::open(&self.config.device_file)?;
        SeriesCollector::process(
            &mut self.traffic,
            &self.config.wan_interface,
            file,
        )
    }
}

impl Handler<Ping> for DeviceCollector {
    type Result = ();

    fn handle(&mut self, _: Ping, ctx: &mut Context<DeviceCollector>) {
        if let Err(error) = self.process_device_file() {
            error!("Process device file failed: {}", error)
        }
        ctx.notify_later(Ping, Duration::from_millis(500));
    }
}

impl Actor for DeviceCollector {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        ctx.notify(Ping);
    }
}

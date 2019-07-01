use super::parse;
use crate::common::Timeseries;
use crate::config::Config;
use actix::{Actor, AsyncContext, Context, Handler, Message};
use std::time::{Duration, Instant};
use std::io::{Read, self};
use std::fs::File;
use log::error;

pub struct DeviceCollector {
    config: Config,
    timeseries: Timeseries,
}

#[derive(Message)]
struct Ping;


struct SeriesCollector<'a> {
    now: Instant,
    interface: &'a str,
    timeseries: &'a mut Timeseries,
}

impl<'a> SeriesCollector<'a> {
    fn process<I: Read>(timeseries: &mut Timeseries, interface: &str, input: I) -> io::Result<()> {
        let collector = SeriesCollector {
            now: Instant::now(),
            interface,
            timeseries,
        };
        let collector = parse::parse(input, collector, SeriesCollector::collect)?;

        collector.cleanup();

        Ok(())
    }

    fn collect(self, stats: &parse::InterfaceStats) -> Self {
        if stats.interface == self.interface {
            self.timeseries
                .push_in(self.now, stats.receive_bytes, stats.receive_packets);
            self.timeseries
                .push_out(self.now, stats.transmit_bytes, stats.transmit_packets);
        }
        self
    }

    fn cleanup(self) {
    }
}

impl DeviceCollector {
    pub fn new(config: Config) -> DeviceCollector {
        DeviceCollector {
            config,
            timeseries: Default::default(),
        }
    }

    fn process_device_file(&mut self) -> io::Result<()> {
        let file = File::open(&self.config.device_file)?;
        SeriesCollector::process(&mut self.timeseries, &self.config.wan_interface, file)
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

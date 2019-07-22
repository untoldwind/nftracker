use super::parse;
use super::Lease;
use crate::config::Config;
use actix::{Actor, AsyncContext, Context, Handler, Message, MessageResult};
use log::{debug, error};
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::net::IpAddr;
use std::time::Duration;

pub struct LeasesCollector {
    config: Config,
    leases: Vec<Lease>,
}

#[derive(Message)]
struct Ping;

#[derive(Message)]
#[rtype(result="HashMap<IpAddr, Lease>")]
struct Snapshot;

impl LeasesCollector {
    pub fn new(config: Config) -> LeasesCollector {
        LeasesCollector {
            config,
            leases: Default::default(),
        }
    }

    fn process_leases_file(&mut self) -> io::Result<()> {
        debug!("Collecting: {}", self.config.leases_file);
        let file = File::open(&self.config.leases_file)?;

        self.leases = parse::parse(file, vec![], |mut leases, lease| {
            leases.push(lease);
            leases
        })?;
        Ok(())
    }
}

impl Handler<Ping> for LeasesCollector {
    type Result = ();

    fn handle(&mut self, _: Ping, ctx: &mut Context<LeasesCollector>) {
        if let Err(error) = self.process_leases_file() {
            error!("Process device file failed: {}", error)
        }
        ctx.notify_later(Ping, Duration::from_millis(500));
    }
}

impl Handler<Snapshot> for LeasesCollector {
    type Result = MessageResult<Snapshot>;

    fn handle(&mut self, _: Snapshot, _: &mut Context<LeasesCollector>) -> Self::Result {
        let mut leases_map : HashMap<IpAddr, Lease> = Default::default();

        for lease in self.leases.iter() {
            leases_map.insert(lease.addr, lease.clone());
        }
        MessageResult(leases_map)
    }
}

impl Actor for LeasesCollector {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        ctx.notify(Ping);
    }
}

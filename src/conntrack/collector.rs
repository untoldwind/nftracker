use super::parse;
use super::{Local, Table};
use crate::config::Config;
use actix::{Actor, AsyncContext, Context, Handler, Message};
use log::error;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, Read};
use std::time::{Duration, Instant};

pub struct ConntrackCollector {
    config: Config,
    table: Table,
}

#[derive(Message)]
struct Ping;

struct TableCollector<'a> {
    now: Instant,
    table: &'a mut Table,
    local_subnets: &'a [String],
    locals: HashSet<Local>,
}

impl<'a> TableCollector<'a> {
    fn process<I: Read>(table: &mut Table, local_subnets: &[String], input: I) -> io::Result<()> {
        let collector = TableCollector {
            now: Instant::now(),
            table,
            local_subnets,
            locals: Default::default(),
        };
        let collector = parse::parse(input, collector, TableCollector::collect)?;

        collector.cleanup();

        Ok(())
    }

    fn collect(mut self, entry: &parse::ConntrackEntry) -> Self {
        for subnet in self.local_subnets {
            if entry.src.starts_with(subnet) {
                self.table
                    .push_out(self.now, entry.src, entry.dst, entry.bytes, entry.packets);
                self.locals.insert(entry.src.to_string());
                break;
            }
            if entry.dst.starts_with(subnet) {
                self.table
                    .push_out(self.now, entry.dst, entry.src, entry.bytes, entry.packets);
                self.locals.insert(entry.dst.to_string());
                break;
            }
        }
        self
    }

    fn cleanup(self) {
        let obsolete = self
            .table
            .0
            .keys()
            .filter(|source| !self.locals.contains(*source))
            .cloned()
            .collect::<Vec<Local>>();

        for source in obsolete {
            self.table.0.remove(&source);
        }
    }
}

impl ConntrackCollector {
    pub fn new(config: Config) -> ConntrackCollector {
        ConntrackCollector {
            config,
            table: Default::default(),
        }
    }

    fn process_conntrack(&mut self) -> io::Result<()> {
        let file = File::open(&self.config.conntrack_file)?;
        TableCollector::process(&mut self.table, &self.config.local_subnets, file)?;

        unimplemented!()
    }
}

impl Handler<Ping> for ConntrackCollector {
    type Result = ();

    fn handle(&mut self, msg: Ping, ctx: &mut Context<ConntrackCollector>) {
        if let Err(error) = self.process_conntrack() {
            error!("Process conntrack failed: {}", error)
        }
        ctx.notify_later(Ping, Duration::from_millis(500));
    }
}

impl Actor for ConntrackCollector {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        ctx.notify(Ping);
    }
}

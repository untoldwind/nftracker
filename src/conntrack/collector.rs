use super::parse;
use super::{Source, Table};
use crate::config::Config;
use actix::{Actor, AsyncContext, Context, Handler, Message};
use log::error;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, Read};
use std::time::Duration;

pub struct ConntrackCollector {
    config: Config,
    table: Table,
}

#[derive(Message)]
struct Ping;

struct TableCollector<'a> {
    table: &'a mut Table,
    local_subnets: &'a [String],
    sources: HashSet<Source>,
}

impl<'a> TableCollector<'a> {
    fn process<I: Read>(table: &mut Table, local_subnets: &[String], input: I) -> io::Result<()> {
        let collector = TableCollector {
            table,
            local_subnets,
            sources: Default::default(),
        };
        let collector = parse::parse(input, collector, TableCollector::collect)?;

        collector.cleanup();

        Ok(())
    }

    fn collect(self, entry: &parse::ConntrackEntry) -> Self {
        self
    }

    fn cleanup(self) {
        let obsolete = self
            .table
            .0
            .keys()
            .filter(|source| !self.sources.contains(*source))
            .cloned()
            .collect::<Vec<Source>>();

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

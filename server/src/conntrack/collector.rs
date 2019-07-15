use super::parse;
use super::{Local, Table};
use crate::common::Subnet;
use crate::config::Config;
use actix::{Actor, AsyncContext, Context, Handler, Message};
use chrono::{NaiveDateTime, Utc};
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
    now: NaiveDateTime,
    table: &'a mut Table,
    local_subnets: &'a [Subnet],
    locals: HashSet<Local>,
}

impl<'a> TableCollector<'a> {
    fn process<I: Read>(table: &mut Table, local_subnets: &[Subnet], input: I) -> io::Result<()> {
        let collector = TableCollector {
            now: Utc::now().naive_utc(),
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
            if subnet.contains(&entry.src) {
                self.table
                    .push_out(self.now, entry.src, entry.dst, entry.bytes, entry.packets);
                self.locals.insert(entry.src);
                break;
            }
            if subnet.contains(&entry.dst) {
                self.table
                    .push_in(self.now, entry.dst, entry.src, entry.bytes, entry.packets);
                self.locals.insert(entry.dst);
                break;
            }
        }
        self
    }

    fn cleanup(self) {
        let obsolete = self
            .table
            .connections
            .keys()
            .filter(|source| !self.locals.contains(*source))
            .cloned()
            .collect::<Vec<Local>>();

        for source in obsolete {
            self.table.connections.remove(&source);
        }
    }
}

impl ConntrackCollector {
    pub fn new(config: Config) -> ConntrackCollector {
        ConntrackCollector {
            table: Table::new(config.retain_data),
            config,
        }
    }

    fn process_conntrack(&mut self) -> io::Result<()> {
        let file = File::open(&self.config.conntrack_file)?;
        TableCollector::process(&mut self.table, &self.config.local_subnets, file)?;

        Ok(())
    }
}

impl Handler<Ping> for ConntrackCollector {
    type Result = ();

    fn handle(&mut self, _: Ping, ctx: &mut Context<ConntrackCollector>) {
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

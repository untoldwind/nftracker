use super::ConntrackSimulator;
use std::fs::File;
use std::io::{self, Write};

pub struct DeviceSimulator<'a> {
    file: &'a str,
}

impl<'a> DeviceSimulator<'a> {
    pub fn new(file: &'a str) -> DeviceSimulator<'a> {
        DeviceSimulator { file }
    }

    pub fn dump(&self, conntrack_simulator: &ConntrackSimulator) -> io::Result<()> {
        let mut f = File::create(self.file)?;
        let in_traffic = conntrack_simulator.total_in_traffic();
        let out_traffic = conntrack_simulator.total_out_traffic();

        writeln!(
            f,
            "Inter-|   Receive                                                |  Transmit"
        )?;
        writeln!(f, " face |bytes    packets errs drop fifo frame compressed multicast|bytes    packets errs drop fifo colls carrier compressed")?;
        writeln!(f,
            "  eth0:  {}    {}    0    0    0     0          0         6   {}    {}    0    0    0     0       0          0",
            in_traffic.bytes, in_traffic.packets, out_traffic.bytes, out_traffic.packets)?;

        Ok(())
    }
}

use super::ConntrackSimulator;
use std::fs::File;
use std::io::{self, Write};

pub struct LeasesSimulator<'a> {
    file: &'a str,
}

impl<'a> LeasesSimulator<'a> {
    pub fn new(file: &'a str) -> LeasesSimulator<'a> {
        LeasesSimulator { file }
    }

    pub fn dump(&self, conntrack_simulator: &ConntrackSimulator) -> io::Result<()> {
        let mut f = File::create(self.file)?;

        Ok(())
    }
}

use super::ConntrackSimulator;

pub struct LeasesSimulator<'a> {
    file: &'a str,
    conntrack_simulator: &'a ConntrackSimulator<'a>,
}

impl<'a> LeasesSimulator<'a> {
    pub fn new(
        file: &'a str,
        conntrack_simulator: &'a ConntrackSimulator<'a>,
    ) -> LeasesSimulator<'a> {
        LeasesSimulator {
            file,
            conntrack_simulator,
        }
    }
}

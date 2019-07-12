use super::ConntrackSimulator;

pub struct DeviceSimulator<'a> {
    file: &'a str,
    conntrack_simulator: &'a ConntrackSimulator<'a>,
}

impl<'a> DeviceSimulator<'a> {
    pub fn new(
        file: &'a str,
        conntrack_simulator: &'a ConntrackSimulator<'a>,
    ) -> DeviceSimulator<'a> {
        DeviceSimulator {
            file,
            conntrack_simulator,
        }
    }
}

pub struct ConntrackSimulator<'a> {
    file: &'a str,
}

impl<'a> ConntrackSimulator<'a> {
    pub fn new(file: &'a str) -> ConntrackSimulator<'a> {
        ConntrackSimulator { file }
    }
}

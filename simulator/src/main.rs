mod cli;
mod simulator;

fn main() -> std::io::Result<()> {
    let matches = cli::app();
    let mut log_builder = env_logger::Builder::from_default_env();

    if matches.is_present("debug") {
        log_builder.filter(None, log::LevelFilter::Debug);
    } else {
        log_builder.filter(None, log::LevelFilter::Info);
    }
    log_builder.init();

    let conntrack_simulator =
        simulator::ConntrackSimulator::new(matches.value_of("conntrack-file").unwrap());
    let device_simulator = simulator::DeviceSimulator::new(
        matches.value_of("device-file").unwrap(),
        &conntrack_simulator,
    );
    let leases_simulator = simulator::LeasesSimulator::new(
        matches.value_of("leases-file").unwrap(),
        &conntrack_simulator,
    );

    Ok(())
}

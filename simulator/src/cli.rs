use clap::{App, Arg, ArgMatches};

pub fn app<'a>() -> ArgMatches<'a> {
    App::new("nftracker-simulator")
        .version("0.1")
        .arg(Arg::with_name("debug").short("D").long("debug"))
        .arg(
            Arg::with_name("leases-file")
                .long("leases-file")
                .value_name("FILE")
                .default_value("simulated.leases"),
        )
        .arg(
            Arg::with_name("conntrack-file")
                .long("conntrack-file")
                .value_name("FILE")
                .default_value("simulated.nf_conntrack"),
        )
        .arg(
            Arg::with_name("device-file")
                .long("device-file")
                .value_name("FILE")
                .default_value("simulated.device"),
        )
        .get_matches()
}

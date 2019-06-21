use clap::{App, Arg, ArgMatches};

pub fn app<'a>() -> ArgMatches<'a> {
    App::new("nftracker")
        .version("0.1")
        .arg(Arg::with_name("debug").short("D").long("debug"))
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .takes_value(true)
                .default_value("nftracker.toml"),
        )
        .get_matches()
}

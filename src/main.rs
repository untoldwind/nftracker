use actix::{Actor, Addr, System};
use actix_web::{web, App, HttpServer, Responder};

mod cli;
mod config;
mod conntrack;
mod device;
mod leases;
mod minivec;
mod model;

use config::Config;
use conntrack::ConntrackCollector;

#[derive(Clone)]
struct Container {
    conntrack: Addr<ConntrackCollector>,
}

fn index(info: web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}", info.1, info.0)
}

fn main() -> std::io::Result<()> {
    let matches = cli::app();
    let mut log_builder = env_logger::Builder::from_default_env();

    if matches.is_present("debug") {
        log_builder.filter(None, log::LevelFilter::Debug);
    } else {
        log_builder.filter(None, log::LevelFilter::Info);
    }
    log_builder.init();

    let config_file = matches.value_of("config").unwrap();
    let config = Config::read(config_file).unwrap();

    let sys = System::new("nftracker");

    let container = web::Data::new(Container {
        conntrack: ConntrackCollector::new(config).start(),
    });

    HttpServer::new(move || {
        App::new()
            .register_data(container.clone())
            .service(web::resource("/{id}/{name}/index.html").to(index))
    })
    .bind("0.0.0.0:8080")?
    .start();

    sys.run()
}

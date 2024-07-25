use std::{
    env::VarError,
    net::{Ipv4Addr, SocketAddrV4},
};

use tokio::net::TcpListener;
use tracing::{error, info, trace, Level};
use tracing_subscriber::FmtSubscriber;

pub fn initialise_logging() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed!");
    info!("quote-engine says hello! backend tracing up...");
}

fn get_port() -> u16 {
    match std::env::var("PORT") {
        Ok(portstr) => match portstr.parse() {
            Ok(num) => num,
            Err(e) => {
                error!("error parsing env var PORT from str to u16: {e}");
                panic!();
            }
        },
        Err(e) => {
            match e {
                VarError::NotPresent => trace!("PORT environment variable not found"),
                _ => info!("PORT environemtn variable error: {e}"),
            }
            return 2019;
        }
    }
}

pub fn get_socket_addr() -> SocketAddrV4 {
    let addr = SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), get_port());
    trace!("desired socket addr is {}", addr.to_string());
    addr
}

pub fn report_listener_socket_addr(listener: &TcpListener) {
    let addr = match listener.local_addr() {
        Ok(addr) => addr,
        Err(e) => {
            error!("error while getting listener socket address: {e}");
            panic!();
        }
    };
    info!("listener socket addr is {}", addr.to_string());
}

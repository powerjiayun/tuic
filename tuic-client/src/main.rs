use self::{
    config::{Config, ConfigError},
    connection::Endpoint,
    socks5::Server,
};
use quinn::{ConnectError, ConnectionError};
use std::{env, io::Error as IoError, process};
use thiserror::Error;
use tuic_quinn::Error as ModelError;
use webpki::Error as WebpkiError;

mod config;
mod connection;
mod socks5;
mod utils;

#[tokio::main]
async fn main() {
    let cfg = match Config::parse(env::args_os()) {
        Ok(cfg) => cfg,
        Err(ConfigError::Version(msg) | ConfigError::Help(msg)) => {
            println!("{msg}");
            process::exit(0);
        }
        Err(err) => {
            eprintln!("{err}");
            process::exit(1);
        }
    };

    match Endpoint::set_config(cfg.relay) {
        Ok(()) => {}
        Err(err) => {
            eprintln!("{err}");
            process::exit(1);
        }
    }

    match Server::set_config(cfg.local) {
        Ok(()) => {}
        Err(err) => {
            eprintln!("{err}");
            process::exit(1);
        }
    }

    Server::start().await;
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] IoError),
    #[error(transparent)]
    Connect(#[from] ConnectError),
    #[error(transparent)]
    Connection(#[from] ConnectionError),
    #[error(transparent)]
    Model(#[from] ModelError),
    #[error(transparent)]
    Webpki(#[from] WebpkiError),
    #[error("timeout establishing connection")]
    Timeout,
    #[error("cannot resolve the server name")]
    DnsResolve,
    #[error("received packet from an unexpected source")]
    WrongPacketSource,
    #[error("invalid socks5 authentication")]
    InvalidSocks5Auth,
}

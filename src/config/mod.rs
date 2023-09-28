use anyhow::anyhow;
use clap::{App, Arg};
#[derive(Clone, Debug)]
pub struct Config {
    pub config_dir: Option<String>,
    pub grpc_host: Option<String>,
    pub grpc_port: Option<u16>,
}

pub fn parse_args() -> anyhow::Result<Config> {
    let version = env!("CARGO_PKG_VERSION");
    let app = App::new("relayer")
        .about("A threshold signature scheme daemon")
        .version(version)
        .arg(
            Arg::new("configdir")
                .long("configdir")
                .short('d')
                .required(true)
                .default_value(""),
        )
        .arg(
            Arg::new("grpc_host")
                .long("grpc_host")
                .short('h')
                .required(false)
                .default_value("127.0.0.1"),
        )
        .arg(
            Arg::new("grpc_port")
                .long("grpc_port")
                .short('p')
                .required(false)
                .default_value("50000"),
        );
    let matches = app.get_matches();

    let config_dir = matches
        .value_of("configdir")
        .ok_or_else(|| anyhow!("config dir"))?
        .to_string();
    let grpc_host = matches
        .value_of("grpc_host")
        .unwrap_or_else(|| "127.0.0.1")
        .to_string();
    let grpc_port = matches
        .value_of("grpc_port")
        .unwrap_or_else(|| "50000")
        .parse()?;
    Ok(Config {
        config_dir: Some(config_dir),
        grpc_host: Some(grpc_host),
        grpc_port: Some(grpc_port),
    })
}

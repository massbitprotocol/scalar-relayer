use std::fs;

use anyhow::{anyhow, Result};
use futures::future::join_all;
use scalar_relayer::config::parse_args;
use scalar_relayer::grpc;
use scalar_relayer::proto::scalar_abci_client::ScalarAbciClient;
use scalar_relayer::relayer::{self, RelayerConfigs};
use tokio::sync::mpsc::{self, UnboundedReceiver};
use tokio::task::JoinHandle;
use tracing::{info, Level};

fn set_up_logs() {
    // enable only tofnd and tofn debug logs - disable serde, tonic, tokio, etc.
    // tracing_subscriber::fmt()
    //     .with_env_filter("tofnd=debug,tofn=debug")
    //     .json()
    //     .with_ansi(atty::is(atty::Stream::Stdout))
    //     .with_target(false)
    //     .with_current_span(false)
    //     .flatten_event(true) // make logs complient with datadog
    //     .init();
    // let subscriber = tracing_subscriber::FmtSubscriber::new();
    let subscriber = tracing_subscriber::fmt()
        //.with_max_level(Level::DEBUG)
        .with_max_level(Level::INFO)
        .finish();

    // use that subscriber to process traces emitted after this point
    // let subscriber = FmtSubscriber::builder()
    //     .with_max_level(Level::INFO)
    //     .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
}

#[tokio::main]
async fn main() -> Result<()> {
    set_up_logs();
    let config = parse_args()?;
    let mut handles = vec![];
    let relayer_config_dir = config
        .config_dir
        .clone()
        .unwrap_or_else(|| String::from("/opt/config"));
    for ind in 0..4 {
        let cfg = config.clone();
        //info!("Start tofnd at path {:?}", &cfg.tofnd_path);
        let grpc_handle = tokio::spawn(async move {
            let _ = grpc::start_server(cfg, ind).await;
        });
        handles.push(grpc_handle);
        let config_path = format!("{}/evm_relayer{}.toml", relayer_config_dir, ind);
        let config_str = fs::read_to_string(config_path.as_str())
            .map_err(|e| {
                let msg = format!("{:?}", e);
                println!("{}", msg.as_str());
                anyhow!(msg)
            })
            .expect(format!("Failed to read relayer config file {}", config_path).as_str());
        let relayer_configs: RelayerConfigs = toml::from_str(config_str.as_str())?;
        let (tx_out, rx_out) = mpsc::unbounded_channel::<String>();
        let sender_handle = spawn_sender(relayer_configs.clone(), rx_out);
        handles.push(sender_handle);

        let relayer_handles = relayer_configs
            .relayer_evm
            .iter()
            .filter(|item| item.start_with_bridge.unwrap_or(false))
            .map(|relayer_config| {
                let gprc_url = format!(
                    "http://{}:{}",
                    config.grpc_host.as_ref().unwrap(),
                    config.grpc_port.as_ref().unwrap() + ind
                );
                let tx = tx_out.clone();
                let relayer_config = relayer_config.clone();
                let handle = tokio::spawn(async move {
                    if let Ok(client) = ScalarAbciClient::connect(gprc_url).await {
                        let _ = relayer::start(relayer_config, client, tx).await;
                    }
                });
                handle
            });

        handles.extend(relayer_handles);
    }
    join_all(handles).await;
    Ok(())
}

fn spawn_sender(
    relayer_configs: RelayerConfigs,
    mut rx_out: UnboundedReceiver<String>,
) -> JoinHandle<()> {
    tokio::spawn(async move {
        while let Some(item) = rx_out.recv().await {
            //let value = serde_json::to_string(&item).unwrap();
            //println!("Push block into stream {:?}", &value);
            info!(
                "Receive message {:?}. Todo: Send it to the destination chain",
                item
            );
        }
    })
}

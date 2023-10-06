use crate::relayer::EvmRelayer;
use anyhow::{anyhow, Result};
use futures::future::join_all;
use scalar_relayer::config::parse_args;
use scalar_relayer::proto::scalar_abci_client::ScalarAbciClient;
use scalar_relayer::relayer::{self, RelayerConfigs};
use scalar_relayer::types::ContractCallFilter;
use scalar_relayer::types::ScalarEventTransaction;
use scalar_relayer::{grpc, TSS_ADDRESS};
use std::fs;
use tokio::sync::mpsc::{self, UnboundedReceiver};
use tokio::task::JoinHandle;
use tracing::{info, warn, Level};
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
    // let tss_address = Address::from_slice(&keccak256(TSS_ADDRESS.as_bytes()));
    for ind in 0..4 {
        let cfg = config.clone();
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
        //info!("RelayerConfigs {:?}", &relayer_configs);
        let (tx_out, rx_out) = mpsc::unbounded_channel::<String>();
        let sender_handle = spawn_sender(relayer_configs.clone(), rx_out);
        handles.push(sender_handle);

        let relayer_handles = relayer_configs
            .relayer_evm
            .iter()
            .filter(|item| item.start_with_bridge.unwrap_or(false))
            .map(|relayer_config| {
                info!("Start relayer with config {:?}", &relayer_config);
                let gprc_url = format!(
                    "{}:{}",
                    config.grpc_host.as_ref().unwrap(),
                    config.grpc_port.as_ref().unwrap() + ind
                );
                warn!("Narwhal Grpc server {:?}", &gprc_url);
                let tx = tx_out.clone();
                let relayer_config = relayer_config.clone();
                let handle = tokio::spawn(async move {
                    let client = ScalarAbciClient::connect(gprc_url).await.ok();
                    if client.is_none() {
                        warn!("Cannot connect to narwhal grpc!");
                    } else {
                        info!("Connected to narwhal grpc!");
                    }
                    //let _ = relayer::start(relayer_config, client, tx).await;
                    let _ = relayer::start_listener(relayer_config, client, tx).await;
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
    let tss_address = TSS_ADDRESS.clone();
    info!("Tss_address {:#02x?}", &tss_address);
    tokio::spawn(async move {
        while let Some(item) = rx_out.recv().await {
            //let value = serde_json::to_string(&item).unwrap();
            //println!("Push block into stream {:?}", &value);
            let ScalarEventTransaction {
                payload,
                tss_signature,
            } = serde_json::from_str(item.as_str()).unwrap();
            let event_value: ContractCallFilter =
                serde_json::from_slice(payload.as_slice()).unwrap();
            info!(
                "Receive message {:?} with signature {:?} of length {:?}. Todo: Send it to the destination chain",
                &event_value, &tss_signature, tss_signature.len()
            );
            //[48, 69, 2, 33, 0, 203, 84, 195, 47, 156, 83, 199, 231, 106, 54, 196, 67, 234, 18, 196, 66, 228, 133, 112, 120, 166, 233, 20, 53, 233, 18, 162, 219, 184, 100, 135, 190, 2, 32, 41, 211, 198, 193, 14, 127, 245, 90, 169, 218, 20, 59, 161, 1, 94, 96, 56, 192, 80, 198, 254, 180, 100, 181, 23, 68, 44, 69, 226, 224, 0, 118]
            // let destination_chain = event_value.destination_chain.clone();
            if let Some(des_relayer_config) = relayer_configs.relayer_evm.iter().find(|item| {
                item.start_with_bridge.unwrap_or(false)
                    && item.rpc_addr.is_some()
                    && item.name.as_str() == event_value.destination_chain.as_str()
            }) {
                let evm_relayer = EvmRelayer::new(&des_relayer_config);
                let res = evm_relayer
                    .call_destination_contract(event_value, tss_address.clone(), tss_signature)
                    .await;
            }
        }
    })
}

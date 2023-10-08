use crate::relayer::EvmRelayer;
use anyhow::{anyhow, Result};
use ethers::utils::keccak256;
use futures::future::join_all;
use k256::{
    ecdsa::{hazmat::VerifyPrimitive, RecoveryId, Signature, VerifyingKey},
    elliptic_curve::{sec1::FromEncodedPoint, PrimeField},
    FieldBytes,
};
use scalar_relayer::config::parse_args;
use scalar_relayer::proto::scalar_abci_client::ScalarAbciClient;
use scalar_relayer::relayer::{self, RelayerConfigs};
use scalar_relayer::types::ScalarEventTransaction;
use scalar_relayer::types::{ContractCallFilter, ScalarOutgoingMessage};
use scalar_relayer::{grpc, TSS_ADDRESS};
use sha3::{Digest, Keccak256};
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
        let (tx_out, rx_out) = mpsc::unbounded_channel::<ScalarOutgoingMessage>();
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
    mut rx_out: UnboundedReceiver<ScalarOutgoingMessage>,
) -> JoinHandle<()> {
    let tss_address = TSS_ADDRESS.clone();
    info!("Tss_address {:#02x?}", &tss_address);
    tokio::spawn(async move {
        let mut pub_key = vec![];
        while let Some(item) = rx_out.recv().await {
            //let value = serde_json::to_string(&item).unwrap();
            //println!("Push block into stream {:?}", &value);
            match item {
                ScalarOutgoingMessage::KeygenData(key) => {
                    pub_key = key.clone();
                    handle_keygen_data(&relayer_configs, key).await;
                }
                ScalarOutgoingMessage::Transaction(tran) => {
                    handle_transaction(&relayer_configs, pub_key.as_slice(), tran).await;
                }
            }
        }
    })
}
async fn handle_keygen_data(
    relayer_configs: &RelayerConfigs,
    pubkey: Vec<u8>,
) -> anyhow::Result<()> {
    //https://docs.rs/k256/latest/k256/ecdsa/#recovering-a-verifyingkey-from-a-signature
    let verified_key = VerifyingKey::from_sec1_bytes(pubkey.as_slice())?;
    let handles = relayer_configs
        .relayer_evm
        .iter()
        .find(|item| item.start_with_bridge.unwrap_or(false) && item.rpc_addr.is_some())
        .map(|config| {
            let config = config.clone();
            tokio::spawn(async move {
                let evm_relayer = EvmRelayer::new(config);
                let res = evm_relayer.update_pubkey(pubkey.clone()).await;
            })
        });
    join_all(handles).await;
    Ok(())
}

async fn handle_transaction(
    relayer_configs: &RelayerConfigs,
    pubkey: &[u8],
    message: String,
) -> anyhow::Result<()> {
    let ScalarEventTransaction {
        payload,
        tss_signature,
    } = serde_json::from_str(message.as_str()).unwrap();
    // https://docs.rs/k256/latest/k256/ecdsa/#recovering-a-verifyingkey-from-a-signature
    // https://bitcoin.stackexchange.com/questions/77191/what-is-the-maximum-size-of-a-der-encoded-ecdsa-signature
    // Todo: new create signature from Signature_Der
    let pubkey =
        k256::AffinePoint::from_encoded_point(&k256::EncodedPoint::from_bytes(pubkey).unwrap())
            .unwrap();
    let signature = k256::ecdsa::Signature::from_der(tss_signature.as_slice()).unwrap();
    let payload_hash = keccak256(payload.as_slice()).to_vec();
    assert!(pubkey
        .verify_prehashed(&FieldBytes::from_slice(payload_hash.as_slice()), &signature)
        .is_ok());
    let recid = RecoveryId::try_from(1u8)?;
    let digest = Keccak256::new_with_prefix(payload);
    let recovered_key = VerifyingKey::recover_from_digest(payload_hash, &signature, recid)?;

    // let event_value: ContractCallFilter = serde_json::copy_from_sliceice(payload.as_slice()).unwrap();
    // info!(
    //     "Receive message {:?} with signature {:?} of length {:?}. Todo: Send it to the destination chain",
    //     &event_value, &tss_signature, tss_signature.len()
    // );
    // //[48, 69, 2, 33, 0, 203, 84, 195, 47, 156, 83, 199, 231, 106, 54, 196, 67, 234, 18, 196, 66, 228, 133, 112, 120, 166, 233, 20, 53, 233, 18, 162, 219, 184, 100, 135, 190, 2, 32, 41, 211, 198, 193, 14, 127, 245, 90, 169, 218, 20, 59, 161, 1, 94, 96, 56, 192, 80, 198, 254, 180, 100, 181, 23, 68, 44, 69, 226, 224, 0, 118]
    // // let destination_chain = event_value.destination_chain.clone();
    // if let Some(des_relayer_config) = relayer_configs.relayer_evm.iter().find(|item| {
    //     item.start_with_bridge.unwrap_or(false)
    //         && item.rpc_addr.is_some()
    //         && item.name.as_str() == event_value.destination_chain.as_str()
    // }) {
    //     let config = des_relayer_config.clone();
    //     let evm_relayer = EvmRelayer::new(config);
    //     let res = evm_relayer
    //         .call_destination_contract(payload, tss_signature)
    //         .await;
    // }
    Ok(())
}

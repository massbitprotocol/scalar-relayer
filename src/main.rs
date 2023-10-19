use crate::relayer::ExecuteData;
use anyhow::{anyhow, Result};
use ethers::types::U256;
use ethers::utils::keccak256;
use futures::future::join_all;
use scalar_relayer::config::parse_args;
use scalar_relayer::proto::scalar_abci_client::ScalarAbciClient;
use scalar_relayer::relayer::{self, EvmRelayer, RelayerConfigs};
use scalar_relayer::types::{ScalarEventTransaction, ScalarOutgoingMessage};
use scalar_relayer::{create_rsv_signature, OWNER_PRIVATE_KEY};
use scalar_relayer::{eth_hash_message, grpc};
use std::collections::HashMap;
use std::fs;
use std::sync::Arc;
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
    let _private_key: String = OWNER_PRIVATE_KEY.clone();
    //info!("Private key {:?}", &private_key);
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
        let evm_relayers: Vec<Arc<EvmRelayer>> = relayer_configs
            .relayer_evm
            .iter()
            .filter(|item| {
                item.start_with_bridge.unwrap_or(false)
                    && item.rpc_addr.is_some()
                    && item.call_contract.is_some()
            })
            .enumerate()
            .map(|(index, relayer_config)| Arc::new(EvmRelayer::new(index, relayer_config.clone())))
            .collect();
        let (tx_out, rx_out) = mpsc::unbounded_channel::<ScalarOutgoingMessage>();
        let sender_handle = spawn_sender(evm_relayers.clone(), rx_out);
        handles.push(sender_handle);
        let grpc_url = format!(
            "{}:{}",
            config.grpc_host.as_ref().unwrap(),
            config.grpc_port.as_ref().unwrap() + ind
        );
        warn!("Narwhal Grpc server {:?}", &grpc_url);
        if let Ok(grpc_client) = ScalarAbciClient::connect(grpc_url).await {
            let relayer_handles = evm_relayers.into_iter().map(|relayer| {
                let tx = tx_out.clone();
                let client = grpc_client.clone();
                tokio::spawn(async move {
                    let _ = relayer::start_listener(relayer, client, tx).await;
                })
            });

            handles.extend(relayer_handles);
        } else {
            warn!("Cannot connect to narwhal grpc!");
        }
    }
    join_all(handles).await;
    Ok(())
}

fn spawn_sender(
    evm_relayers: Vec<Arc<EvmRelayer>>,
    mut rx_out: UnboundedReceiver<ScalarOutgoingMessage>,
) -> JoinHandle<()> {
    tokio::spawn(async move {
        while let Some(item) = rx_out.recv().await {
            //let value = serde_json::to_string(&item).unwrap();
            //println!("Push block into stream {:?}", &value);
            match item {
                ScalarOutgoingMessage::KeygenData((epoch, pubkey)) => {
                    let _ = handle_keygen_data(&evm_relayers, epoch, pubkey).await;
                }
                ScalarOutgoingMessage::Transaction(tran) => {
                    let _ = handle_transaction(&evm_relayers, tran).await;
                }
            }
        }
    })
}
/*
 * pubkey here is Der encoded (compressed form with 33bytes length started with 0x02 or 0x03)
 * Need uncompress it before calculate hash for 20 bytes of ETH address
 */
async fn handle_keygen_data(
    evm_relayers: &Vec<Arc<EvmRelayer>>,
    epoch: u64,
    pubkey: Vec<u8>,
) -> anyhow::Result<()> {
    // https://ethereum.org/en/developers/docs/accounts/#:~:text=The%20public%20key%20is%20generated,adding%200x%20to%20the%20beginning.
    // https://narteysarso.hashnode.dev/how-ethereum-addresses-are-generated
    // let verified_key = VerifyingKey::from_sec1_bytes(pubkey.as_slice())?;
    // let address = PublicKey::from(verified_key);
    // let address = PublicKey::from_sec1_bytes(pubkey.as_slice())
    //     .map_err(|err| anyhow!("Convert to Pubkey with error {:?}", &err))?;
    // let address_data = address.to_public_key_der().unwrap();
    /*
     * The public key is generated from the private key using the Elliptic Curve Digital Signature Algorithm.
     * You get a public address for your account by taking the last 20 bytes of the Keccak-256 hash
     * of the public key and adding 0x to the beginning.
     */
    let pubkey_hash = secp256k1::PublicKey::from_slice(&pubkey.as_slice())
        .map(|pk| {
            let uncompressed_pubkey = pk.serialize_uncompressed();
            info!(
                "Uncompressed pubkey 0x{}",
                hex::encode(&uncompressed_pubkey)
            );
            // Ignore first byte (0x04)
            let pubkey = &uncompressed_pubkey[1..];
            keccak256(&pubkey)
        })
        .map_err(|err| anyhow!("Error while decompress public key {:?}", &err))?;

    info!(
        "Tss pubkey 0x{}, keccak 256 hash 0x{}",
        hex::encode(pubkey.as_slice()),
        hex::encode(&pubkey_hash)
    );
    //Find fiest relayer with match chain id
    let mut map_chain = HashMap::<U256, bool>::new();
    let mut handles = vec![];
    for evm_relayer in evm_relayers.iter() {
        if evm_relayer
            .get_chain_id()
            .await
            .and_then(|chain_id| map_chain.insert(chain_id.clone(), true))
            .is_none()
        {
            info!("Update pubkey for chain");
            let key = pubkey.clone();
            let hash = pubkey_hash.clone();
            let relayer = evm_relayer.clone();
            handles.push(tokio::spawn(async move {
                if let Err(err) = relayer.update_pubkey(epoch, key, hash).await {
                    warn!("Update pubkey error {:?}", &err);
                }
            }));
        }
    }
    join_all(handles).await;
    Ok(())
}

async fn handle_transaction(
    evm_relayers: &Vec<Arc<EvmRelayer>>,
    message: String,
) -> anyhow::Result<()> {
    let ScalarEventTransaction {
        payload,
        tss_signature,
    } = serde_json::from_str(message.as_str()).unwrap();
    info!("Handle transaction to destination chain");
    // https://docs.rs/k256/latest/k256/ecdsa/#recovering-a-verifyingkey-from-a-signature
    // https://bitcoin.stackexchange.com/questions/77191/what-is-the-maximum-size-of-a-der-encoded-ecdsa-signature
    // Todo: new create signature from Signature_Der
    // let expected_key = VerifyingKey::from_sec1_bytes(pubkey)?;
    // let pubkey =
    //     k256::AffinePoint::from_encoded_point(&k256::EncodedPoint::from_bytes(pubkey).unwrap())
    //         .unwrap();
    let signature = k256::ecdsa::Signature::from_der(tss_signature.as_slice()).unwrap();
    let mut rsv_signature: Vec<u8> = signature.to_vec();
    create_rsv_signature(&mut rsv_signature);
    let payload_hash = eth_hash_message(payload.as_slice());
    info!(
        "Hash 0x{}, origin signature 0x{}, rsv sig 0x{}",
        hex::encode(&payload_hash),
        hex::encode(signature.to_bytes().as_slice()),
        hex::encode(rsv_signature.as_slice())
    );
    let execute_data = ExecuteData::try_from(payload.as_slice())?;
    let chain_id = execute_data.get_chain_id();
    //Find fiest relayer with match chain id
    for evm_relayer in evm_relayers.iter() {
        info!("Call destination contract with first matched relayer");
        if evm_relayer.get_chain_id().await.unwrap_or(U256::zero()) == chain_id {
            if let Err(err) = evm_relayer
                .call_destination_contract(payload, signature)
                .await
            {
                warn!("Call destination contract error {:?}", &err);
            }
            break;
        }
    }

    // assert!(pubkey
    //     .verify_prehashed(&FieldBytes::from_slice(payload_hash.as_slice()), &signature)
    //     .is_ok());
    // let recid = RecoveryId::try_from(1u8)?;
    // let digest = Keccak256::new_with_prefix(payload);

    // let recovered_key = VerifyingKey::recover_from_digest(digest, &signature, recid)?;
    // info!("Verify public key");
    // assert_eq!(recovered_key, expected_key);
    //let pub_key = PublicKey::from(recovered_key);

    // let event_value: ContractCallFilter = serde_json::copy_from_sliceice(payload.as_slice()).unwrap();
    // info!(
    //     "Receive message {:?} with signature {:?} of length {:?}. Todo: Send it to the destination chain",
    //     &event_value, &tss_signature, tss_signature.len()
    // );
    // let destination_chain = event_value.destination_chain.clone();
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

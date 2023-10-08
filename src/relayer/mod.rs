use crate::proto::scalar_abci_response::Message;
use crate::proto::{KeygenOutput, RequestArk, ScalarAbciResponse, ScalarOutTransaction};
use crate::{SELECTOR_APPROVE_CONTRACT_CALL, SUPPORTED_CHAINS};
mod evm;
pub(super) mod types;
use crate::proto::{scalar_abci_client::ScalarAbciClient, ScalarAbciRequest};
use crate::relayer::types::{ApproveContractCallParam, ExecuteData};
use crate::types::ScalarOutgoingMessage;
use crate::{
    abis::axelar_gateway::{AxelarGateway, AxelarGatewayEvents},
    types::ContractCallFilter,
};
use anyhow::anyhow;
use ethers::prelude::*;
use ethers::utils::hex::FromHex;
pub use evm::*;
use futures::future::join_all;
use serde::Deserialize;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::task::JoinHandle;
use tokio::time::{sleep, Duration};
use tonic::transport::Channel;
use tracing::{info, warn};
pub const NAMESPACE: &str = "scalar";

#[derive(Clone, Debug, Deserialize)]
pub struct RelayerConfigs {
    pub relayer_evm: Vec<RelayerConfig>,
}
/// This is what we're going to decode into. Each field is optional, meaning
/// that it doesn't have to be present in TOML.
#[derive(Clone, Debug, Deserialize)]
pub struct RelayerConfig {
    pub name: String,
    pub call_contract: Option<String>,
    pub rpc_addr: Option<String>,
    pub ws_addr: Option<String>,
    pub start_with_bridge: Option<bool>,
}
impl RelayerConfig {
    pub fn get_chain_id(&self) -> Option<U256> {
        let name = self.name.to_ascii_lowercase();
        SUPPORTED_CHAINS.get(name.as_str()).map(|id| id.clone())
    }
}
pub trait Relayer {}

pub async fn start_listener(
    config: RelayerConfig,
    grpc_client: Option<ScalarAbciClient<Channel>>,
    tx: mpsc::UnboundedSender<ScalarOutgoingMessage>,
) -> anyhow::Result<()> {
    //let handle = tokio::spawn(async move {});
    if let (Some(url), Some(call_contract), Some(chain_id)) = (
        config.ws_addr.as_ref(),
        config.call_contract.as_ref(),
        config.get_chain_id(),
    ) {
        info!("Start relayer with websocket url {:?}", url);
        let provider = Provider::<Ws>::connect(url.as_str())
            .await
            .expect(format!("Cannot connect to rpc url {:?}", url.as_str()).as_str());
        info!("Connected to websocket {:?} successfully", url.as_str());
        let client = Arc::new(provider);
        let address: Address = call_contract.parse()?;
        info!("Call contract {:?}", &address);
        let (tx_external_event, mut rx_external_event) = mpsc::unbounded_channel::<Vec<u8>>();
        let mut handles: Vec<JoinHandle<Result<(), anyhow::Error>>> = Vec::new();
        let chain_id_clone = chain_id.clone();
        let listener_handle: JoinHandle<Result<(), _>> = tokio::spawn(async move {
            let gateway = AxelarGateway::new(address, client.clone());
            let events = gateway.events().from_block(9794376);
            let stream = events
                .stream()
                .await
                .map_err(|e| Err::<(), anyhow::Error>(anyhow!("{:?}", e)))
                .expect("Contract error")
                .take(1);
            let bytes = Bytes::from_hex("0000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000000f48656c6c6f204176616c616e6368650000000000000000000000000000000000");
            // info!("{:?}", bytes);
            let event_value = ContractCallFilter {
                sender: Address::from_slice(
                    Vec::from_hex("00d618074b5031354854913a4cc51c1ae16c904c")
                        .unwrap()
                        .as_slice(),
                ),
                destination_chain: "Goerli".to_owned(),
                destination_contract_address: "0x81ee1a76a3869A4604eAF390E6A9793468BCA343"
                    .to_owned(),
                payload_hash: [
                    112, 101, 15, 4, 9, 145, 221, 135, 52, 181, 191, 255, 190, 2, 100, 178, 161,
                    219, 6, 255, 101, 134, 235, 14, 47, 14, 216, 50, 87, 249, 177, 71,
                ],
                payload: bytes.unwrap(),
            };
            info!("AxelarGateway event {:?}", &event_value);

            let duration = Duration::from_millis(180_000);
            loop {
                let approve_contract_param: Vec<u8> =
                    ApproveContractCallParam::from(event_value.clone()).into();
                let execute_data = ExecuteData::from_command(
                    chain_id_clone,
                    SELECTOR_APPROVE_CONTRACT_CALL,
                    approve_contract_param,
                );
                info!("Init AxelarGateway event {:?}", &execute_data);
                let message: Vec<u8> = execute_data.into();
                let res = tx_external_event.send(message);
                sleep(duration).await;
            }
            //     while let Some(Ok(evt)) = stream.next().await {
            //         match evt {
            //             AxelarGatewayEvents::ContractCallApprovedFilter(evt) => {
            //                 info!("AxelarGateway event {:?}", &evt);
            //             }
            //             AxelarGatewayEvents::ContractCallFilter(evt) => {
            //                 info!("AxelarGateway event {:?}", &evt);
            //                 let value = ContractCallFilter::from(evt);
            //                 let json = serde_json::to_string(&value).unwrap();
            //                 let _ = tx_external_event.send(json);
            //                 //ContractCallFilter { sender: , destination_chain: "Avalanche", destination_contract_address: "0x81ee1a76a3869A4604eAF390E6A9793468BCA343", payload_hash: [112, 101, 15, 4, 9, 145, 221, 135, 52, 181, 191, 255, 190, 2, 100, 178, 161, 219, 6, 255, 101, 134, 235, 14, 47, 14, 216, 50, 87, 249, 177, 71], payload: Bytes(0x0000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000000f48656c6c6f204176616c616e6368650000000000000000000000000000000000) }
            //             }
            //             AxelarGatewayEvents::ContractCallApprovedWithMintFilter(evt) => {
            //                 info!("AxelarGateway event {:?}", &evt);
            //             }
            //             AxelarGatewayEvents::ContractCallWithTokenFilter(evt) => {
            //                 info!("AxelarGateway event {:?}", &evt);
            //             }
            //             AxelarGatewayEvents::ExecutedFilter(evt) => {
            //                 info!("AxelarGateway event {:?}", &evt);
            //             }
            //             AxelarGatewayEvents::GovernanceTransferredFilter(evt) => {
            //                 info!("AxelarGateway event {:?}", &evt);
            //             }
            //             AxelarGatewayEvents::MintLimiterTransferredFilter(evt) => {
            //                 info!("AxelarGateway event {:?}", &evt);
            //             }
            //             AxelarGatewayEvents::OperatorshipTransferredFilter(evt) => {
            //                 info!("AxelarGateway event {:?}", &evt);
            //             }
            //             AxelarGatewayEvents::TokenDeployedFilter(evt) => {
            //                 info!("AxelarGateway event {:?}", &evt);
            //             }
            //             AxelarGatewayEvents::TokenMintLimitUpdatedFilter(evt) => {
            //                 info!("AxelarGateway event {:?}", &evt);
            //             }
            //             AxelarGatewayEvents::TokenSentFilter(evt) => {
            //                 info!("AxelarGateway event {:?}", &evt);
            //             }
            //             AxelarGatewayEvents::UpgradedFilter(evt) => {
            //                 info!("AxelarGateway event {:?}", &evt);
            //             }
            //         }
            //     }
            Ok(())
        });
        handles.push(listener_handle);
        if let Some(mut grpc_client) = grpc_client {
            let grpc_handle = tokio::spawn(async move {
                let in_stream = async_stream::stream! {
                    while let Some(item) = rx_external_event.recv().await {
                        //let value = serde_json::to_string(&item).unwrap();
                        //println!("Push block into stream {:?}", &value);
                        info!("Receive message from external, send it into narwhal consensus");
                        yield ScalarAbciRequest{
                            payload: item,
                        };
                    }
                };
                //pin_mut!(in_stream);
                let response = grpc_client
                    .bidirectional_streaming_scalar_abci(in_stream)
                    .await
                    .unwrap();

                let mut resp_stream = response.into_inner();

                while let Some(Ok(ScalarAbciResponse { message })) = resp_stream.next().await {
                    match message {
                        Some(Message::Ark(RequestArk { payload })) => {
                            info!("Ark message: `{}`", hex::encode(&payload));
                        }
                        Some(Message::Tran(ScalarOutTransaction { message })) => {
                            info!("Send message to the relayer: `{}`", &message);
                            let _ = tx.send(ScalarOutgoingMessage::Transaction(message));
                        }
                        Some(Message::Keygen(KeygenOutput { pub_key })) => {
                            info!("Send new tss - pubkey to the relayers: {:?}", &pub_key);
                            let _ = tx.send(ScalarOutgoingMessage::KeygenData(pub_key));
                        }
                        None => {}
                    }
                }
                Ok(())
            });
            handles.push(grpc_handle);
        }
        let _ = join_all(handles).await;

        //info!("Stop listen event from external chain");
    } else {
        warn!(
            "Missing websocket url or contract address in the config {:?}",
            &config
        );
    }
    Ok(())
}

use crate::proto::scalar_abci_response::Message;
use crate::proto::{KeygenOutput, RequestArk, ScalarAbciResponse, ScalarOutTransaction};
use crate::{create_mock_event, SELECTOR_APPROVE_CONTRACT_CALL, SUPPORTED_CHAINS};
mod evm;
mod types;

use crate::proto::{scalar_abci_client::ScalarAbciClient, ScalarAbciRequest};
use crate::relayer::types::ApproveContractCallParam;
use crate::types::ScalarOutgoingMessage;
use crate::{
    abis::{ScalarGateway, ScalarGatewayEvents},
    types::ContractCallFilter,
};
use anyhow::anyhow;
use ethers::prelude::*;
use ethers::utils::keccak256;
pub use evm::*;
use futures::future::join_all;
use serde::Deserialize;
use std::sync::Arc;
use tokio::sync::mpsc::{self, UnboundedSender};
use tokio::task::JoinHandle;
use tokio::time::{sleep, Duration};
use tonic::transport::Channel;
use tracing::{info, warn};
pub use types::*;
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
        SUPPORTED_CHAINS
            .get(name.as_str())
            .map(|id| id.clone())
            .clone()
    }
    //Return tuple of chain_id, chain_name, rpc_addr, ws_addr and contract_addr
    pub fn zip(&self) -> Option<(U256, String, String, String, String)> {
        let name = self.name.to_ascii_lowercase();
        if let (Some(chain_id), Some(rpc_addr), Some(ws_addr), Some(contract_addr)) = (
            SUPPORTED_CHAINS.get(name.as_str()),
            self.rpc_addr.as_ref(),
            self.ws_addr.as_ref(),
            self.call_contract.as_ref(),
        ) {
            Some((
                chain_id.clone(),
                self.name.clone(),
                rpc_addr.clone(),
                ws_addr.clone(),
                contract_addr.clone(),
            ))
        } else {
            None
        }
    }
}
pub trait Relayer {}

pub async fn start_listener(
    relayer: Arc<EvmRelayer>,
    mut grpc_client: ScalarAbciClient<Channel>,
    tx: mpsc::UnboundedSender<ScalarOutgoingMessage>,
) -> anyhow::Result<()> {
    //let handle = tokio::spawn(async move {});
    let config = relayer.get_config_infos().await;
    if let Some((_chain_id, chain_name, _rpc_addr, ws_addr, contract_addr)) = config {
        info!("Start relayer with websocket url {:?}", ws_addr);
        let provider = Provider::<Ws>::connect(ws_addr.as_str())
            .await
            .expect(format!("Cannot connect to rpc url {:?}", ws_addr.as_str()).as_str());
        info!("Connected to websocket {:?} successfully", ws_addr.as_str());
        let client = Arc::new(provider);
        let address: Address = contract_addr.parse()?;
        info!("Call contract {:?}", &address);
        let (tx_external_event, mut rx_external_event) = mpsc::unbounded_channel::<Vec<u8>>();
        let mut handles: Vec<JoinHandle<Result<(), anyhow::Error>>> = Vec::new();
        let listener_handle: JoinHandle<Result<(), _>> = tokio::spawn(async move {
            let chain_name_str = chain_name.as_str();
            //emit_mock_event(relayer, chain_name_str, tx_external_event.clone()).await;
            let gateway = ScalarGateway::new(address, client.clone());
            let events = gateway.events().from_block(9794376);
            let mut stream = events
                .stream()
                .await
                .map_err(|e| Err::<(), anyhow::Error>(anyhow!("{:?}", e)))
                .expect("Contract error")
                .take(1);

            while let Some(Ok(evt)) = stream.next().await {
                match evt {
                    ScalarGatewayEvents::ContractCallApprovedFilter(evt) => {
                        info!("ScalarGateway event {:?}", &evt);
                    }
                    ScalarGatewayEvents::ContractCallFilter(evt) => {
                        info!("AxelarGateway event {:?}", &evt);
                        let value = ContractCallFilter::from(evt);
                        if let Ok(message) = create_external_message(chain_name_str, &value) {
                            // relayer
                            //     .store_payload(value.payload.clone(), value.payload_hash.clone())
                            //     .await;
                            let _ = tx_external_event.send(message);
                        }
                    }
                    ScalarGatewayEvents::ContractCallApprovedWithMintFilter(evt) => {
                        info!("AxelarGateway event {:?}", &evt);
                    }
                    ScalarGatewayEvents::ContractCallWithTokenFilter(evt) => {
                        info!("AxelarGateway event {:?}", &evt);
                    }
                    ScalarGatewayEvents::ExecutedFilter(evt) => {
                        info!("AxelarGateway event {:?}", &evt);
                    }
                    ScalarGatewayEvents::GovernanceTransferredFilter(evt) => {
                        info!("AxelarGateway event {:?}", &evt);
                    }
                    ScalarGatewayEvents::MintLimiterTransferredFilter(evt) => {
                        info!("AxelarGateway event {:?}", &evt);
                    }
                    ScalarGatewayEvents::OperatorshipTransferredFilter(evt) => {
                        info!("AxelarGateway event {:?}", &evt);
                    }
                    ScalarGatewayEvents::TokenDeployedFilter(evt) => {
                        info!("AxelarGateway event {:?}", &evt);
                    }
                    ScalarGatewayEvents::TokenMintLimitUpdatedFilter(evt) => {
                        info!("AxelarGateway event {:?}", &evt);
                    }
                    ScalarGatewayEvents::TokenSentFilter(evt) => {
                        info!("AxelarGateway event {:?}", &evt);
                    }
                    ScalarGatewayEvents::UpgradedFilter(evt) => {
                        info!("AxelarGateway event {:?}", &evt);
                    }
                }
            }
            Ok(())
        });
        handles.push(listener_handle);

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
                        info!("Send a ScalarOutTransaction to the relayer");
                        let _ = tx.send(ScalarOutgoingMessage::Transaction(message));
                    }
                    Some(Message::Keygen(KeygenOutput { epoch, pub_key })) => {
                        info!(
                            "Send new tss - pubkey at the epoch {} to the relayers: {:?}",
                            epoch, &pub_key
                        );
                        let _ = tx.send(ScalarOutgoingMessage::KeygenData((epoch, pub_key)));
                    }
                    None => {}
                }
            }
            Ok(())
        });
        handles.push(grpc_handle);

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

fn create_external_message(
    chain_name: &str,
    event_value: &ContractCallFilter,
) -> anyhow::Result<Vec<u8>> {
    //Destination chain_id
    if let Some(chain_id) = SUPPORTED_CHAINS
        .get(event_value.destination_chain.to_ascii_lowercase().as_str())
        .map(|id| id.clone())
        .clone()
    {
        let payload = event_value.payload.to_vec();
        //Todo: Get real TxHash
        let tx_hash = hex::encode(keccak256(payload));
        let approve_contract_param: Vec<u8> =
            ApproveContractCallParam::from((chain_name, tx_hash, event_value)).into();
        let execute_data = ExecuteData::from_command(
            //Destination chain id
            chain_id,
            SELECTOR_APPROVE_CONTRACT_CALL,
            approve_contract_param,
        );
        let message: Vec<u8> = execute_data.into();
        Ok(message)
    } else {
        return Err(anyhow!(
            "Chain {:?} is not supported",
            &event_value.destination_chain
        ));
    }
}

async fn emit_mock_event(
    relayer: Arc<EvmRelayer>,
    chain_name: &str,
    tx_external_event: UnboundedSender<Vec<u8>>,
) {
    let event_value = create_mock_event();
    info!("ScalarGateway event {:?}", &event_value);
    let duration = Duration::from_millis(60_000);
    loop {
        if relayer.has_tss_pubkey().await {
            if let Ok(message) = create_external_message(chain_name, &event_value) {
                let _ = relayer
                    .store_payload(
                        event_value.payload.to_vec(),
                        event_value.payload_hash.clone(),
                    )
                    .await;
                let _ = tx_external_event.send(message);

                break;
            }
        } else {
            info!("Waiting for Tss's key generation");
        }
        sleep(duration).await;
    }
    info!("Stop emit event");
}

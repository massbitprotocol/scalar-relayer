use super::types::ApproveContractCallParam;
use super::types::ExecuteData;
use super::RelayerConfig;
use crate::abis::axelar_gateway::AxelarGateway;
use crate::types::ContractCallFilter;
use crate::HASH_SELECTOR_APPROVE_CONTRACT_CALL;
use crate::SELECTOR_APPROVE_CONTRACT_CALL;
use crate::{abis::axelar_auth_weighted::AxelarAuthWeighted, relayer::types::ExecuteParam};
use anyhow::anyhow;
use ethers::abi::ParamType;
use ethers::prelude::*;
use ethers::utils::hex::FromHex;
use ethers::utils::keccak256;
use k256::ecdsa::{DerSignature, Signature};
use std::ops::Add;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::info;
pub struct EvmRelayerInner {
    config: RelayerConfig,
}

impl EvmRelayerInner {
    fn new(config: RelayerConfig) -> Self {
        Self { config }
    }
    pub async fn update_tss_private_key(&mut self) -> anyhow::Result<()> {
        if let (Some(rpc_url), Some(contract_addr)) = (
            self.config.rpc_addr.as_ref(),
            self.config.call_contract.as_ref(),
        ) {
            let provider = Provider::<Http>::try_from(rpc_url)?;
            let client = Arc::new(provider);
            let address: Address = contract_addr.parse()?;
            let contract = AxelarAuthWeighted::new(address, client);
            //contract.
        }
        Ok(())
    }
    pub async fn call_destination_contract(
        &mut self,
        event_value: ContractCallFilter,
        tss_address: Address,
        signature: Vec<u8>,
    ) -> anyhow::Result<()> {
        if let (Some(rpc_url), Some(contract_addr)) = (
            self.config.rpc_addr.as_ref(),
            self.config.call_contract.as_ref(),
        ) {
            let provider = Provider::<Http>::try_from(rpc_url)?;
            let client = Arc::new(provider);
            let address: Address = contract_addr.parse()?;
            let contract = AxelarGateway::new(address, client);
            let approve_call_param = ApproveContractCallParam {
                source_chain: "Goerli".to_string(),
                source_address: event_value.sender.to_string(),
                contract_address: event_value.destination_contract_address.parse().unwrap(),
                payload_hash: event_value.payload_hash.clone(), //keccak256(event_value.payload),
                source_tx_hash: keccak256(
                    "0x69a38e71ce125c1e205a958c33a61b17de25852ae497837034ddaed60a8a33ca",
                ),
                source_event_index: U256::from(1),
            };
            let param_data: Vec<u8> = approve_call_param.into();
            info!("Approve params 0x{}", hex::encode(param_data.as_slice()));
            //info!("Approve params {:#02x?}", param_data);
            //let signature = DerSignature::try_from(signature.as_slice())?;
            //let signature = Signature::from_der(signature.as_slice());
            //How to sign before call
            //Whehe to use tss_signature
            //Create random or sequence command id
            // let command_id = keccak256(vec![42; 32]);
            // let source_chain = "Goerli".to_string();
            // let source_address = event_value.sender.to_string();
            // let contract_address = Address::from_slice(
            //     Vec::from_hex(event_value.destination_contract_address.clone())
            //         .unwrap()
            //         .as_slice(),
            // );
            // let payload_hash = keccak256(event_value.payload.clone());
            /*
             * Approve contract call
             * params: Byte array encoded from values:
             *      sourceChainName,
             *      sourceCustomContract,
             *      destinationCustomContract,
             *      payloadHash,
             *      sourceTxHash,
             *      sourceEventIndex
             * command_id: Some random 32 bytes hash
             */
            // let params = Bytes::from(vec![]);
            // let contract_call = contract.approve_contract_call(params, command_id);
            // let is_contract_call_approved = contract
            //     .is_contract_call_approved(
            //         command_id,
            //         source_chain,
            //         source_address,
            //         contract_address,
            //         payload_hash,
            //     )
            //     .call()
            //     .await;
            /**
             * @notice Executes a batch of commands signed by the Axelar network. There are a finite set of command types that can be executed.
             * @param input The encoded input containing the data for the batch of commands, as well as the proof that verifies the integrity of the data.
             * @dev Each command has a corresponding commandID that is guaranteed to be unique from the Axelar network.
             * @dev This function allows retrying a commandID if the command initially failed to be processed.
             * @dev Ignores unknown commands or duplicate commandIDs.
             * @dev Emits an Executed event for successfully executed commands.
             */
            //Note: Only this function in Axelar gateway use validateProof
            let command_ids = vec![HASH_SELECTOR_APPROVE_CONTRACT_CALL.clone()];
            let commands = vec![SELECTOR_APPROVE_CONTRACT_CALL.to_string()];
            let execute_param = self
                .config
                .get_chain_id()
                .map(|chain_id| {
                    ExecuteData::new(
                        chain_id,
                        command_ids,
                        commands,
                        vec![event_value.payload.as_ref().to_vec()],
                    )
                })
                .and_then(|data| {
                    ExecuteParam::from_tss_signature(data, tss_address, signature.clone()).ok()
                });
            if let Some(param) = execute_param {
                let param_data: Vec<u8> = param.into();
                info!("Execute params 0x{}", hex::encode(param_data.as_slice()));
                match contract.execute(Bytes::from_iter(param_data)).call().await {
                    Ok(_) => {
                        info!("Executed successfully");
                    }
                    Err(err) => {
                        info!("Executed with error {:?}", err);
                    }
                }
            }
            // let contract_call = contract.call_contract(
            //     event_value.destination_chain.clone(),
            //     event_value.destination_contract_address.clone(),
            //     event_value.payload.clone(),
            // );
            // match contract_call.call().await {
            //     Ok(_) => {
            //         info!("Contract call successfulled");
            //     }
            //     Err(e) => {
            //         info!("Contract call error {:?}", e);
            //     }
            // }
        }

        Ok(())
    }
}
pub struct EvmRelayer {
    internal: Arc<RwLock<EvmRelayerInner>>,
}

impl EvmRelayer {
    pub fn new(relayer_config: &RelayerConfig) -> Self {
        let inner = EvmRelayerInner::new(relayer_config.clone());
        Self {
            internal: Arc::new(RwLock::new(inner)),
        }
    }
    pub async fn call_destination_contract(
        &self,
        event_value: ContractCallFilter,
        tss_address: Address,
        signature: Vec<u8>,
    ) -> anyhow::Result<()> {
        let mut guard = self.internal.write().await;
        guard
            .call_destination_contract(event_value, tss_address, signature)
            .await
    }
}

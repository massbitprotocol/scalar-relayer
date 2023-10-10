use super::types::ExecuteData;
use super::types::OwnerShipData;
use super::RelayerConfig;

use crate::abis::axelar_gateway::AxelarGateway;
use crate::create_rsv_signature;
use crate::relayer::types::ExecuteParam;
use crate::relayer::types::ExecuteProof;
use crate::types::Byte32;

use crate::SELECTOR_TRANSFER_OPERATORSHIP;
use anyhow::anyhow;
use ethers::abi::Token;
use ethers::prelude::*;
use ethers::utils::keccak256;
use k256::ecdsa::{RecoveryId, Signature, VerifyingKey};
use k256::PublicKey;
use sha3::{Digest, Keccak256};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::info;

pub type MapPayload = HashMap<[u8; 32], bool>;
pub struct EvmRelayerInner {
    config: RelayerConfig,
    epoch: u64,
    pubkey: Vec<u8>,
    pubkey_hash: [u8; 32],
    send_payloads: MapPayload,
}

impl EvmRelayerInner {
    fn new(config: RelayerConfig) -> Self {
        Self {
            config,
            epoch: 0,
            pubkey: vec![],
            pubkey_hash: [0u8; 32],
            send_payloads: MapPayload::default(),
        }
    }
    pub fn get_chain_id(&self) -> Option<U256> {
        self.config.get_chain_id()
    }
    pub fn sent_payload(&mut self, payload_hash: [u8; 32]) {
        self.send_payloads.insert(payload_hash, true);
    }
    // pub fn owner_call(&self) {
    //     let wallet: LocalWallet = OWNER_PRIVATE_KEY.parse().unwrap();
    //     let rpc_url = self.config.rpc_addr.as_ref().unwrap().clone();
    //     let contract_addr = self.config.call_contract.as_ref().unwrap().clone();
    //     let provider = Provider::<Http>::try_from(rpc_url)?;
    //     let client = Arc::new(provider);
    //     let address: Address = contract_addr.parse()?;
    //     let contract = AxelarGateway::new(address, client);
    //     contract.transfer_operatorship(Bytes::from_iter(ownership_data.clone()), pubkey_hash);
    // }
    fn get_ownership_data(&self, address: &Address) -> Bytes {
        let owner = OwnerShipData::from(address.clone());
        let data: Vec<u8> = owner.into();
        info!("Ownership data 0x{}", hex::encode(data.as_slice()));
        Bytes::from_iter(data)
    }
    // pub async fn update_ownership(&mut self) -> anyhow::Result<()> {
    //     if let (Some(rpc_url), Some(contract_addr)) = (
    //         self.config.rpc_addr.as_ref(),
    //         self.config.call_contract.as_ref(),
    //     ) {
    //         let provider = Provider::<Http>::try_from(rpc_url)?;
    //         let client = Arc::new(provider);
    //         let address: Address = contract_addr.parse()?;
    //         let contract = AxelarGateway::new(address, client);

    //         if let Some(execute_param) = self.config.get_chain_id().map(|chain_id| {
    //             //Question? Which tss address put in here
    //             let param = OwnerShipData::from(TSS_ADDRESS.clone()).into();
    //             ExecuteData::from_command(chain_id, SELECTOR_TRANSFER_OPERATORSHIP, param)
    //         }) {
    //             let proof = ExecuteProof::owner_sign(&execute_param).await;
    //             let execute_param: Vec<u8> = ExecuteParam::new(execute_param, proof).into();
    //             info!("Execute params 0x{}", hex::encode(execute_param.as_slice()));
    //             match contract
    //                 .execute(Bytes::from_iter(execute_param))
    //                 .call()
    //                 .await
    //             {
    //                 Ok(_) => {
    //                     info!("Executed successfully");
    //                 }
    //                 Err(err) => {
    //                     info!("Executed with error {:?}", err);
    //                 }
    //             }
    //         }
    //     }
    //     Ok(())
    // }
    pub async fn update_pubkey(
        &mut self,
        epoch: u64,
        pubkey: Vec<u8>,
        pubkey_hash: Byte32,
    ) -> anyhow::Result<()> {
        if self.pubkey_hash == pubkey_hash && self.epoch == epoch {
            return Err(anyhow!("Pubkey already updated"));
        }
        self.epoch = epoch;
        self.pubkey = pubkey;
        self.pubkey_hash = pubkey_hash;
        let rpc_url = self.config.rpc_addr.as_ref().unwrap().clone();
        let contract_addr = self.config.call_contract.as_ref().unwrap().clone();
        let provider = Provider::<Http>::try_from(rpc_url)?;
        let client = Arc::new(provider);
        let address: Address = contract_addr.parse()?;
        let _contract = AxelarGateway::new(address, client);
        info!(
            "last 20 bytes of hash 0x{}",
            hex::encode(&self.pubkey_hash[12..])
        );
        let tss_address: Address = hex::encode(&self.pubkey_hash[12..])
            .parse()
            .map_err(|err| anyhow!("Adress parser error {:?}", &err))?;
        info!("Update new address for tss {:?}", tss_address.to_string());
        let ownership_data = OwnerShipData::from(tss_address).into();
        //Call transfer_opratorship onlySelf method
        // let wallet: LocalWallet = OWNER_PRIVATE_KEY.parse().unwrap();
        // let caller =
        //     contract.transfer_operatorship(Bytes::from_iter(ownership_data.clone()), pubkey_hash);
        // let res = caller.call().await;
        // info!("Call transfer_operatorship result {:?}", &res);
        let execute_data = self.config.get_chain_id().map(|chain_id| {
            ExecuteData::from_command(chain_id, SELECTOR_TRANSFER_OPERATORSHIP, ownership_data)
        });
        // Call gateway execute
        if let Some(execute_data) = execute_data {
            //Todo: This code work only for first epoch, from second epoch, use tss-signer instead of owner sign
            let proof = ExecuteProof::owner_sign(&execute_data).await;
            let execute_param: Vec<u8> = ExecuteParam::new(execute_data, proof).into();
            info!(
                "Execute params of update pubkey call 0x{}",
                hex::encode(execute_param.as_slice())
            );
            // match contract
            //     .execute(Bytes::from_iter(execute_param))
            //     .call()
            //     .await
            // {
            //     Ok(_) => {
            //         info!("Executed successfully");
            //     }
            //     Err(err) => {
            //         info!("Executed with error {:?}", err);
            //     }
            // }
        }

        Ok(())
    }
    fn verify_signature(&self, payload: &[u8], signature: &Signature) -> anyhow::Result<bool> {
        // assert!(pubkey
        //     .verify_prehashed(&FieldBytes::from_slice(payload_hash.as_slice()), &signature)
        //     .is_ok());
        let expected_key = VerifyingKey::from_sec1_bytes(self.pubkey.as_slice())?;
        let signature_data = signature.to_vec();
        let first_s = signature_data[32].clone();
        let v = first_s >> 7;
        let recid = RecoveryId::try_from(v)?;
        let digest = Keccak256::new_with_prefix(payload);
        let recovered_key = VerifyingKey::recover_from_digest(digest, &signature, recid)?;
        info!("Verify public key");
        assert_eq!(recovered_key, expected_key);
        let _pub_key = PublicKey::from(recovered_key);
        Ok(true)
    }
    pub async fn call_destination_contract(
        &mut self,
        payload: Vec<u8>,
        signature: Signature,
    ) -> anyhow::Result<()> {
        let payload_hash = keccak256(payload.as_slice());
        if self.send_payloads.contains_key(&payload_hash) {
            return Err(anyhow!("Payload already send to destination chain"));
        } else {
            self.sent_payload(payload_hash);
        }
        let rpc_url = self.config.rpc_addr.as_ref().unwrap().clone();
        let contract_addr = self.config.call_contract.as_ref().unwrap().clone();
        let verified = self.verify_signature(payload.as_slice(), &signature);
        info!("Verify result {:?}", &verified);
        let provider = Provider::<Http>::try_from(&rpc_url)?;
        let client = Arc::new(provider);
        let address: Address = contract_addr.parse()?;
        let contract = AxelarGateway::new(address, client);
        // let approve_call_param = ApproveContractCallParam {
        //     source_chain: "Goerli".to_string(),
        //     source_address: event_value.sender.to_string(),
        //     contract_address: event_value.destination_contract_address.parse().unwrap(),
        //     payload_hash: event_value.payload_hash.clone(), //keccak256(event_value.payload),
        //     source_tx_hash: keccak256(
        //         "0x69a38e71ce125c1e205a958c33a61b17de25852ae497837034ddaed60a8a33ca",
        //     ),
        //     source_event_index: U256::from(1),
        // };
        // let param_data: Vec<u8> = approve_call_param.into();
        info!("Approve params 0x{}", hex::encode(payload.as_slice()));
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
        //Payload is ExecuteData'serialized bytes
        let execute_proof = hex::encode(&self.pubkey_hash[12..])
            .parse()
            .map_err(|err| anyhow!("Adress parser error {:?}", &err))
            .ok()
            .map(|address| {
                let mut rsv_signature = signature.to_vec();
                create_rsv_signature(&mut rsv_signature);
                ExecuteProof::from_rsv_signature(address, rsv_signature)
            });

        if let Some(proof) = execute_proof {
            let mut tokens = vec![];
            tokens.push(Token::Bytes(payload));
            tokens.push(Token::Bytes(proof.into()));
            let param_data = ethers::abi::encode(tokens.as_slice());
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
        Ok(())
    }
}
pub struct EvmRelayer {
    internal: Arc<RwLock<EvmRelayerInner>>,
}

impl EvmRelayer {
    pub fn new(relayer_config: RelayerConfig) -> Self {
        let inner = EvmRelayerInner::new(relayer_config.clone());
        Self {
            internal: Arc::new(RwLock::new(inner)),
        }
    }
    pub async fn get_chain_id(&self) -> Option<U256> {
        self.internal.read().await.get_chain_id()
    }
    pub async fn update_pubkey(
        &self,
        epoch: u64,
        pubkey: Vec<u8>,
        pubkey_hash: Byte32,
    ) -> anyhow::Result<()> {
        let mut guard = self.internal.write().await;
        guard.update_pubkey(epoch, pubkey, pubkey_hash).await
    }
    pub async fn call_destination_contract(
        &self,
        payload: Vec<u8>,
        signature: Signature,
    ) -> anyhow::Result<()> {
        let mut guard = self.internal.write().await;
        guard.call_destination_contract(payload, signature).await
    }
}

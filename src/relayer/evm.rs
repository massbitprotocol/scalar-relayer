use super::types::ExecuteData;
use super::types::OwnerShipData;
use super::RelayerConfig;

use crate::abis::AxelarExecutable;
use crate::abis::ScalarGateway;
use crate::create_rsv_signature;
use crate::eth_hash_message;
use crate::relayer::types::ApproveContractCallParam;
use crate::relayer::types::ExecuteParam;
use crate::relayer::types::ExecuteProof;
use crate::types::Byte32;

use crate::SELECTOR_TRANSFER_OPERATORSHIP;
use anyhow::anyhow;
use ethers::abi::Token;
use ethers::prelude::*;
use k256::ecdsa::{RecoveryId, Signature, VerifyingKey};
use k256::PublicKey;
use sha3::{Digest, Keccak256};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::info;

pub type MapPayload<V> = HashMap<[u8; 32], V>;

pub struct EvmRelayerInner {
    config: RelayerConfig,
    epoch: u64,
    pubkey: Vec<u8>,
    pubkey_hash: [u8; 32],
    tss_address: Option<Address>,
    //Todo: simple design for get source's payload then send it to destination smartcontract
    payloads: MapPayload<Vec<u8>>,
    //Payload sent to destination gateway to approve
    send_payloads: MapPayload<bool>,
}

impl EvmRelayerInner {
    fn new(config: RelayerConfig) -> Self {
        Self {
            config,
            epoch: 0,
            pubkey: vec![],
            pubkey_hash: [0u8; 32],
            tss_address: None,
            payloads: MapPayload::<Vec<u8>>::default(),
            send_payloads: MapPayload::<bool>::default(),
        }
    }
    pub fn get_config_infos(&self) -> Option<(U256, String, String, String, String)> {
        self.config.zip()
    }
    pub fn get_chain_id(&self) -> Option<U256> {
        self.config.get_chain_id()
    }
    pub fn add_sent_payload(&mut self, payload_hash: [u8; 32]) {
        self.send_payloads.insert(payload_hash, true);
    }
    pub fn store_payload(&mut self, payload: Vec<u8>, payload_hash: Byte32) -> anyhow::Result<()> {
        self.payloads.insert(payload_hash, payload);
        Ok(())
    }
    fn get_payload_by_hash(&self, payload_hash: &[u8]) -> Option<&Vec<u8>> {
        self.payloads.get(payload_hash)
    }
    // pub fn owner_call(&self) {
    //     let wallet: LocalWallet = OWNER_PRIVATE_KEY.parse().unwrap();
    //     let rpc_url = self.config.rpc_addr.as_ref().unwrap().clone();
    //     let contract_addr = self.config.call_contract.as_ref().unwrap().clone();
    //     let provider = Provider::<Http>::try_from(rpc_url)?;
    //     let client = Arc::new(provider);
    //     let address: Address = contract_addr.parse()?;
    //     let contract = ScalarGateway::new(address, client);
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
    //         let contract = ScalarGateway::new(address, client);

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
        let contract = ScalarGateway::new(address, client);
        info!(
            "last 20 bytes of hash 0x{}",
            hex::encode(&self.pubkey_hash[12..])
        );
        let tss_address: Address = hex::encode(&self.pubkey_hash[12..])
            .parse()
            .map_err(|err| anyhow!("Adress parser error {:?}", &err))?;
        info!("Update new address for tss {:?}", tss_address.to_string());
        self.tss_address = Some(tss_address.clone());
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
                "Execute params of update tss address call 0x{}",
                hex::encode(execute_param.as_slice())
            );
            match contract
                .execute(Bytes::from_iter(execute_param))
                .call()
                .await
            {
                Ok(_) => {
                    info!("Executed update tss address successfully");
                }
                Err(err) => {
                    info!("Executed update tss address with error {:?}", err);
                }
            }
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
        // let hash_payload = keccak256(payload);
        // let recovered_key = VerifyingKey::recover_from_msg(&hash_payload, &signature, recid)?;
        info!("Verify public key");
        assert_eq!(recovered_key, expected_key);
        let _pub_key = PublicKey::from(recovered_key);
        Ok(true)
    }
    pub async fn call_destination_contract(
        &mut self,
        //Payload is a param of a ExecuteData with sigle command
        payload: Vec<u8>,
        signature: Signature,
    ) -> anyhow::Result<()> {
        let payload_hash = eth_hash_message(payload.as_slice());
        if self.send_payloads.contains_key(&payload_hash) {
            return Err(anyhow!("Payload already send to destination chain"));
        } else {
            self.add_sent_payload(payload_hash);
        }
        let rpc_url = self.config.rpc_addr.as_ref().unwrap().clone();
        let contract_addr = self.config.call_contract.as_ref().unwrap().clone();
        //let verified = self.verify_signature(payload.as_slice(), &signature);
        //info!("Verify result {:?}", &verified);
        let provider = Provider::<Http>::try_from(&rpc_url)?;
        let client = Arc::new(provider);
        let address: Address = contract_addr.parse()?;
        let contract = ScalarGateway::new(address, client.clone());

        //Payload is ExecuteData'serialized bytes
        let execute_data = ExecuteData::try_from(payload.as_slice());

        let execute_proof = self.tss_address.as_ref().map(|address| {
            let mut rsv_signature = signature.to_vec();
            create_rsv_signature(&mut rsv_signature);
            ExecuteProof::from_rsv_signature(address.clone(), rsv_signature)
        });

        if let (
            Ok(ExecuteData {
                chain_id: _,
                command_ids,
                commands: _,
                params,
            }),
            Some(proof),
        ) = (execute_data, execute_proof)
        {
            // 1. First call ScalarGateway.approveContractCall
            // In the method Scalar gateway mark the smartcontract is valid for call
            let mut tokens = vec![];
            tokens.push(Token::Bytes(payload));
            tokens.push(Token::Bytes(proof.into()));
            let param_data = ethers::abi::encode(tokens.as_slice());
            info!(
                "Execute params for transaction 0x{}",
                hex::encode(param_data.as_slice())
            );
            contract
                .execute(Bytes::from_iter(param_data))
                .call()
                .await
                .map_err(|err| anyhow!("Executed transaction with error {:?}", err))?;
            // 2. Call DestinationContract.execute (DestinationContract must extend ScalarExecute)
            // This method call gateway for check if method is valid for call then call execute
            if let Some(ApproveContractCallParam {
                source_chain,
                source_address,
                contract_address,
                payload_hash,
                source_tx_hash: _,
                source_event_index: _,
            }) = params
                .get(0)
                .and_then(|param| ApproveContractCallParam::try_from(param.as_slice()).ok())
            {
                let executable_contract = AxelarExecutable::new(contract_address, client);
                let command_id = command_ids.get(0).unwrap().clone();
                if let Some(contract_payload) = self.get_payload_by_hash(&payload_hash) {
                    executable_contract
                        .execute(
                            command_id,
                            source_chain,
                            source_address,
                            Bytes::from_iter(contract_payload),
                        )
                        .call()
                        .await
                        .map_err(|err| anyhow!("Call destination execute error {:?}", err))?;
                }
            }
        } else {
            return Err(anyhow!("Missing tss operator"));
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
    //Return (chain_id, chain_name, rpc_address, ws_address, contract_addr)
    pub async fn get_config_infos(&self) -> Option<(U256, String, String, String, String)> {
        self.internal.read().await.get_config_infos()
    }
    pub async fn get_chain_id(&self) -> Option<U256> {
        self.internal.read().await.get_chain_id()
    }
    pub async fn store_payload(
        &self,
        payload: Vec<u8>,
        payload_hash: Byte32,
    ) -> anyhow::Result<()> {
        let mut guard = self.internal.write().await;
        guard.store_payload(payload, payload_hash)
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

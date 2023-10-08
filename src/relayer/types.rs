use crate::{
    types::{Byte32, ContractCallFilter},
    OWNER_ADDRESS, OWNER_PRIVATE_KEY, SELECTOR_APPROVE_CONTRACT_CALL,
};
use anyhow::{anyhow, Result};
use ethers::{
    abi::{Bytes, Token},
    core::types::Signature,
    signers::{LocalWallet, Signer},
    types::{Address, Sign, U256},
    utils::keccak256,
};
use k256::ecdsa::DerSignature;
use serde::{Deserialize, Serialize};
pub struct OwnerShipData {
    operators: Vec<Address>,
    weights: Vec<U256>,
    threshold: U256,
}
impl OwnerShipData {
    // pub fn new(
    //     operators: Vec<Address>,
    //     weights: Vec<U256>,
    //     threshold: U256,
    // ) -> Self {
    //     Self {
    //         operators,
    //         weights,
    //         threshold,
    //     }
    // }
}
impl From<Address> for OwnerShipData {
    fn from(value: Address) -> Self {
        Self {
            operators: vec![value],
            weights: vec![U256::one()],
            threshold: U256::one(),
        }
    }
}
impl Into<Vec<u8>> for OwnerShipData {
    fn into(self) -> Vec<u8> {
        let Self {
            operators,
            weights,
            threshold,
        } = self;
        let mut tokens = vec![];
        tokens.push(Token::Array(
            operators
                .into_iter()
                .map(|addr| Token::Address(addr))
                .collect::<Vec<Token>>(),
        ));
        tokens.push(Token::Array(
            weights
                .into_iter()
                .map(|weight| Token::Uint(weight))
                .collect::<Vec<Token>>(),
        ));
        tokens.push(Token::Uint(threshold));
        ethers::abi::encode(tokens.as_slice())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApproveContractCallParam {
    pub source_chain: String,
    pub source_address: String,
    pub contract_address: Address,
    pub payload_hash: Byte32,
    pub source_tx_hash: Byte32,
    pub source_event_index: U256,
}
impl From<ContractCallFilter> for ApproveContractCallParam {
    fn from(value: ContractCallFilter) -> Self {
        let contract_address = value.destination_contract_address.parse().unwrap();
        Self {
            source_chain: String::default(),
            source_address: value.sender.to_string(),
            contract_address,
            payload_hash: value.payload_hash.clone(),
            source_tx_hash: keccak256("Sometransaction"),
            source_event_index: U256::one(),
        }
    }
}
impl Into<Vec<u8>> for ApproveContractCallParam {
    fn into(self) -> Vec<u8> {
        let Self {
            source_chain,
            source_address,
            contract_address,
            payload_hash,
            source_tx_hash,
            source_event_index,
        } = self;
        let mut tokens = vec![];
        tokens.push(Token::String(source_chain));
        tokens.push(Token::String(source_address));
        tokens.push(Token::Address(contract_address));
        tokens.push(Token::FixedBytes(payload_hash.to_vec()));
        tokens.push(Token::FixedBytes(source_tx_hash.to_vec()));
        tokens.push(Token::Uint(source_event_index));
        ethers::abi::encode(tokens.as_slice())
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExecuteData {
    chain_id: U256,
    command_ids: Vec<Byte32>,
    commands: Vec<String>,
    params: Vec<Bytes>,
}
impl ExecuteData {
    pub fn new(
        chain_id: U256,
        command_ids: Vec<Byte32>,
        commands: Vec<String>,
        params: Vec<Bytes>,
    ) -> Self {
        Self {
            chain_id,
            command_ids,
            commands,
            params,
        }
    }
    pub fn from_command(chain_id: U256, command: &str, param: Bytes) -> Self {
        Self {
            chain_id,
            command_ids: vec![keccak256(command.as_bytes())],
            commands: vec![String::from(command)],
            params: vec![param],
        }
    }
    pub fn set_chain_id(&mut self, chain_id: U256) {
        self.chain_id = chain_id;
    }
}

impl Into<Vec<u8>> for ExecuteData {
    fn into(self) -> Vec<u8> {
        let Self {
            chain_id,
            command_ids,
            commands,
            params,
        } = self;
        let mut tokens = vec![];
        tokens.push(Token::Uint(chain_id));
        tokens.push(Token::Array(
            command_ids
                .into_iter()
                .map(|id| Token::FixedBytes(id.to_vec()))
                .collect::<Vec<Token>>(),
        ));
        tokens.push(Token::Array(
            commands
                .into_iter()
                .map(|command| Token::String(command))
                .collect::<Vec<Token>>(),
        ));
        tokens.push(Token::Array(
            params
                .into_iter()
                .map(|param| Token::Bytes(param))
                .collect(),
        ));
        ethers::abi::encode(tokens.as_slice())
    }
}

pub struct ExecuteProof {
    operators: Vec<Address>,
    weights: Vec<U256>,
    threshold: U256,
    signatures: Vec<Bytes>,
}
impl ExecuteProof {
    pub async fn owner_sign(data: &ExecuteData) -> Self {
        let bytes: Vec<u8> = data.clone().into();
        //let wallet: LocalWallet = OWNER_WALLET;
        let wallet: LocalWallet = OWNER_PRIVATE_KEY.parse().unwrap();
        let signature: Signature = wallet.sign_message(bytes).await.unwrap();
        Self {
            operators: vec![OWNER_ADDRESS.clone()],
            weights: vec![U256::one()],
            threshold: U256::one(),
            signatures: vec![signature.to_vec()],
        }
    }
}
// impl ExecuteProof {
//     pub fn new(
//         operators: Vec<Address>,
//         weights: Vec<U256>,
//         threshold: U256,
//         signatures: Vec<Bytes>,
//     ) -> Self {
//         Self {
//             operators,
//             weights,
//             threshold,
//             signatures,
//         }
//     }
// }
impl Into<Vec<u8>> for ExecuteProof {
    fn into(self) -> Vec<u8> {
        let Self {
            operators,
            weights,
            threshold,
            signatures,
        } = self;
        let mut tokens = vec![];
        tokens.push(Token::Array(
            operators
                .into_iter()
                .map(|addr| Token::Address(addr))
                .collect::<Vec<Token>>(),
        ));
        tokens.push(Token::Array(
            weights
                .into_iter()
                .map(|weight| Token::Uint(weight))
                .collect::<Vec<Token>>(),
        ));
        tokens.push(Token::Uint(threshold));
        tokens.push(Token::Array(
            signatures
                .into_iter()
                .map(|signature| Token::Bytes(signature))
                .collect(),
        ));
        ethers::abi::encode(tokens.as_slice())
    }
}

impl TryFrom<(Address, DerSignature)> for ExecuteProof {
    type Error = anyhow::Error;
    #[inline(always)]
    fn try_from(
        (address, der_signature): (Address, DerSignature),
    ) -> Result<ExecuteProof, Self::Error> {
        // Signature::from_der(der_signature.as_bytes())
        //     .map(|signature| {
        //         let mut sig_data = signature.to_vec();
        //         sig_data.push(27_u8);
        //         ExecuteProof {
        //             operators: vec![address],
        //             weights: vec![U256::one()],
        //             threshold: U256::one(),
        //             signatures: vec![sig_data],
        //         }
        //     })
        //     .map_err(|err| anyhow!("Deserialize error {:?}", &err))
        Err(anyhow!("Testing"))
    }
}

impl ExecuteProof {
    //Create Proof from tss signature in forma ANS.1 DER
    pub fn from_tss_signature(address: Address, der_signature: Vec<u8>) -> Result<Self> {
        // Signature::from_der(der_signature.as_ref())
        //     .map(|signature| {
        //         //Signature contain only r,s component
        //         let mut sig_data = signature.to_vec();
        //         sig_data.push(27_u8);
        //         ExecuteProof {
        //             operators: vec![address],
        //             weights: vec![U256::one()],
        //             threshold: U256::one(),
        //             signatures: vec![sig_data],
        //         }
        //     })
        //     .map_err(|err| anyhow!("Deserialize error {:?}", &err))
        Err(anyhow!("Testing"))
    }
}
pub struct ExecuteParam {
    data: ExecuteData,
    proof: ExecuteProof,
}

impl ExecuteParam {
    pub fn new(data: ExecuteData, proof: ExecuteProof) -> Self {
        Self { data, proof }
    }

    pub fn from_tss_signature(
        data: ExecuteData,
        address: Address,
        der_signature: Vec<u8>,
    ) -> Result<Self> {
        ExecuteProof::from_tss_signature(address, der_signature).map(|proof| Self { data, proof })
    }
}

impl Into<Vec<u8>> for ExecuteParam {
    fn into(self) -> Vec<u8> {
        let ExecuteParam { data, proof } = self;
        let mut tokens = vec![];
        tokens.push(Token::Bytes(data.into()));
        tokens.push(Token::Bytes(proof.into()));
        ethers::abi::encode(tokens.as_slice())
    }
}

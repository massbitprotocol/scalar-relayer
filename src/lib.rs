#[macro_use]
extern crate lazy_static;

pub mod abis;
pub mod config;
pub mod grpc;
pub mod proto;
pub mod relayer;
pub mod types;
// pub const TSS_ADDRESS: &str = "scalar_tss_address";
pub const SELECTOR_BURN_TOKEN: &str = "burnToken";
pub const SELECTOR_DEPLOY_TOKEN: &str = "deployToken";
pub const SELECTOR_MINT_TOKEN: &str = "mintToken";
pub const SELECTOR_APPROVE_CONTRACT_CALL: &str = "approveContractCall";
pub const SELECTOR_APPROVE_CONTRACT_CALL_WITH_MINT: &str = "approveContractCallWithMint";
pub const SELECTOR_TRANSFER_OPERATORSHIP: &str = "transferOperatorship";
use ethers::types::{Address, U256};
use ethers::utils::keccak256;
use std::collections::HashMap;
use tiny_keccak::{Hasher, Keccak};

lazy_static! {
    /*
     * Get chain id by config name
     * https://chainlist.org/
     */
    pub static ref SUPPORTED_CHAINS: HashMap<&'static str, U256> = {
        HashMap::from([
            ("ethereum", U256::from(1)),
            ("goerli", U256::from(5)),
            ("fuji", U256::from(43113)),
            ("avalanche", U256::from(43114)),
            ("linea_testnet", U256::from(59140)),
            ("linea", U256::from(59144)),
        ])
    };
    pub static ref TSS_ADDRESS: Address = {
        let mut output = [0u8; 20];
        let mut hasher = Keccak::v256();
        hasher.update("scalar_tss_address".as_bytes());
        hasher.finalize(&mut output);
        Address::from_slice(&output)
    };
    pub static ref HASH_SELECTOR_BURN_TOKEN: [u8; 32] = keccak256("burnToken".as_bytes());
    pub static ref HASH_SELECTOR_DEPLOY_TOKEN: [u8; 32] = keccak256("deployToken");
    pub static ref HASH_SELECTOR_MINT_TOKEN: [u8; 32] = keccak256("mintToken");
    pub static ref HASH_SELECTOR_APPROVE_CONTRACT_CALL: [u8; 32] = keccak256("approveContractCall");
    pub static ref HASH_SELECTOR_APPROVE_CONTRACT_CALL_WITH_MINT: [u8; 32] =
        keccak256("approveContractCallWithMint");
    pub static ref HASH_SELECTOR_TRANSFER_OPERATORSHIP: [u8; 32] =
        keccak256("transferOperatorship");
}

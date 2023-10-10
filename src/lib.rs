#[macro_use]
extern crate lazy_static;
pub mod abis;
pub mod config;
pub mod grpc;
pub mod proto;
pub mod relayer;
pub mod types;

use ethers::types::{Address, U256};
use ethers::utils::{hex::FromHex, keccak256};
use std::collections::HashMap;
use std::env;

// pub const TSS_ADDRESS: &str = "scalar_tss_address";
pub const SELECTOR_BURN_TOKEN: &str = "burnToken";
pub const SELECTOR_DEPLOY_TOKEN: &str = "deployToken";
pub const SELECTOR_MINT_TOKEN: &str = "mintToken";
pub const SELECTOR_APPROVE_CONTRACT_CALL: &str = "approveContractCall";
pub const SELECTOR_APPROVE_CONTRACT_CALL_WITH_MINT: &str = "approveContractCallWithMint";
pub const SELECTOR_TRANSFER_OPERATORSHIP: &str = "transferOperatorship";
pub const ETH_PREFIX_HASH: &str = "\x19Ethereum Signed Message:\n32";
lazy_static! {
    pub static ref OWNER_PRIVATE_KEY: String = {
        env::var("OWNER_PRIVATE_KEY").unwrap_or_default()
    };

    /*
     * Get chain id by config name
     * https://chainlist.org/
     */
    static ref SUPPORTED_CHAINS: HashMap<&'static str, U256> = {
        HashMap::from([
            ("ethereum", U256::from(1)),
            ("goerli", U256::from(5)),
            ("fuji", U256::from(43113)),
            ("avalanche", U256::from(43114)),
            ("linea_testnet", U256::from(59140)),
            ("linea", U256::from(59144)),
        ])
    };
    // pub static ref TSS_ADDRESS: Address = {
    //     let mut output = [0u8; 20];
    //     let mut hasher = Keccak::v256();
    //     hasher.update("scalar_tss_address".as_bytes());
    //     hasher.finalize(&mut output);
    //     Address::from_slice(&output)
    // };
    static ref OWNER_ADDRESS: Address =
        Address::from_slice(
            Vec::from_hex("2F467c697798c24788086e327B0BFD25952105fe")
                .unwrap()
                .as_slice(),
        );

    static ref HASH_SELECTOR_BURN_TOKEN: [u8; 32] = keccak256("burnToken".as_bytes());
    static ref HASH_SELECTOR_DEPLOY_TOKEN: [u8; 32] = keccak256("deployToken");
    static ref HASH_SELECTOR_MINT_TOKEN: [u8; 32] = keccak256("mintToken");
    static ref HASH_SELECTOR_APPROVE_CONTRACT_CALL: [u8; 32] = keccak256("approveContractCall");
    static ref HASH_SELECTOR_APPROVE_CONTRACT_CALL_WITH_MINT: [u8; 32] =
        keccak256("approveContractCallWithMint");
    static ref HASH_SELECTOR_TRANSFER_OPERATORSHIP: [u8; 32] =
        keccak256("transferOperatorship");
}

// https://github.com/ethers-io/ethers.js/blob/v5.7/packages/bytes/src.ts/index.ts#L351
// EIP-2098; pull the v from the top bit of s and clear it
pub fn create_rsv_signature(signature: &mut Vec<u8>) {
    if signature.len() == 64 {
        // https://github.com/ethers-io/ethers.js/blob/v5.7/packages/bytes/src.ts/index.ts#L351
        // EIP-2098; pull the v from the top bit of s and clear it
        let first_s = &mut signature[32];
        let v = 27 + (first_s.clone() >> 7);
        *first_s &= 0x7f;
        signature.push(v);
    }
}

pub fn eth_hash_message(message: &[u8]) -> [u8; 32] {
    let hash = keccak256(message);
    let mut eth_message = Vec::with_capacity(ETH_PREFIX_HASH.len() + hash.len());
    eth_message.extend_from_slice(ETH_PREFIX_HASH.as_bytes());
    eth_message.extend_from_slice(&hash);
    keccak256(&eth_message)
}

use crate::abis::axelar_gateway;
use ethers::types::{Bytes, H160};
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ContractCallFilter {
    pub sender: H160,
    pub destination_chain: String,
    pub destination_contract_address: String,
    pub payload_hash: [u8; 32],
    pub payload: Bytes,
}

pub enum ScalarGatewayEvents {
    ContractCallFilter(ContractCallFilter),
    // ContractCallApprovedFilter(ContractCallApprovedFilter),
    // ContractCallApprovedWithMintFilter(ContractCallApprovedWithMintFilter),
    // ContractCallWithTokenFilter(ContractCallWithTokenFilter),
    // ExecutedFilter(ExecutedFilter),
    // GovernanceTransferredFilter(GovernanceTransferredFilter),
    // MintLimiterTransferredFilter(MintLimiterTransferredFilter),
    // OperatorshipTransferredFilter(OperatorshipTransferredFilter),
    // TokenDeployedFilter(TokenDeployedFilter),
    // TokenMintLimitUpdatedFilter(TokenMintLimitUpdatedFilter),
    // TokenSentFilter(TokenSentFilter),
    // UpgradedFilter(UpgradedFilter),
}

impl From<axelar_gateway::ContractCallFilter> for ContractCallFilter {
    fn from(value: axelar_gateway::ContractCallFilter) -> Self {
        let axelar_gateway::ContractCallFilter {
            sender,
            destination_chain,
            destination_contract_address,
            payload_hash,
            payload,
        } = value;
        ContractCallFilter {
            sender,
            destination_chain,
            destination_contract_address,
            payload_hash,
            payload,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ScalarEventTransaction {
    pub payload: Vec<u8>,
    pub tss_signature: Vec<u8>,
}

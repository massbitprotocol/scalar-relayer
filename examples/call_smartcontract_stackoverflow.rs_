use ethcontract::common::hash::keccak256;
use ethers::prelude::*;
use ethers::{contract::Contract, types::transaction::eip2718::TypedTransaction};
use hex::FromHex;
use k256::{
    ecdsa::{SigningKey, VerifyingKey},
    schnorr::signature::Verifier,
    SecretKey,
};
use scalar_relayer::{abis::ScalarGateway, OWNER_PRIVATE_KEY};
use std::sync::Arc;
use tracing::{info, Level};
use web3::ethabi::Token;
fn set_up_logs() {
    let subscriber = tracing_subscriber::fmt()
        //.with_max_level(Level::DEBUG)
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
}
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    set_up_logs();
    let execute_param = "SOMERAWDATA";
    //Working url
    let rpc_url = "https://eth-goerli.g.alchemy.com/v2/{APIKEY}";
    let contract_addr = "0x1577875Dd69f5276AEEabE3540b1F7c64690b3F8";
    let signer = OWNER_PRIVATE_KEY.parse::<LocalWallet>()?;
    info!(
        "Call from address {:?}",
        hex::encode(signer.address().as_bytes()).as_str()
    );
    let signer_middleware = Provider::<Http>::try_from(rpc_url)?.with_signer(signer.clone());
    //info!("Default sender {:?}", provider.default_sender());
    let client = Arc::new(signer_middleware);
    let address: Address = contract_addr.parse()?;
    let contract = ScalarGateway::new(address.clone(), client.clone());
    let params = Bytes::from_hex(execute_param)?;
    let contract_call = contract.execute(params);
    let res = contract_call.send().await?.await;
    info!("Call result {:?}", &res);
    Ok(())
}

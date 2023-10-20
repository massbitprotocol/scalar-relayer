use ethers::prelude::*;
use ethers::utils::rlp::Rlp;
use ethers::{contract::Contract, types::transaction::eip2718::TypedTransaction, utils::keccak256};
use hex::FromHex;
use k256::{
    ecdsa::{SigningKey, VerifyingKey},
    schnorr::signature::Verifier,
    SecretKey,
};
use scalar_relayer::{abis::ScalarGateway, OWNER_PRIVATE_KEY};
use sha3::{Digest, Sha3_256};
use std::sync::Arc;
use tiny_keccak::{Hasher, Sha3};
use tracing::{info, warn, Level};
use web3::ethabi::Token;
fn set_up_logs() {
    let subscriber = tracing_subscriber::fmt()
        //.with_max_level(Level::DEBUG)
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
}
//Successfully call to blockchain
const SIGNED_NODE_TX: &str = "02f90570052c8459682f008459682f1682cfe4941577875dd69f5276aeeabe3540b1f7c64690b3f880b9050409c5eabe000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000004c0000000000000000000000000000000000000000000000000000000000000004000000000000000000000000000000000000000000000000000000000000002e000000000000000000000000000000000000000000000000000000000000002800000000000000000000000000000000000000000000000000000000000000005000000000000000000000000000000000000000000000000000000000000008000000000000000000000000000000000000000000000000000000000000000c000000000000000000000000000000000000000000000000000000000000001400000000000000000000000000000000000000000000000000000000000000001cecce1a2e9fd648d0228ef25a5f99d31a68c8c18fc370f893e091bc19e6f61470000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000000147472616e736665724f70657261746f72736869700000000000000000000000000000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000000e0000000000000000000000000000000000000000000000000000000000000006000000000000000000000000000000000000000000000000000000000000000a00000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000000100000000000000000000000046a4cad7a08062f5a67c5bc4aa6d8c8816eff2470000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000001c0000000000000000000000000000000000000000000000000000000000000008000000000000000000000000000000000000000000000000000000000000000c00000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000000010000000000000000000000002f467c697798c24788086e327b0bfd25952105fe000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000000414aaeab55123571f2fae033d6d95b814975601c91470bdaf0c5c73c67cc22098d0b03cffdd2b0df7fb329a48bd80679ea17a78f992de05101f996afa169bbe0471c00000000000000000000000000000000000000000000000000000000000000c080a017d4b0603b7e63924d69fb1ef1fa62df1a51f22f1b60a1e6bbecbe85284ac741a06609510c7934ef41a6851a2e94cb58288b1b29a106ec8b864a80464a4811d635";
const NODE_RAW_INPUT: &str = "02f9052d052c8459682f008459682f1682cfe4941577875dd69f5276aeeabe3540b1f7c64690b3f880b9050409c5eabe000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000004c0000000000000000000000000000000000000000000000000000000000000004000000000000000000000000000000000000000000000000000000000000002e000000000000000000000000000000000000000000000000000000000000002800000000000000000000000000000000000000000000000000000000000000005000000000000000000000000000000000000000000000000000000000000008000000000000000000000000000000000000000000000000000000000000000c000000000000000000000000000000000000000000000000000000000000001400000000000000000000000000000000000000000000000000000000000000001cecce1a2e9fd648d0228ef25a5f99d31a68c8c18fc370f893e091bc19e6f61470000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000000147472616e736665724f70657261746f72736869700000000000000000000000000000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000000e0000000000000000000000000000000000000000000000000000000000000006000000000000000000000000000000000000000000000000000000000000000a00000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000000100000000000000000000000046a4cad7a08062f5a67c5bc4aa6d8c8816eff2470000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000001c0000000000000000000000000000000000000000000000000000000000000008000000000000000000000000000000000000000000000000000000000000000c00000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000000010000000000000000000000002f467c697798c24788086e327b0bfd25952105fe000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000000414aaeab55123571f2fae033d6d95b814975601c91470bdaf0c5c73c67cc22098d0b03cffdd2b0df7fb329a48bd80679ea17a78f992de05101f996afa169bbe0471c00000000000000000000000000000000000000000000000000000000000000c0";
const NODE_SIGNATURE: &str = "80a017d4b0603b7e63924d69fb1ef1fa62df1a51f22f1b60a1e6bbecbe85284ac741a06609510c7934ef41a6851a2e94cb58288b1b29a106ec8b864a80464a4811d635";
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    set_up_logs();
    let execute_param ="000000000000000000000000000000000000000000000000000000000000004000000000000000000000000000000000000000000000000000000000000002e000000000000000000000000000000000000000000000000000000000000002800000000000000000000000000000000000000000000000000000000000000005000000000000000000000000000000000000000000000000000000000000008000000000000000000000000000000000000000000000000000000000000000c000000000000000000000000000000000000000000000000000000000000001400000000000000000000000000000000000000000000000000000000000000001cecce1a2e9fd648d0228ef25a5f99d31a68c8c18fc370f893e091bc19e6f61470000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000000147472616e736665724f70657261746f72736869700000000000000000000000000000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000000e0000000000000000000000000000000000000000000000000000000000000006000000000000000000000000000000000000000000000000000000000000000a00000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000000100000000000000000000000046a4cad7a08062f5a67c5bc4aa6d8c8816eff2470000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000001c0000000000000000000000000000000000000000000000000000000000000008000000000000000000000000000000000000000000000000000000000000000c00000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000000010000000000000000000000002f467c697798c24788086e327b0bfd25952105fe000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000000414aaeab55123571f2fae033d6d95b814975601c91470bdaf0c5c73c67cc22098d0b03cffdd2b0df7fb329a48bd80679ea17a78f992de05101f996afa169bbe0471c00000000000000000000000000000000000000000000000000000000000000";
    let node_raw_data = "02f90570052b8459682f008459682f1882cfe4941577875dd69f5276aeeabe3540b1f7c64690b3f880b9050409c5eabe000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000004c0000000000000000000000000000000000000000000000000000000000000004000000000000000000000000000000000000000000000000000000000000002e000000000000000000000000000000000000000000000000000000000000002800000000000000000000000000000000000000000000000000000000000000005000000000000000000000000000000000000000000000000000000000000008000000000000000000000000000000000000000000000000000000000000000c000000000000000000000000000000000000000000000000000000000000001400000000000000000000000000000000000000000000000000000000000000001cecce1a2e9fd648d0228ef25a5f99d31a68c8c18fc370f893e091bc19e6f61470000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000000147472616e736665724f70657261746f72736869700000000000000000000000000000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000000e0000000000000000000000000000000000000000000000000000000000000006000000000000000000000000000000000000000000000000000000000000000a00000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000000100000000000000000000000046a4cad7a08062f5a67c5bc4aa6d8c8816eff2470000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000001c0000000000000000000000000000000000000000000000000000000000000008000000000000000000000000000000000000000000000000000000000000000c00000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000000010000000000000000000000002f467c697798c24788086e327b0bfd25952105fe000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000000414aaeab55123571f2fae033d6d95b814975601c91470bdaf0c5c73c67cc22098d0b03cffdd2b0df7fb329a48bd80679ea17a78f992de05101f996afa169bbe0471c00000000000000000000000000000000000000000000000000000000000000c0";

    let serialized_sig = "01a08d1cbdbc11eba3b08df739d4ffd219aa8443d27cb719ba8a624caa3301b33222a03bf397ad686ff1230165dfd58a2f808149498ebbde09c2a039d3bf594473c362";
    let rpc_url = "https://eth-goerli.g.alchemy.com/v2/DpCscOiv_evEPscGYARI3cOVeJ59CRo8";
    let contract_addr = "0x1577875Dd69f5276AEEabE3540b1F7c64690b3F8";
    let sender_addr = "0x2F467c697798c24788086e327B0BFD25952105fe";
    let sender_addr = sender_addr.parse::<Address>()?;
    let signer = OWNER_PRIVATE_KEY.parse::<LocalWallet>()?;
    info!(
        "Call from address {:?}",
        hex::encode(signer.address().as_bytes()).as_str()
    );
    assert_eq!(signer.address(), sender_addr);
    let provider = Provider::<Http>::try_from(rpc_url)?;
    let signer_middleware = provider.with_signer(signer.clone());
    let signing_key =
        SigningKey::from_slice(hex::decode(OWNER_PRIVATE_KEY.as_bytes())?.as_slice())?;
    let verifying_key = signing_key.verifying_key();
    //info!("Default sender {:?}", provider.default_sender());
    let client = Arc::new(signer_middleware);
    let address: Address = contract_addr.parse()?;
    let contract = ScalarGateway::new(address.clone(), client.clone());
    let abi = contract.abi();
    let payload = hex::decode(execute_param)?;
    let params = Bytes::from_hex(execute_param)?;
    info!("Params {:?}", &params);
    let mut contract_call = contract.execute(params);
    let _ = fill_transaction(&client, &mut contract_call).await;
    let signed_tx = create_signed_transaction(&signer, &contract_call)?;
    info!("Signed tx {:?}", signed_tx);
    let res: Option<TransactionReceipt> = client.send_raw_transaction(signed_tx).await?.await?;
    info!("Call result {:?}", res);
    // decode_rlp();
    // info!("Decoded input {:?}", decoded_input.as_slice());
    // let tx_hash = keccak256(decoded_input);
    // info!("Keccak tx_hash {:?}", tx_hash);
    // let tx_hash = H256::from_slice(&tx_hash);
    // info!("tx_hash {:?}", hex::encode(tx_hash));
    // //let raw_data = Bytes::from_hex(node_raw_data)?;
    // if let Ok(mut signature) = signer.sign_hash(tx_hash) {
    //     info!("Signature {}", &signature);
    //     let raw_params = contract_call.tx.rlp_signed(&signature);
    //     let res: Option<TransactionReceipt> =
    //         client.send_raw_transaction(raw_params).await?.await?;
    //     info!("Call result {:?}", res);
    //     if signature
    //         .verify(
    //             RecoveryMessage::Data(NODE_RAW_INPUT.as_bytes().to_vec()),
    //             sender_addr.clone(),
    //         )
    //         .is_ok()
    //     {
    //         //adjust signature for rlp

    //         // adjust_signature(&contract_call.tx, &mut signature);
    //         // info!("Adjusted Signature {:?}", &signature);
    //         // let raw_params = contract_call.tx.rlp_signed(&signature);
    //         // info!("Raw params {:?}", &raw_params);
    //         //let raw_params = Bytes::from_hex(NODE_RAW_INPUT)?;
    //         // let res: Option<TransactionReceipt> =
    //         //     client.send_raw_transaction(raw_params).await?.await?;
    //         // info!("Call result {:?}", res);
    //     } else {
    //         warn!("Recovered address do not match")
    //     }
    // }

    // if let Ok(signature) = signer.sign_transaction(&contract_call.tx).await {
    //     let verify = signature.verify(
    //         RecoveryMessage::Data(raw_data.as_ref().to_vec()),
    //         sender_addr,
    //     );
    //     info!("Verify {:?}", verify);
    //     info!("Signature {}", &signature);
    //     info!("Signature {:?}", &signature);

    //     // extracts the chainid from the signature v value based on EIP-155
    //     let raw_params = contract_call.tx.rlp_signed(&signature);
    //     info!("Raw params {:?}", &raw_params);
    //     // let node_raw_params = "02f90570052a8459682f008459682f1282cfe4941577875dd69f5276aeeabe3540b1f7c64690b3f880b9050409c5eabe000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000004c0000000000000000000000000000000000000000000000000000000000000004000000000000000000000000000000000000000000000000000000000000002e000000000000000000000000000000000000000000000000000000000000002800000000000000000000000000000000000000000000000000000000000000005000000000000000000000000000000000000000000000000000000000000008000000000000000000000000000000000000000000000000000000000000000c000000000000000000000000000000000000000000000000000000000000001400000000000000000000000000000000000000000000000000000000000000001cecce1a2e9fd648d0228ef25a5f99d31a68c8c18fc370f893e091bc19e6f61470000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000000147472616e736665724f70657261746f72736869700000000000000000000000000000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000000e0000000000000000000000000000000000000000000000000000000000000006000000000000000000000000000000000000000000000000000000000000000a00000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000000100000000000000000000000046a4cad7a08062f5a67c5bc4aa6d8c8816eff2470000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000001c0000000000000000000000000000000000000000000000000000000000000008000000000000000000000000000000000000000000000000000000000000000c00000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000000010000000000000000000000002f467c697798c24788086e327b0bfd25952105fe000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000000414aaeab55123571f2fae033d6d95b814975601c91470bdaf0c5c73c67cc22098d0b03cffdd2b0df7fb329a48bd80679ea17a78f992de05101f996afa169bbe0471c00000000000000000000000000000000000000000000000000000000000000c001a08d1cbdbc11eba3b08df739d4ffd219aa8443d27cb719ba8a624caa3301b33222a03bf397ad686ff1230165dfd58a2f808149498ebbde09c2a039d3bf594473c362";
    //     // info!("Raw params {}", &node_raw_params);
    //     // let node_raw_params = hex::decode(node_raw_params)?;
    //     // let node_raw_params = Bytes::from(node_raw_params);
    //     // info!("Raw params {:?}", &node_raw_params);
    //     // let res: Option<TransactionReceipt> =
    //     //     client.send_raw_transaction(raw_params).await?.await?;
    //     // info!("Call result {:?}", res);
    // }
    // if let Ok(gas) = contract_call.estimate_gas().await {
    //     contract_call = contract_call.gas(gas);
    // }

    // let calldata = contract_call.calldata();
    // info!("Call data {:?}", &calldata);
    // info!("TxTransaction {:?}", &contract_call.tx);
    // info!("Bytes {}", contract_call.tx.rlp());
    // let sighash = contract_call.tx.sighash();
    // info!("Sighash {}", sighash.to_string());
    // let signature = signer.sign_transaction(&contract_call.tx).await?;
    // info!("Signature {}", &signature);

    // let pending_tx = contract_call.send().await?;

    // match pending_tx.await {
    //     Ok(res) => {
    //         info!("Executed update tss address successfully {:?}", res);
    //     }
    //     Err(err) => {
    //         info!("Executed update tss address with error {:?}", err);
    //     }
    // }
    Ok(())
}
async fn fill_transaction(
    client: &Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
    contract_call: &mut FunctionCall<
        Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
        SignerMiddleware<Provider<Http>, Wallet<SigningKey>>,
        (),
    >,
) -> anyhow::Result<()> {
    let _ = client
        .fill_transaction(&mut contract_call.tx, contract_call.block.clone())
        .await;
    contract_call.tx.set_value(U256::zero());
    //signer_middleware.get_chainid().await;
    contract_call.tx.set_chain_id(U64::from(5));
    info!("Filled transaction {:?}", &contract_call.tx);
    Ok(())
}
fn create_signed_transaction(
    signer: &Wallet<SigningKey>,
    contract_call: &FunctionCall<
        Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
        SignerMiddleware<Provider<Http>, Wallet<SigningKey>>,
        (),
    >,
) -> anyhow::Result<Bytes> {
    let encoded = contract_call.tx.rlp();
    info!("Encoded tx {:?}", &encoded);
    let tx_hash = contract_call.tx.sighash();
    info!("TxHash {:?}", &tx_hash);
    let mut signature = signer.sign_hash(tx_hash)?;
    info!("Signature {}", &signature);
    adjust_signature(&contract_call.tx, &mut signature);
    let signed_tx = contract_call.tx.rlp_signed(&signature);
    decode_signed_rlp(&signed_tx);
    Ok(signed_tx)
}
fn decode_signed_rlp(signed_tx: &Bytes) -> anyhow::Result<()> {
    info!("Try decode encoded transaction");
    let rlp = Rlp::new(signed_tx.as_ref());
    let (tx, sig) = TypedTransaction::decode_signed(&rlp)?;
    info!("Transaction {:?}", &tx);
    info!("Transaction rlp {:?}", tx.rlp());
    info!("Signature {}", &sig);
    Ok(())
}
// https://emn178.github.io/online-tools/keccak_256.html
pub fn sha3_256<T: AsRef<[u8]>>(bytes: T) -> [u8; 32] {
    let mut hasher = Sha3_256::new();

    // write input message
    hasher.update(bytes.as_ref());

    // read hash digest
    let result = hasher.finalize();
    result.into()
    // let mut output = [0u8; 32];
    // let mut hasher = Sha3::v256();
    // hasher.update(bytes.as_ref());
    // hasher.finalize(&mut output);

    // output
}
fn adjust_signature(tx: &TypedTransaction, signature: &mut Signature) {
    let chain_id = tx.chain_id().unwrap_or_else(U64::one).as_u64();
    match tx {
        TypedTransaction::Legacy(ref tx) => {}
        TypedTransaction::Eip2930(_) => {
            if signature.v >= 27 {
                //signature.v = signature.v + 35 + 2 * chain_id - 27;
                signature.v = signature.v - 27;
            }
        }
        TypedTransaction::Eip1559(_) => {
            if signature.v >= 27 {
                //signature.v = signature.v + 35 + 2 * chain_id - 27;
                signature.v = signature.v - 27;
            }
        }
        #[cfg(feature = "optimism")]
        TypedTransaction::OptimismDeposited(inner) => {
            if signature.v >= 27 {
                //signature.v = signature.v + 35 + 2 * chain_id - 27;
                signature.v = signature.v - 27;
            }
        }
    };
}

//""

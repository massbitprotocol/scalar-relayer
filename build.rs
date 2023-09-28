use std::io::Error;

use ethers::prelude::Abigen;
use ethers_solc::utils;
const SMART_CONTRACT_PATH: &str = "./solidity/artifacts/contracts/call-contract";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    build_proto().expect("Failed to compile proto(s)");
    abi_generator("MessageSender", "message_sender.rs")?;
    abi_generator("MessageReceiver", "message_receiver.rs")?;
    match abi_generator("ExecutableSample", "executable_sample.rs") {
        Ok(_) => {
            println!("Generate abis successfully");
        }
        Err(e) => {
            panic!("Generate abis error {:?}", &e);
        }
    }
    Ok(())
}

fn build_proto() -> Result<(), Error> {
    tonic_build::configure()
        .out_dir("src/proto")
        .compile(&["proto/abci.proto"], &["proto"])
}

fn abi_generator(contract: &str, output_file: &str) -> eyre::Result<()> {
    let abi_source = format!("{}/{}.sol/{}.json", SMART_CONTRACT_PATH, contract, contract);
    //"./solidity/artifacts/contracts/call-contract/ExecutableSample.sol/ExecutableSample.json";
    let root = utils::canonicalize(env!("CARGO_MANIFEST_DIR"))?.join("src/abis");
    let out_file = root.join(output_file);
    if out_file.exists() {
        std::fs::remove_file(&out_file)?;
    }
    Abigen::new(contract, abi_source)?
        .generate()?
        .write_to_file(out_file)?;
    Ok(())
}

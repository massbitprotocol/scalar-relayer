use std::io::Error;

use ethers::prelude::Abigen;
use ethers_solc::utils;
const SMART_CONTRACT_PATH: &str = "./solidity/artifacts/contracts";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    build_proto().expect("Failed to compile proto(s)");
    abi_generator("AxelarAuthWeighted", "axelar_auth_weighted.rs", "/auth")?;
    abi_generator("AxelarGateway", "axelar_gateway.rs", "")?;
    abi_generator("AxelarGatewayProxy", "axelar_gateway_proxy.rs", "")?;
    // match abi_generator("ExecutableSample", "executable_sample.rs") {
    //     Ok(_) => {
    //         println!("Generate abis successfully");
    //     }
    //     Err(e) => {
    //         panic!("Generate abis error {:?}", &e);
    //     }
    // }
    Ok(())
}

fn build_proto() -> Result<(), Error> {
    tonic_build::configure()
        .out_dir("src/proto")
        .compile(&["proto/abci.proto"], &["proto"])
}

fn abi_generator(contract: &str, output_file: &str, subdir: &str) -> eyre::Result<()> {
    let abi_source = format!(
        "{}{}/{}.sol/{}.json",
        SMART_CONTRACT_PATH, subdir, contract, contract
    );
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

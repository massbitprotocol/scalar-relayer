use std::io::Error;

use ethers::prelude::Abigen;
use ethers_solc::utils;
const SMART_CONTRACT_PATH: &str = "./solidity/artifacts";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    build_proto().expect("Failed to compile proto(s)");
    abi_generator(
        "ScalarAuthWeighted",
        "scalar_auth_weighted.rs",
        "/contracts/auth",
    )?;
    abi_generator("ScalarGateway", "scalar_gateway.rs", "/contracts")?;
    abi_generator(
        "ScalarGatewayProxy",
        "scalar_gateway_proxy.rs",
        "/contracts",
    )?;
    abi_generator(
        "AxelarExecutable",
        "axelar_executable.rs",
        "/@axelar-network/axelar-gmp-sdk-solidity/contracts/executable",
    )?;
    //Using ethcontract
    //build_ethcontract
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

// fn build_ethcontract() -> Result<(), Error> {
//     let out_dir = std::env::var("OUT_DIR").unwrap();
//     let dest = std::path::Path::new(&out_dir).join("contracts.rs");

//     let artifact = HardHatLoader::new()
//         .deny_network_by_name("localhost")
//         .load_from_directory("../hardhat/deployments")
//         .unwrap();

//     for contract in artifact.iter() {
//         ContractBuilder::new()
//             .generate(contract)
//             .unwrap()
//             .write_to_file(&dest)
//             .unwrap();
//     }
//     Ok(())
// }
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

use ethers::{types::U256, utils::keccak256};
use k256::{
    ecdsa::{
        signature::{hazmat::PrehashVerifier, DigestVerifier, Verifier},
        RecoveryId, VerifyingKey,
    },
    schnorr::signature::Keypair,
};
use scalar_relayer::{eth_hash_message, ETH_PREFIX_HASH, OWNER_PRIVATE_KEY};
use scalar_relayer::{eth_message, relayer::ExecuteData};
use sha3::Digest;
use sha3::Keccak256;
const TSS_PUBKEY: [u8; 33] = [
    3, 202, 230, 112, 205, 173, 128, 25, 230, 39, 147, 35, 228, 190, 233, 216, 77, 214, 10, 103,
    150, 88, 50, 80, 82, 155, 160, 86, 62, 212, 81, 50, 125,
];
const TSS_PUBKEY_HEX: &str = "0x03cae670cdad8019e6279323e4bee9d84dd60a6796583250529ba0563ed451327d";
const TSS_UNCOMPRESS_PUBKEY : &str = "0x04cae670cdad8019e6279323e4bee9d84dd60a6796583250529ba0563ed451327dbf523d9a6df0927dc49b5e80dac064f43793d2e2bfb1c576f31c2c0e1e4535fd";
const TSS_UNCOMPRESS_PUBKEY_HASH: &str =
    "0x975f49a1598b2c7563129b40bdcbc60b3b510b7c3f688c24c666a24c8800bea1";
const TSS_ADDRESS: &str = "0xbdcbc60b3b510b7c3f688c24c666a24c8800bea1";

const INPUT_ETH_HASH: &str = "0x31f5c604263eba0c2e3ada494b6e939bd74b639f5db5f47dea26f5e63b83c9e8";
const TSS_SIGNATURE_ARR: [u8; 71] = [
    48, 69, 2, 33, 0, 191, 253, 160, 28, 140, 150, 128, 178, 133, 178, 135, 165, 151, 92, 30, 11,
    134, 159, 232, 5, 239, 69, 227, 195, 132, 226, 134, 238, 67, 135, 70, 176, 2, 32, 60, 143, 68,
    95, 50, 158, 154, 31, 230, 220, 163, 101, 144, 188, 204, 230, 199, 11, 56, 91, 148, 150, 15,
    255, 157, 254, 1, 28, 235, 230, 174, 181,
];
//k256::ecdsa::Signature without v component
const TSS_SIGNATURE: &str = "0xbffda01c8c9680b285b287a5975c1e0b869fe805ef45e3c384e286ee438746b03c8f445f329e9a1fe6dca36590bccce6c70b385b94960fff9dfe011cebe6aeb5";
const TSS_RSV_SIG: &str = "0x00bffda01c8c9680b285b287a5975c1e0b869fe805ef45e3c384e286ee438746b03c8f445f329e9a1fe6dca36590bccce6c70b385b94960fff9dfe011cebe6aeb5";

const SIGNED_TX : &str = "0x02f905ce053984b2d05e0084b2d05e1280941577875dd69f5276aeeabe3540b1f7c64690b3f880b9056409c5eabe000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000005200000000000000000000000000000000000000000000000000000000000000040000000000000000000000000000000000000000000000000000000000000034000000000000000000000000000000000000000000000000000000000000002e00000000000000000000000000000000000000000000000000000000000000005000000000000000000000000000000000000000000000000000000000000008000000000000000000000000000000000000000000000000000000000000000c000000000000000000000000000000000000000000000000000000000000001400000000000000000000000000000000000000000000000000000000000000001efbecbcace0a2d3c164f4b5cbfb13c4626353e009d12abe04858ea576e2e5d23000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000013617070726f7665436f6e747261637443616c6c0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000014000000000000000000000000000000000000000000000000000000000000000c00000000000000000000000000000000000000000000000000000000000000100000000000000000000000000dc4a108e0cb62c22931209e8deebbaa49522670070650f040991dd8734b5bfffbe0264b2a1db06ff6586eb0e2f0ed83257f9b1475c8b1bab3c7e966373d9efa25239237c08906c7b89fd09f558bf70242864e1d200000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000006476f65726c690000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000d307833306330e280a6353735300000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001c0000000000000000000000000000000000000000000000000000000000000008000000000000000000000000000000000000000000000000000000000000000c0000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000000001000000000000000000000000bdcbc60b3b510b7c3f688c24c666a24c8800bea10000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000004100bffda01c8c9680b285b287a5975c1e0b869fe805ef45e3c384e286ee438746b03c8f445f329e9a1fe6dca36590bccce6c70b385b94960fff9dfe011cebe6aeb500000000000000000000000000000000000000000000000000000000000000c080a05bcd2e0e5d838f2dbba1f517b686e41b7b74903a4045eff8d67633a0c0a4d4efa068105bc5a85b372adddb7abe621e575c7333ae06c9cfca556527590b570d3cfc";

#[tokio::test]
async fn tss_pubkey() {
    assert!(true);
}

#[tokio::test]
async fn tss_signature() {
    let execute_data = ExecuteData {
        chain_id: U256::from(5),
        command_ids: vec![[
            239, 190, 203, 202, 206, 10, 45, 60, 22, 79, 75, 92, 191, 177, 60, 70, 38, 53, 62, 0,
            157, 18, 171, 224, 72, 88, 234, 87, 110, 46, 93, 35,
        ]],
        commands: vec!["approveContractCall".to_string()],
        params: vec![vec![
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 192, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 220, 74, 16, 142, 12, 182, 44,
            34, 147, 18, 9, 232, 222, 235, 186, 164, 149, 34, 103, 0, 112, 101, 15, 4, 9, 145, 221,
            135, 52, 181, 191, 255, 190, 2, 100, 178, 161, 219, 6, 255, 101, 134, 235, 14, 47, 14,
            216, 50, 87, 249, 177, 71, 92, 139, 27, 171, 60, 126, 150, 99, 115, 217, 239, 162, 82,
            57, 35, 124, 8, 144, 108, 123, 137, 253, 9, 245, 88, 191, 112, 36, 40, 100, 225, 210,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 6, 71, 111, 101, 114, 108, 105, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 48, 120, 51, 48, 99, 48, 226, 128,
            166, 53, 55, 53, 48, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ]],
    };
    //info!("{:?}", OWNER_PRIVATE_KEY);
    let payload: Vec<u8> = execute_data.into();
    //Add eth signed message prefix
    let eth_message = eth_message(&payload.as_slice());
    let payload_eth_hash = eth_hash_message(payload.as_slice());
    let signature = k256::ecdsa::Signature::from_der(&TSS_SIGNATURE_ARR);
    assert!(signature.is_ok());
    let signature = signature.unwrap();
    let verifying_key = VerifyingKey::from_sec1_bytes(&TSS_PUBKEY).unwrap();
    assert!(verifying_key
        .verify_prehash(&payload_eth_hash, &signature)
        .is_ok());

    let mut digest = Keccak256::new_with_prefix(ETH_PREFIX_HASH.as_bytes());
    digest.update(keccak256(payload.as_slice()));
    assert!(verifying_key.verify_digest(digest, &signature).is_ok());
    let digest = Keccak256::new_with_prefix(eth_message.as_slice());
    assert!(verifying_key.verify_digest(digest, &signature).is_ok());
    let recovery_id =
        RecoveryId::trial_recovery_from_prehash(&verifying_key, &payload_eth_hash, &signature);
    assert!(recovery_id.is_ok());
    // assert!(verifying_key
    //     .verify(eth_message.as_slice(), &signature)
    //     .is_ok());
}

// fn verify_signature(&self, payload: &[u8], signature: &Signature) -> anyhow::Result<bool> {
//     // assert!(pubkey
//     //     .verify_prehashed(&FieldBytes::from_slice(payload_hash.as_slice()), &signature)
//     //     .is_ok());
//     let expected_key = VerifyingKey::from_sec1_bytes(self.pubkey.as_slice())?;
//     let signature_data = signature.to_vec();
//     let first_s = signature_data[32].clone();
//     let v = first_s >> 7;
//     let recid = RecoveryId::try_from(v)?;
//     let digest = Keccak256::new_with_prefix(payload);
//     let recovered_key = VerifyingKey::recover_from_digest(digest, &signature, recid)?;
//     // let hash_payload = keccak256(payload);
//     // let recovered_key = VerifyingKey::recover_from_msg(&hash_payload, &signature, recid)?;
//     info!("Verify public key");
//     assert_eq!(recovered_key, expected_key);
//     let _pub_key = PublicKey::from(recovered_key);
//     Ok(true)
// }
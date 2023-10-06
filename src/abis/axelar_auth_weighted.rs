pub use axelar_auth_weighted::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types,
)]
pub mod axelar_auth_weighted {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("recentOperators"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Array(
                            ::std::boxed::Box::new(
                                ::ethers::core::abi::ethabi::ParamType::Bytes,
                            ),
                        ),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("bytes[]"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("acceptOwnership"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("acceptOwnership"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("currentEpoch"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("currentEpoch"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("epochForHash"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("epochForHash"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("hashForEpoch"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("hashForEpoch"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("owner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("owner"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("owner_"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("pendingOwner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("pendingOwner"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("owner_"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("proposeOwnership"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("proposeOwnership"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("transferOperatorship"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "transferOperatorship",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("params"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("transferOwnership"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("transferOwnership"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("validateProof"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("validateProof"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("messageHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("proof"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("OperatorshipTransferred"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OperatorshipTransferred",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newOperators"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newWeights"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newThreshold"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OwnershipTransferStarted"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OwnershipTransferStarted",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OwnershipTransferred"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OwnershipTransferred",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
            ]),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("DuplicateOperators"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("DuplicateOperators"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidOperators"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidOperators"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidOwner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidOwner"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidOwnerAddress"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InvalidOwnerAddress",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidS"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidS"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidSignature"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidSignature"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidSignatureLength"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InvalidSignatureLength",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidThreshold"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidThreshold"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidV"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidV"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidWeights"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidWeights"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("LowSignaturesWeight"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "LowSignaturesWeight",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("MalformedSigners"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("MalformedSigners"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NotOwner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("NotOwner"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static AXELARAUTHWEIGHTED_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[P`@Qb\0\x18\xD18\x03\x80b\0\x18\xD1\x839\x81\x01`@\x81\x90Rb\0\x004\x91b\0\x04\x18V[3b\0\0@\x81b\0\0\x98V[P\x80Q`\0[\x81\x81\x10\x15b\0\0\x8FWb\0\0|\x83\x82\x81Q\x81\x10b\0\0hWb\0\0hb\0\x05VV[` \x02` \x01\x01Qb\0\x01<` \x1B` \x1CV[b\0\0\x87\x81b\0\x05\x82V[\x90Pb\0\0FV[PPPb\0\x07\xA0V[`\x01`\x01`\xA0\x1B\x03\x81\x16b\0\0\xC0W`@Qc6I9}`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Q`\x01`\x01`\xA0\x1B\x03\x82\x16\x90\x7F\x04\xDB\xA6\"\xD2\x84\xED\0\x14\xEEK\x9Ajh8k\xE1\xA4\xC0\x8AI\x13\xAE'-\xE8\x91\x99\xCChac\x90`\0\x90\xA2\x7F\x02\x01h6\xA5kq\xF0\xD0&\x89\xE6\x9E2oOL\x1B\x90W\x16N\xF5\x92g\x1C\xF0\xD3|\x80@\xC0U`\0\x7F\x98U8A\"\xB5Y6\xFB\xFB\x8C\xA5\x12\x0Ec\xC6Sz\x1A\xC4\x0C\xAFj\xE35\x02\xB3\xC5\xDA\x8C\x87\xD1UV[`\0\x80`\0\x83\x80` \x01\x90Q\x81\x01\x90b\0\x01W\x91\x90b\0\x06\rV[\x82Q\x82Q\x93\x96P\x91\x94P\x92P\x90\x81\x15\x80b\0\x01zWPb\0\x01x\x85b\0\x02\xEEV[\x15[\x15b\0\x01\x99W`@Qc\x08Ii\x9D`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81\x81\x14b\0\x01\xBAW`@Qc\x10\x8C\xEF\x9D`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80[\x82\x81\x10\x15b\0\x02\x06W\x85\x81\x81Q\x81\x10b\0\x01\xDCWb\0\x01\xDCb\0\x05VV[` \x02` \x01\x01Q\x82b\0\x01\xF1\x91\x90b\0\x06\xF4V[\x91Pb\0\x01\xFE\x81b\0\x05\x82V[\x90Pb\0\x01\xBEV[P\x83\x15\x80b\0\x02\x14WP\x83\x81\x10[\x15b\0\x023W`@Qc\xAA\xBDZ\t`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x86Q` \x80\x89\x01\x91\x90\x91 `\0\x81\x81R`\x02\x90\x92R`@\x90\x91 T\x15b\0\x02mW`@Qc\xAD\xDAG\xF7`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80Tb\0\x02~\x90`\x01b\0\x06\xF4V[`\0\x81\x81U\x81\x81R`\x01` \x90\x81R`@\x80\x83 \x86\x90U\x85\x83R`\x02\x90\x91R\x90\x81\x90 \x82\x90UQ\x90\x91P\x7F\x05\xB53b\xD4\xAF\xEAu3\xE85\xBD\x99\xF6\xC0\xF2\xC2Q\xE2\xF0\x8B\\F\x174\x82\x95\x16Q\x9D\xD5\xAC\x90b\0\x02\xDB\x90\x8A\x90\x8A\x90\x8A\x90b\0\x07\x0FV[`@Q\x80\x91\x03\x90\xA1PPPPPPPPPV[\x80Q`\0\x90\x81\x83\x81\x83b\0\x03\x06Wb\0\x03\x06b\0\x05VV[` \x02` \x01\x01Q\x90P`\0`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x14\x15b\0\x036WP`\0\x93\x92PPPV[`\x01[\x82\x81\x10\x15b\0\x03\x9EW`\0\x85\x82\x81Q\x81\x10b\0\x03YWb\0\x03Yb\0\x05VV[` \x02` \x01\x01Q\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x10b\0\x03\x89WP`\0\x95\x94PPPPPV[\x91Pb\0\x03\x96\x81b\0\x05\x82V[\x90Pb\0\x039V[P`\x01\x94\x93PPPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15b\0\x03\xEAWb\0\x03\xEAb\0\x03\xA9V[`@R\x91\x90PV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15b\0\x04\x0EWb\0\x04\x0Eb\0\x03\xA9V[P`\x05\x1B` \x01\x90V[`\0` \x80\x83\x85\x03\x12\x15b\0\x04,W`\0\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x80\x82\x11\x15b\0\x04DW`\0\x80\xFD[\x81\x85\x01\x91P`\x1F\x86\x81\x84\x01\x12b\0\x04ZW`\0\x80\xFD[\x82Qb\0\x04qb\0\x04k\x82b\0\x03\xF2V[b\0\x03\xBFV[\x81\x81R`\x05\x91\x90\x91\x1B\x84\x01\x85\x01\x90\x85\x81\x01\x90\x89\x83\x11\x15b\0\x04\x91W`\0\x80\xFD[\x86\x86\x01[\x83\x81\x10\x15b\0\x05HW\x80Q\x86\x81\x11\x15b\0\x04\xAFW`\0\x80\x81\xFD[\x87\x01`?\x81\x01\x8C\x13b\0\x04\xC2W`\0\x80\x81\xFD[\x88\x81\x01Q\x87\x81\x11\x15b\0\x04\xD9Wb\0\x04\xD9b\0\x03\xA9V[b\0\x04\xEC\x81\x88\x01`\x1F\x19\x16\x8B\x01b\0\x03\xBFV[\x81\x81R`@\x8E\x81\x84\x86\x01\x01\x11\x15b\0\x05\x04W`\0\x80\x81\xFD[`\0[\x83\x81\x10\x15b\0\x05$W\x84\x81\x01\x82\x01Q\x83\x82\x01\x8E\x01R\x8C\x01b\0\x05\x07V[\x83\x81\x11\x15b\0\x056W`\0\x8D\x85\x85\x01\x01R[PP\x85RPP\x91\x87\x01\x91\x87\x01b\0\x04\x95V[P\x99\x98PPPPPPPPPV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\0\x19\x82\x14\x15b\0\x05\x99Wb\0\x05\x99b\0\x05lV[P`\x01\x01\x90V[`\0\x82`\x1F\x83\x01\x12b\0\x05\xB2W`\0\x80\xFD[\x81Q` b\0\x05\xC5b\0\x04k\x83b\0\x03\xF2V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15b\0\x05\xE5W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15b\0\x06\x02W\x80Q\x83R\x91\x83\x01\x91\x83\x01b\0\x05\xE9V[P\x96\x95PPPPPPV[`\0\x80`\0``\x84\x86\x03\x12\x15b\0\x06#W`\0\x80\xFD[\x83Q`\x01`\x01`@\x1B\x03\x80\x82\x11\x15b\0\x06;W`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12b\0\x06PW`\0\x80\xFD[\x81Q` b\0\x06cb\0\x04k\x83b\0\x03\xF2V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x8A\x84\x11\x15b\0\x06\x83W`\0\x80\xFD[\x94\x82\x01\x94[\x83\x86\x10\x15b\0\x06\xBAW\x85Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\x06\xAAW`\0\x80\x81\xFD[\x82R\x94\x82\x01\x94\x90\x82\x01\x90b\0\x06\x88V[\x91\x89\x01Q\x91\x97P\x90\x93PPP\x80\x82\x11\x15b\0\x06\xD4W`\0\x80\xFD[Pb\0\x06\xE3\x86\x82\x87\x01b\0\x05\xA0V[\x92PP`@\x84\x01Q\x90P\x92P\x92P\x92V[`\0\x82\x19\x82\x11\x15b\0\x07\nWb\0\x07\nb\0\x05lV[P\x01\x90V[``\x80\x82R\x84Q\x90\x82\x01\x81\x90R`\0\x90` \x90`\x80\x84\x01\x90\x82\x88\x01\x84[\x82\x81\x10\x15b\0\x07SW\x81Q`\x01`\x01`\xA0\x1B\x03\x16\x84R\x92\x84\x01\x92\x90\x84\x01\x90`\x01\x01b\0\x07,V[PPP\x83\x81\x03\x82\x85\x01R\x85Q\x80\x82R\x86\x83\x01\x91\x83\x01\x90`\0[\x81\x81\x10\x15b\0\x07\x8AW\x83Q\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01b\0\x07lV[PP\x80\x93PPPP\x82`@\x83\x01R\x94\x93PPPPV[a\x11!\x80b\0\x07\xB0`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xBEW`\x005`\xE0\x1C\x80c\xBAgB\xE5\x11a\0vW\x80c\xE3\x0C9x\x11a\0[W\x80c\xE3\x0C9x\x14a\x01\x8DW\x80c\xF1P\x1C\x89\x14a\x01\xB4W\x80c\xF2\xFD\xE3\x8B\x14a\x01\xD4W`\0\x80\xFD[\x80c\xBAgB\xE5\x14a\x01ZW\x80c\xD2\x89\xD1\xCB\x14a\x01zW`\0\x80\xFD[\x80cvg\x18\x08\x11a\0\xA7W\x80cvg\x18\x08\x14a\x01\0W\x80cy\xBAP\x97\x14a\x01\x17W\x80c\x8D\xA5\xCB[\x14a\x01\x1FW`\0\x80\xFD[\x80cq\x0B\xF3\"\x14a\0\xC3W\x80cs\xE3\xD6j\x14a\0\xD8W[`\0\x80\xFD[a\0\xD6a\0\xD16`\x04a\x0B\x01V[a\x01\xE7V[\0[a\0\xEBa\0\xE66`\x04a\x0BnV[a\x02\xB6V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\t`\0T\x81V[`@Q\x90\x81R` \x01a\0\xF7V[a\0\xD6a\x03bV[\x7F\x02\x01h6\xA5kq\xF0\xD0&\x89\xE6\x9E2oOL\x1B\x90W\x16N\xF5\x92g\x1C\xF0\xD3|\x80@\xC0T[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\xF7V[a\x01\ta\x01h6`\x04a\x0B\xBAV[`\x01` R`\0\x90\x81R`@\x90 T\x81V[a\0\xD6a\x01\x886`\x04a\x0B\xD3V[a\x03\xDCV[\x7F\x98U8A\"\xB5Y6\xFB\xFB\x8C\xA5\x12\x0Ec\xC6Sz\x1A\xC4\x0C\xAFj\xE35\x02\xB3\xC5\xDA\x8C\x87\xD1Ta\x01BV[a\x01\ta\x01\xC26`\x04a\x0B\xBAV[`\x02` R`\0\x90\x81R`@\x90 T\x81V[a\0\xD6a\x01\xE26`\x04a\x0B\x01V[a\x04oV[3a\x02\x10\x7F\x02\x01h6\xA5kq\xF0\xD0&\x89\xE6\x9E2oOL\x1B\x90W\x16N\xF5\x92g\x1C\xF0\xD3|\x80@\xC0T\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x027W`@Qc0\xCDtq`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x02^W`@Qc6I9}`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Q`\x01`\x01`\xA0\x1B\x03\x82\x16\x90\x7F\xD9\xBE\x0E\x8E\x07A~\0\xF2R\x1D\xB66\xCBS\xE3\x16\xFD(\x8FPQ\xF1m*\xA2\xBF\x0C98\xA8v\x90`\0\x90\xA2\x7F\x98U8A\"\xB5Y6\xFB\xFB\x8C\xA5\x12\x0Ec\xC6Sz\x1A\xC4\x0C\xAFj\xE35\x02\xB3\xC5\xDA\x8C\x87\xD1UV[`\0\x80\x80\x80\x80a\x02\xC8\x86\x88\x01\x88a\r\xCFV[\x93P\x93P\x93P\x93P`\0\x84\x84\x84`@Q` \x01a\x02\xE7\x93\x92\x91\x90a\x0E\xBFV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R\x81Q` \x92\x83\x01 `\0\x81\x81R`\x02\x90\x93R\x90\x82 T\x91T\x90\x92P\x81\x15\x80a\x03)WP`\x10a\x03&\x83\x83a\x0FbV[\x10\x15[\x15a\x03GW`@Qc\x08Ii\x9D`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x03T\x8B\x88\x88\x88\x88a\x04\xBFV[\x14\x99\x98PPPPPPPPPV[`\0a\x03\x8C\x7F\x98U8A\"\xB5Y6\xFB\xFB\x8C\xA5\x12\x0Ec\xC6Sz\x1A\xC4\x0C\xAFj\xE35\x02\xB3\xC5\xDA\x8C\x87\xD1T\x90V[\x90P`\x01`\x01`\xA0\x1B\x03\x81\x163\x14a\x03\xD0W`@Q\x7FI\xE2|\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x03\xD9\x81a\x06\x11V[PV[3a\x04\x05\x7F\x02\x01h6\xA5kq\xF0\xD0&\x89\xE6\x9E2oOL\x1B\x90W\x16N\xF5\x92g\x1C\xF0\xD3|\x80@\xC0T\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x04,W`@Qc0\xCDtq`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x04k\x82\x82\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\x06\xB4\x92PPPV[PPV[3a\x04\x98\x7F\x02\x01h6\xA5kq\xF0\xD0&\x89\xE6\x9E2oOL\x1B\x90W\x16N\xF5\x92g\x1C\xF0\xD3|\x80@\xC0T\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x03\xD0W`@Qc0\xCDtq`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x83Q\x81Q`\0\x80\x80[\x83\x81\x10\x15a\x05\xD7W`\0a\x04\xF5\x8B\x88\x84\x81Q\x81\x10a\x04\xE8Wa\x04\xE8a\x0FyV[` \x02` \x01\x01Qa\x08\x9AV[\x90P[\x85\x84\x10\x80\x15a\x052WP\x89\x84\x81Q\x81\x10a\x05\x14Wa\x05\x14a\x0FyV[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x14\x15[\x15a\x05GWa\x05@\x84a\x0F\x8FV[\x93Pa\x04\xF8V[\x85\x84\x14\x15a\x05\x81W`@Q\x7F\xC6\xFBS\x93\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x88\x84\x81Q\x81\x10a\x05\x93Wa\x05\x93a\x0FyV[` \x02` \x01\x01Q\x83a\x05\xA6\x91\x90a\x0F\xAAV[\x92P\x87\x83\x10a\x05\xBAWPPPPPPa\x06\nV[a\x05\xC3\x84a\x0F\x8FV[\x93PP\x80a\x05\xD0\x90a\x0F\x8FV[\x90Pa\x04\xC8V[P`@Q\x7F ;\"X\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x068W`@Qc6I9}`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Q`\x01`\x01`\xA0\x1B\x03\x82\x16\x90\x7F\x04\xDB\xA6\"\xD2\x84\xED\0\x14\xEEK\x9Ajh8k\xE1\xA4\xC0\x8AI\x13\xAE'-\xE8\x91\x99\xCChac\x90`\0\x90\xA2\x7F\x02\x01h6\xA5kq\xF0\xD0&\x89\xE6\x9E2oOL\x1B\x90W\x16N\xF5\x92g\x1C\xF0\xD3|\x80@\xC0U`\0\x7F\x98U8A\"\xB5Y6\xFB\xFB\x8C\xA5\x12\x0Ec\xC6Sz\x1A\xC4\x0C\xAFj\xE35\x02\xB3\xC5\xDA\x8C\x87\xD1UV[`\0\x80`\0\x83\x80` \x01\x90Q\x81\x01\x90a\x06\xCD\x91\x90a\x10\x1DV[\x82Q\x82Q\x93\x96P\x91\x94P\x92P\x90\x81\x15\x80a\x06\xEDWPa\x06\xEB\x85a\n=V[\x15[\x15a\x07\x0BW`@Qc\x08Ii\x9D`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81\x81\x14a\x07DW`@Q\x7F\x84g|\xE8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80[\x82\x81\x10\x15a\x07\x87W\x85\x81\x81Q\x81\x10a\x07bWa\x07ba\x0FyV[` \x02` \x01\x01Q\x82a\x07u\x91\x90a\x0F\xAAV[\x91Pa\x07\x80\x81a\x0F\x8FV[\x90Pa\x07HV[P\x83\x15\x80a\x07\x94WP\x83\x81\x10[\x15a\x07\xCBW`@Q\x7F\xAA\xBDZ\t\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x86Q` \x80\x89\x01\x91\x90\x91 `\0\x81\x81R`\x02\x90\x92R`@\x90\x91 T\x15a\x08\x1DW`@Q\x7F\xAD\xDAG\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80Ta\x08,\x90`\x01a\x0F\xAAV[`\0\x81\x81U\x81\x81R`\x01` \x90\x81R`@\x80\x83 \x86\x90U\x85\x83R`\x02\x90\x91R\x90\x81\x90 \x82\x90UQ\x90\x91P\x7F\x05\xB53b\xD4\xAF\xEAu3\xE85\xBD\x99\xF6\xC0\xF2\xC2Q\xE2\xF0\x8B\\F\x174\x82\x95\x16Q\x9D\xD5\xAC\x90a\x08\x87\x90\x8A\x90\x8A\x90\x8A\x90a\x0E\xBFV[`@Q\x80\x91\x03\x90\xA1PPPPPPPPPV[`\0\x81Q`A\x14a\x08\xD7W`@Q\x7FK\xE62\x1B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[` \x82\x01Q`@\x83\x01Q``\x84\x01Q`\0\x1A\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x82\x11\x15a\tCW`@Q\x7F@\xC1\xE7H\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80`\xFF\x16`\x1B\x14\x15\x80\x15a\t[WP\x80`\xFF\x16`\x1C\x14\x15[\x15a\t\x92W`@Q\x7F\x11\x9B\xCE9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q`\0\x80\x82R` \x82\x01\x80\x84R\x89\x90R`\xFF\x84\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x85\x90R`\x80\x81\x01\x84\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\t\xE6W=`\0\x80>=`\0\xFD[PPP` `@Q\x03Q\x94P\x84`\x01`\x01`\xA0\x1B\x03\x16\x14\x15a\n4W`@Q\x7F\x8B\xAAW\x9F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPP\x92\x91PPV[\x80Q`\0\x90\x81\x83\x81\x83a\nRWa\nRa\x0FyV[` \x02` \x01\x01Q\x90P`\0`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x14\x15a\n\x81WP`\0\x93\x92PPPV[`\x01[\x82\x81\x10\x15a\n\xE1W`\0\x85\x82\x81Q\x81\x10a\n\xA0Wa\n\xA0a\x0FyV[` \x02` \x01\x01Q\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x10a\n\xCFWP`\0\x95\x94PPPPPV[\x91Pa\n\xDA\x81a\x0F\x8FV[\x90Pa\n\x84V[P`\x01\x94\x93PPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x03\xD9W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x0B\x13W`\0\x80\xFD[\x815a\x0B\x1E\x81a\n\xECV[\x93\x92PPPV[`\0\x80\x83`\x1F\x84\x01\x12a\x0B7W`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0BOW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\x0BgW`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0`@\x84\x86\x03\x12\x15a\x0B\x83W`\0\x80\xFD[\x835\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0B\xA1W`\0\x80\xFD[a\x0B\xAD\x86\x82\x87\x01a\x0B%V[\x94\x97\x90\x96P\x93\x94PPPPV[`\0` \x82\x84\x03\x12\x15a\x0B\xCCW`\0\x80\xFD[P5\x91\x90PV[`\0\x80` \x83\x85\x03\x12\x15a\x0B\xE6W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0B\xFDW`\0\x80\xFD[a\x0C\t\x85\x82\x86\x01a\x0B%V[\x90\x96\x90\x95P\x93PPPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x0CTWa\x0CTa\x0C\x15V[`@R\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x0CvWa\x0Cva\x0C\x15V[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a\x0C\x91W`\0\x80\xFD[\x815` a\x0C\xA6a\x0C\xA1\x83a\x0C\\V[a\x0C+V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a\x0C\xC5W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a\x0C\xE0W\x805\x83R\x91\x83\x01\x91\x83\x01a\x0C\xC9V[P\x96\x95PPPPPPV[`\0`\x1F\x83\x81\x84\x01\x12a\x0C\xFDW`\0\x80\xFD[\x825` a\r\ra\x0C\xA1\x83a\x0C\\V[\x82\x81R`\x05\x92\x90\x92\x1B\x85\x01\x81\x01\x91\x81\x81\x01\x90\x87\x84\x11\x15a\r,W`\0\x80\xFD[\x82\x87\x01[\x84\x81\x10\x15a\r\xC3W\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\rQW`\0\x80\x81\xFD[\x81\x8A\x01\x91P\x8A`?\x83\x01\x12a\rfW`\0\x80\x81\xFD[\x85\x82\x015`@\x82\x82\x11\x15a\r|Wa\r|a\x0C\x15V[a\r\x8D\x82\x8B\x01`\x1F\x19\x16\x89\x01a\x0C+V[\x92P\x81\x83R\x8C\x81\x83\x86\x01\x01\x11\x15a\r\xA4W`\0\x80\x81\xFD[\x81\x81\x85\x01\x89\x85\x017P`\0\x90\x82\x01\x87\x01R\x84RP\x91\x83\x01\x91\x83\x01a\r0V[P\x97\x96PPPPPPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\r\xE5W`\0\x80\xFD[\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\r\xFDW`\0\x80\xFD[\x81\x87\x01\x91P\x87`\x1F\x83\x01\x12a\x0E\x11W`\0\x80\xFD[\x815` a\x0E!a\x0C\xA1\x83a\x0C\\V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x8B\x84\x11\x15a\x0E@W`\0\x80\xFD[\x94\x82\x01\x94[\x83\x86\x10\x15a\x0EgW\x855a\x0EX\x81a\n\xECV[\x82R\x94\x82\x01\x94\x90\x82\x01\x90a\x0EEV[\x98PP\x88\x015\x92PP\x80\x82\x11\x15a\x0E}W`\0\x80\xFD[a\x0E\x89\x88\x83\x89\x01a\x0C\x80V[\x94P`@\x87\x015\x93P``\x87\x015\x91P\x80\x82\x11\x15a\x0E\xA6W`\0\x80\xFD[Pa\x0E\xB3\x87\x82\x88\x01a\x0C\xEBV[\x91PP\x92\x95\x91\x94P\x92PV[``\x80\x82R\x84Q\x90\x82\x01\x81\x90R`\0\x90` \x90`\x80\x84\x01\x90\x82\x88\x01\x84[\x82\x81\x10\x15a\x0F\x01W\x81Q`\x01`\x01`\xA0\x1B\x03\x16\x84R\x92\x84\x01\x92\x90\x84\x01\x90`\x01\x01a\x0E\xDCV[PPP\x83\x81\x03\x82\x85\x01R\x85Q\x80\x82R\x86\x83\x01\x91\x83\x01\x90`\0[\x81\x81\x10\x15a\x0F6W\x83Q\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\x0F\x1AV[PP\x80\x93PPPP\x82`@\x83\x01R\x94\x93PPPPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0\x82\x82\x10\x15a\x0FtWa\x0Fta\x0FLV[P\x03\x90V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0`\0\x19\x82\x14\x15a\x0F\xA3Wa\x0F\xA3a\x0FLV[P`\x01\x01\x90V[`\0\x82\x19\x82\x11\x15a\x0F\xBDWa\x0F\xBDa\x0FLV[P\x01\x90V[`\0\x82`\x1F\x83\x01\x12a\x0F\xD3W`\0\x80\xFD[\x81Q` a\x0F\xE3a\x0C\xA1\x83a\x0C\\V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a\x10\x02W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a\x0C\xE0W\x80Q\x83R\x91\x83\x01\x91\x83\x01a\x10\x06V[`\0\x80`\0``\x84\x86\x03\x12\x15a\x102W`\0\x80\xFD[\x83Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x10JW`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a\x10^W`\0\x80\xFD[\x81Q` a\x10na\x0C\xA1\x83a\x0C\\V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x8A\x84\x11\x15a\x10\x8DW`\0\x80\xFD[\x94\x82\x01\x94[\x83\x86\x10\x15a\x10\xB4W\x85Qa\x10\xA5\x81a\n\xECV[\x82R\x94\x82\x01\x94\x90\x82\x01\x90a\x10\x92V[\x91\x89\x01Q\x91\x97P\x90\x93PPP\x80\x82\x11\x15a\x10\xCDW`\0\x80\xFD[Pa\x10\xDA\x86\x82\x87\x01a\x0F\xC2V[\x92PP`@\x84\x01Q\x90P\x92P\x92P\x92V\xFE\xA2dipfsX\"\x12 \0\xA6N\xA6\xE2\xEC\xC2\xFF&\xE8'\xBE\xBFC\xADz\xB6\xED\xEEC\"a=\xEAJ\xA8\xB6\xE3\x87\xC8Z\xB4dsolcC\0\x08\t\x003";
    /// The bytecode of the contract.
    pub static AXELARAUTHWEIGHTED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0\xBEW`\x005`\xE0\x1C\x80c\xBAgB\xE5\x11a\0vW\x80c\xE3\x0C9x\x11a\0[W\x80c\xE3\x0C9x\x14a\x01\x8DW\x80c\xF1P\x1C\x89\x14a\x01\xB4W\x80c\xF2\xFD\xE3\x8B\x14a\x01\xD4W`\0\x80\xFD[\x80c\xBAgB\xE5\x14a\x01ZW\x80c\xD2\x89\xD1\xCB\x14a\x01zW`\0\x80\xFD[\x80cvg\x18\x08\x11a\0\xA7W\x80cvg\x18\x08\x14a\x01\0W\x80cy\xBAP\x97\x14a\x01\x17W\x80c\x8D\xA5\xCB[\x14a\x01\x1FW`\0\x80\xFD[\x80cq\x0B\xF3\"\x14a\0\xC3W\x80cs\xE3\xD6j\x14a\0\xD8W[`\0\x80\xFD[a\0\xD6a\0\xD16`\x04a\x0B\x01V[a\x01\xE7V[\0[a\0\xEBa\0\xE66`\x04a\x0BnV[a\x02\xB6V[`@Q\x90\x15\x15\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x01\t`\0T\x81V[`@Q\x90\x81R` \x01a\0\xF7V[a\0\xD6a\x03bV[\x7F\x02\x01h6\xA5kq\xF0\xD0&\x89\xE6\x9E2oOL\x1B\x90W\x16N\xF5\x92g\x1C\xF0\xD3|\x80@\xC0T[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0\xF7V[a\x01\ta\x01h6`\x04a\x0B\xBAV[`\x01` R`\0\x90\x81R`@\x90 T\x81V[a\0\xD6a\x01\x886`\x04a\x0B\xD3V[a\x03\xDCV[\x7F\x98U8A\"\xB5Y6\xFB\xFB\x8C\xA5\x12\x0Ec\xC6Sz\x1A\xC4\x0C\xAFj\xE35\x02\xB3\xC5\xDA\x8C\x87\xD1Ta\x01BV[a\x01\ta\x01\xC26`\x04a\x0B\xBAV[`\x02` R`\0\x90\x81R`@\x90 T\x81V[a\0\xD6a\x01\xE26`\x04a\x0B\x01V[a\x04oV[3a\x02\x10\x7F\x02\x01h6\xA5kq\xF0\xD0&\x89\xE6\x9E2oOL\x1B\x90W\x16N\xF5\x92g\x1C\xF0\xD3|\x80@\xC0T\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x027W`@Qc0\xCDtq`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x02^W`@Qc6I9}`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Q`\x01`\x01`\xA0\x1B\x03\x82\x16\x90\x7F\xD9\xBE\x0E\x8E\x07A~\0\xF2R\x1D\xB66\xCBS\xE3\x16\xFD(\x8FPQ\xF1m*\xA2\xBF\x0C98\xA8v\x90`\0\x90\xA2\x7F\x98U8A\"\xB5Y6\xFB\xFB\x8C\xA5\x12\x0Ec\xC6Sz\x1A\xC4\x0C\xAFj\xE35\x02\xB3\xC5\xDA\x8C\x87\xD1UV[`\0\x80\x80\x80\x80a\x02\xC8\x86\x88\x01\x88a\r\xCFV[\x93P\x93P\x93P\x93P`\0\x84\x84\x84`@Q` \x01a\x02\xE7\x93\x92\x91\x90a\x0E\xBFV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R\x81Q` \x92\x83\x01 `\0\x81\x81R`\x02\x90\x93R\x90\x82 T\x91T\x90\x92P\x81\x15\x80a\x03)WP`\x10a\x03&\x83\x83a\x0FbV[\x10\x15[\x15a\x03GW`@Qc\x08Ii\x9D`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x03T\x8B\x88\x88\x88\x88a\x04\xBFV[\x14\x99\x98PPPPPPPPPV[`\0a\x03\x8C\x7F\x98U8A\"\xB5Y6\xFB\xFB\x8C\xA5\x12\x0Ec\xC6Sz\x1A\xC4\x0C\xAFj\xE35\x02\xB3\xC5\xDA\x8C\x87\xD1T\x90V[\x90P`\x01`\x01`\xA0\x1B\x03\x81\x163\x14a\x03\xD0W`@Q\x7FI\xE2|\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x03\xD9\x81a\x06\x11V[PV[3a\x04\x05\x7F\x02\x01h6\xA5kq\xF0\xD0&\x89\xE6\x9E2oOL\x1B\x90W\x16N\xF5\x92g\x1C\xF0\xD3|\x80@\xC0T\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x04,W`@Qc0\xCDtq`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x04k\x82\x82\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPa\x06\xB4\x92PPPV[PPV[3a\x04\x98\x7F\x02\x01h6\xA5kq\xF0\xD0&\x89\xE6\x9E2oOL\x1B\x90W\x16N\xF5\x92g\x1C\xF0\xD3|\x80@\xC0T\x90V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x03\xD0W`@Qc0\xCDtq`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x83Q\x81Q`\0\x80\x80[\x83\x81\x10\x15a\x05\xD7W`\0a\x04\xF5\x8B\x88\x84\x81Q\x81\x10a\x04\xE8Wa\x04\xE8a\x0FyV[` \x02` \x01\x01Qa\x08\x9AV[\x90P[\x85\x84\x10\x80\x15a\x052WP\x89\x84\x81Q\x81\x10a\x05\x14Wa\x05\x14a\x0FyV[` \x02` \x01\x01Q`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x14\x15[\x15a\x05GWa\x05@\x84a\x0F\x8FV[\x93Pa\x04\xF8V[\x85\x84\x14\x15a\x05\x81W`@Q\x7F\xC6\xFBS\x93\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x88\x84\x81Q\x81\x10a\x05\x93Wa\x05\x93a\x0FyV[` \x02` \x01\x01Q\x83a\x05\xA6\x91\x90a\x0F\xAAV[\x92P\x87\x83\x10a\x05\xBAWPPPPPPa\x06\nV[a\x05\xC3\x84a\x0F\x8FV[\x93PP\x80a\x05\xD0\x90a\x0F\x8FV[\x90Pa\x04\xC8V[P`@Q\x7F ;\"X\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x068W`@Qc6I9}`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Q`\x01`\x01`\xA0\x1B\x03\x82\x16\x90\x7F\x04\xDB\xA6\"\xD2\x84\xED\0\x14\xEEK\x9Ajh8k\xE1\xA4\xC0\x8AI\x13\xAE'-\xE8\x91\x99\xCChac\x90`\0\x90\xA2\x7F\x02\x01h6\xA5kq\xF0\xD0&\x89\xE6\x9E2oOL\x1B\x90W\x16N\xF5\x92g\x1C\xF0\xD3|\x80@\xC0U`\0\x7F\x98U8A\"\xB5Y6\xFB\xFB\x8C\xA5\x12\x0Ec\xC6Sz\x1A\xC4\x0C\xAFj\xE35\x02\xB3\xC5\xDA\x8C\x87\xD1UV[`\0\x80`\0\x83\x80` \x01\x90Q\x81\x01\x90a\x06\xCD\x91\x90a\x10\x1DV[\x82Q\x82Q\x93\x96P\x91\x94P\x92P\x90\x81\x15\x80a\x06\xEDWPa\x06\xEB\x85a\n=V[\x15[\x15a\x07\x0BW`@Qc\x08Ii\x9D`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81\x81\x14a\x07DW`@Q\x7F\x84g|\xE8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80[\x82\x81\x10\x15a\x07\x87W\x85\x81\x81Q\x81\x10a\x07bWa\x07ba\x0FyV[` \x02` \x01\x01Q\x82a\x07u\x91\x90a\x0F\xAAV[\x91Pa\x07\x80\x81a\x0F\x8FV[\x90Pa\x07HV[P\x83\x15\x80a\x07\x94WP\x83\x81\x10[\x15a\x07\xCBW`@Q\x7F\xAA\xBDZ\t\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x86Q` \x80\x89\x01\x91\x90\x91 `\0\x81\x81R`\x02\x90\x92R`@\x90\x91 T\x15a\x08\x1DW`@Q\x7F\xAD\xDAG\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80Ta\x08,\x90`\x01a\x0F\xAAV[`\0\x81\x81U\x81\x81R`\x01` \x90\x81R`@\x80\x83 \x86\x90U\x85\x83R`\x02\x90\x91R\x90\x81\x90 \x82\x90UQ\x90\x91P\x7F\x05\xB53b\xD4\xAF\xEAu3\xE85\xBD\x99\xF6\xC0\xF2\xC2Q\xE2\xF0\x8B\\F\x174\x82\x95\x16Q\x9D\xD5\xAC\x90a\x08\x87\x90\x8A\x90\x8A\x90\x8A\x90a\x0E\xBFV[`@Q\x80\x91\x03\x90\xA1PPPPPPPPPV[`\0\x81Q`A\x14a\x08\xD7W`@Q\x7FK\xE62\x1B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[` \x82\x01Q`@\x83\x01Q``\x84\x01Q`\0\x1A\x7F\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF]WnsW\xA4P\x1D\xDF\xE9/Fh\x1B \xA0\x82\x11\x15a\tCW`@Q\x7F@\xC1\xE7H\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80`\xFF\x16`\x1B\x14\x15\x80\x15a\t[WP\x80`\xFF\x16`\x1C\x14\x15[\x15a\t\x92W`@Q\x7F\x11\x9B\xCE9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@\x80Q`\0\x80\x82R` \x82\x01\x80\x84R\x89\x90R`\xFF\x84\x16\x92\x82\x01\x92\x90\x92R``\x81\x01\x85\x90R`\x80\x81\x01\x84\x90R`\x01\x90`\xA0\x01` `@Q` \x81\x03\x90\x80\x84\x03\x90\x85Z\xFA\x15\x80\x15a\t\xE6W=`\0\x80>=`\0\xFD[PPP` `@Q\x03Q\x94P\x84`\x01`\x01`\xA0\x1B\x03\x16\x14\x15a\n4W`@Q\x7F\x8B\xAAW\x9F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPP\x92\x91PPV[\x80Q`\0\x90\x81\x83\x81\x83a\nRWa\nRa\x0FyV[` \x02` \x01\x01Q\x90P`\0`\x01`\x01`\xA0\x1B\x03\x16\x81`\x01`\x01`\xA0\x1B\x03\x16\x14\x15a\n\x81WP`\0\x93\x92PPPV[`\x01[\x82\x81\x10\x15a\n\xE1W`\0\x85\x82\x81Q\x81\x10a\n\xA0Wa\n\xA0a\x0FyV[` \x02` \x01\x01Q\x90P\x80`\x01`\x01`\xA0\x1B\x03\x16\x83`\x01`\x01`\xA0\x1B\x03\x16\x10a\n\xCFWP`\0\x95\x94PPPPPV[\x91Pa\n\xDA\x81a\x0F\x8FV[\x90Pa\n\x84V[P`\x01\x94\x93PPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x03\xD9W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a\x0B\x13W`\0\x80\xFD[\x815a\x0B\x1E\x81a\n\xECV[\x93\x92PPPV[`\0\x80\x83`\x1F\x84\x01\x12a\x0B7W`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0BOW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\x0BgW`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0`@\x84\x86\x03\x12\x15a\x0B\x83W`\0\x80\xFD[\x835\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0B\xA1W`\0\x80\xFD[a\x0B\xAD\x86\x82\x87\x01a\x0B%V[\x94\x97\x90\x96P\x93\x94PPPPV[`\0` \x82\x84\x03\x12\x15a\x0B\xCCW`\0\x80\xFD[P5\x91\x90PV[`\0\x80` \x83\x85\x03\x12\x15a\x0B\xE6W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0B\xFDW`\0\x80\xFD[a\x0C\t\x85\x82\x86\x01a\x0B%V[\x90\x96\x90\x95P\x93PPPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\x0CTWa\x0CTa\x0C\x15V[`@R\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x0CvWa\x0Cva\x0C\x15V[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a\x0C\x91W`\0\x80\xFD[\x815` a\x0C\xA6a\x0C\xA1\x83a\x0C\\V[a\x0C+V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a\x0C\xC5W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a\x0C\xE0W\x805\x83R\x91\x83\x01\x91\x83\x01a\x0C\xC9V[P\x96\x95PPPPPPV[`\0`\x1F\x83\x81\x84\x01\x12a\x0C\xFDW`\0\x80\xFD[\x825` a\r\ra\x0C\xA1\x83a\x0C\\V[\x82\x81R`\x05\x92\x90\x92\x1B\x85\x01\x81\x01\x91\x81\x81\x01\x90\x87\x84\x11\x15a\r,W`\0\x80\xFD[\x82\x87\x01[\x84\x81\x10\x15a\r\xC3W\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\rQW`\0\x80\x81\xFD[\x81\x8A\x01\x91P\x8A`?\x83\x01\x12a\rfW`\0\x80\x81\xFD[\x85\x82\x015`@\x82\x82\x11\x15a\r|Wa\r|a\x0C\x15V[a\r\x8D\x82\x8B\x01`\x1F\x19\x16\x89\x01a\x0C+V[\x92P\x81\x83R\x8C\x81\x83\x86\x01\x01\x11\x15a\r\xA4W`\0\x80\x81\xFD[\x81\x81\x85\x01\x89\x85\x017P`\0\x90\x82\x01\x87\x01R\x84RP\x91\x83\x01\x91\x83\x01a\r0V[P\x97\x96PPPPPPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a\r\xE5W`\0\x80\xFD[\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\r\xFDW`\0\x80\xFD[\x81\x87\x01\x91P\x87`\x1F\x83\x01\x12a\x0E\x11W`\0\x80\xFD[\x815` a\x0E!a\x0C\xA1\x83a\x0C\\V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x8B\x84\x11\x15a\x0E@W`\0\x80\xFD[\x94\x82\x01\x94[\x83\x86\x10\x15a\x0EgW\x855a\x0EX\x81a\n\xECV[\x82R\x94\x82\x01\x94\x90\x82\x01\x90a\x0EEV[\x98PP\x88\x015\x92PP\x80\x82\x11\x15a\x0E}W`\0\x80\xFD[a\x0E\x89\x88\x83\x89\x01a\x0C\x80V[\x94P`@\x87\x015\x93P``\x87\x015\x91P\x80\x82\x11\x15a\x0E\xA6W`\0\x80\xFD[Pa\x0E\xB3\x87\x82\x88\x01a\x0C\xEBV[\x91PP\x92\x95\x91\x94P\x92PV[``\x80\x82R\x84Q\x90\x82\x01\x81\x90R`\0\x90` \x90`\x80\x84\x01\x90\x82\x88\x01\x84[\x82\x81\x10\x15a\x0F\x01W\x81Q`\x01`\x01`\xA0\x1B\x03\x16\x84R\x92\x84\x01\x92\x90\x84\x01\x90`\x01\x01a\x0E\xDCV[PPP\x83\x81\x03\x82\x85\x01R\x85Q\x80\x82R\x86\x83\x01\x91\x83\x01\x90`\0[\x81\x81\x10\x15a\x0F6W\x83Q\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a\x0F\x1AV[PP\x80\x93PPPP\x82`@\x83\x01R\x94\x93PPPPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0\x82\x82\x10\x15a\x0FtWa\x0Fta\x0FLV[P\x03\x90V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0`\0\x19\x82\x14\x15a\x0F\xA3Wa\x0F\xA3a\x0FLV[P`\x01\x01\x90V[`\0\x82\x19\x82\x11\x15a\x0F\xBDWa\x0F\xBDa\x0FLV[P\x01\x90V[`\0\x82`\x1F\x83\x01\x12a\x0F\xD3W`\0\x80\xFD[\x81Q` a\x0F\xE3a\x0C\xA1\x83a\x0C\\V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a\x10\x02W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a\x0C\xE0W\x80Q\x83R\x91\x83\x01\x91\x83\x01a\x10\x06V[`\0\x80`\0``\x84\x86\x03\x12\x15a\x102W`\0\x80\xFD[\x83Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x10JW`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a\x10^W`\0\x80\xFD[\x81Q` a\x10na\x0C\xA1\x83a\x0C\\V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x8A\x84\x11\x15a\x10\x8DW`\0\x80\xFD[\x94\x82\x01\x94[\x83\x86\x10\x15a\x10\xB4W\x85Qa\x10\xA5\x81a\n\xECV[\x82R\x94\x82\x01\x94\x90\x82\x01\x90a\x10\x92V[\x91\x89\x01Q\x91\x97P\x90\x93PPP\x80\x82\x11\x15a\x10\xCDW`\0\x80\xFD[Pa\x10\xDA\x86\x82\x87\x01a\x0F\xC2V[\x92PP`@\x84\x01Q\x90P\x92P\x92P\x92V\xFE\xA2dipfsX\"\x12 \0\xA6N\xA6\xE2\xEC\xC2\xFF&\xE8'\xBE\xBFC\xADz\xB6\xED\xEEC\"a=\xEAJ\xA8\xB6\xE3\x87\xC8Z\xB4dsolcC\0\x08\t\x003";
    /// The deployed bytecode of the contract.
    pub static AXELARAUTHWEIGHTED_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct AxelarAuthWeighted<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for AxelarAuthWeighted<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for AxelarAuthWeighted<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for AxelarAuthWeighted<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for AxelarAuthWeighted<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(AxelarAuthWeighted))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> AxelarAuthWeighted<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    AXELARAUTHWEIGHTED_ABI.clone(),
                    client,
                ),
            )
        }
        /// Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it.
        /// Returns a new instance of a deployer that returns an instance of this contract after sending the transaction
        ///
        /// Notes:
        /// - If there are no constructor arguments, you should pass `()` as the argument.
        /// - The default poll duration is 7 seconds.
        /// - The default number of confirmations is 1 block.
        ///
        ///
        /// # Example
        ///
        /// Generate contract bindings with `abigen!` and deploy a new contract instance.
        ///
        /// *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact.
        ///
        /// ```ignore
        /// # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {
        ///     abigen!(Greeter, "../greeter.json");
        ///
        ///    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();
        ///    let msg = greeter_contract.greet().call().await.unwrap();
        /// # }
        /// ```
        pub fn deploy<T: ::ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::core::result::Result<
            ::ethers::contract::builders::ContractDeployer<M, Self>,
            ::ethers::contract::ContractError<M>,
        > {
            let factory = ::ethers::contract::ContractFactory::new(
                AXELARAUTHWEIGHTED_ABI.clone(),
                AXELARAUTHWEIGHTED_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `acceptOwnership` (0x79ba5097) function
        pub fn accept_ownership(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([121, 186, 80, 151], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `currentEpoch` (0x76671808) function
        pub fn current_epoch(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([118, 103, 24, 8], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `epochForHash` (0xf1501c89) function
        pub fn epoch_for_hash(
            &self,
            p0: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([241, 80, 28, 137], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `hashForEpoch` (0xba6742e5) function
        pub fn hash_for_epoch(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([186, 103, 66, 229], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `owner` (0x8da5cb5b) function
        pub fn owner(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([141, 165, 203, 91], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pendingOwner` (0xe30c3978) function
        pub fn pending_owner(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([227, 12, 57, 120], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `proposeOwnership` (0x710bf322) function
        pub fn propose_ownership(
            &self,
            new_owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 11, 243, 34], new_owner)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferOperatorship` (0xd289d1cb) function
        pub fn transfer_operatorship(
            &self,
            params: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([210, 137, 209, 203], params)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferOwnership` (0xf2fde38b) function
        pub fn transfer_ownership(
            &self,
            new_owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 253, 227, 139], new_owner)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `validateProof` (0x73e3d66a) function
        pub fn validate_proof(
            &self,
            message_hash: [u8; 32],
            proof: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([115, 227, 214, 106], (message_hash, proof))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `OperatorshipTransferred` event
        pub fn operatorship_transferred_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OperatorshipTransferredFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `OwnershipTransferStarted` event
        pub fn ownership_transfer_started_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OwnershipTransferStartedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `OwnershipTransferred` event
        pub fn ownership_transferred_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OwnershipTransferredFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AxelarAuthWeightedEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for AxelarAuthWeighted<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `DuplicateOperators` with signature `DuplicateOperators()` and selector `0xadda47f7`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "DuplicateOperators", abi = "DuplicateOperators()")]
    pub struct DuplicateOperators;
    ///Custom Error type `InvalidOperators` with signature `InvalidOperators()` and selector `0x1092d33a`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "InvalidOperators", abi = "InvalidOperators()")]
    pub struct InvalidOperators;
    ///Custom Error type `InvalidOwner` with signature `InvalidOwner()` and selector `0x49e27cff`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "InvalidOwner", abi = "InvalidOwner()")]
    pub struct InvalidOwner;
    ///Custom Error type `InvalidOwnerAddress` with signature `InvalidOwnerAddress()` and selector `0xd924e5f4`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "InvalidOwnerAddress", abi = "InvalidOwnerAddress()")]
    pub struct InvalidOwnerAddress;
    ///Custom Error type `InvalidS` with signature `InvalidS()` and selector `0x40c1e748`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "InvalidS", abi = "InvalidS()")]
    pub struct InvalidS;
    ///Custom Error type `InvalidSignature` with signature `InvalidSignature()` and selector `0x8baa579f`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "InvalidSignature", abi = "InvalidSignature()")]
    pub struct InvalidSignature;
    ///Custom Error type `InvalidSignatureLength` with signature `InvalidSignatureLength()` and selector `0x4be6321b`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "InvalidSignatureLength", abi = "InvalidSignatureLength()")]
    pub struct InvalidSignatureLength;
    ///Custom Error type `InvalidThreshold` with signature `InvalidThreshold()` and selector `0xaabd5a09`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "InvalidThreshold", abi = "InvalidThreshold()")]
    pub struct InvalidThreshold;
    ///Custom Error type `InvalidV` with signature `InvalidV()` and selector `0x119bce39`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "InvalidV", abi = "InvalidV()")]
    pub struct InvalidV;
    ///Custom Error type `InvalidWeights` with signature `InvalidWeights()` and selector `0x84677ce8`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "InvalidWeights", abi = "InvalidWeights()")]
    pub struct InvalidWeights;
    ///Custom Error type `LowSignaturesWeight` with signature `LowSignaturesWeight()` and selector `0x203b2258`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "LowSignaturesWeight", abi = "LowSignaturesWeight()")]
    pub struct LowSignaturesWeight;
    ///Custom Error type `MalformedSigners` with signature `MalformedSigners()` and selector `0xc6fb5393`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "MalformedSigners", abi = "MalformedSigners()")]
    pub struct MalformedSigners;
    ///Custom Error type `NotOwner` with signature `NotOwner()` and selector `0x30cd7471`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "NotOwner", abi = "NotOwner()")]
    pub struct NotOwner;
    ///Container type for all of the contract's custom errors
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum AxelarAuthWeightedErrors {
        DuplicateOperators(DuplicateOperators),
        InvalidOperators(InvalidOperators),
        InvalidOwner(InvalidOwner),
        InvalidOwnerAddress(InvalidOwnerAddress),
        InvalidS(InvalidS),
        InvalidSignature(InvalidSignature),
        InvalidSignatureLength(InvalidSignatureLength),
        InvalidThreshold(InvalidThreshold),
        InvalidV(InvalidV),
        InvalidWeights(InvalidWeights),
        LowSignaturesWeight(LowSignaturesWeight),
        MalformedSigners(MalformedSigners),
        NotOwner(NotOwner),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for AxelarAuthWeightedErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <DuplicateOperators as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DuplicateOperators(decoded));
            }
            if let Ok(decoded) = <InvalidOperators as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidOperators(decoded));
            }
            if let Ok(decoded) = <InvalidOwner as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidOwner(decoded));
            }
            if let Ok(decoded) = <InvalidOwnerAddress as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidOwnerAddress(decoded));
            }
            if let Ok(decoded) = <InvalidS as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidS(decoded));
            }
            if let Ok(decoded) = <InvalidSignature as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidSignature(decoded));
            }
            if let Ok(decoded) = <InvalidSignatureLength as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidSignatureLength(decoded));
            }
            if let Ok(decoded) = <InvalidThreshold as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidThreshold(decoded));
            }
            if let Ok(decoded) = <InvalidV as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidV(decoded));
            }
            if let Ok(decoded) = <InvalidWeights as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidWeights(decoded));
            }
            if let Ok(decoded) = <LowSignaturesWeight as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LowSignaturesWeight(decoded));
            }
            if let Ok(decoded) = <MalformedSigners as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MalformedSigners(decoded));
            }
            if let Ok(decoded) = <NotOwner as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::NotOwner(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for AxelarAuthWeightedErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::DuplicateOperators(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidOperators(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidOwner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidOwnerAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidS(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidSignature(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidSignatureLength(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidThreshold(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidV(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidWeights(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LowSignaturesWeight(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MalformedSigners(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NotOwner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for AxelarAuthWeightedErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <DuplicateOperators as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidOperators as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidOwner as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <InvalidOwnerAddress as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidS as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <InvalidSignature as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidSignatureLength as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidThreshold as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidV as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <InvalidWeights as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <LowSignaturesWeight as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <MalformedSigners as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NotOwner as ::ethers::contract::EthError>::selector() => true,
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for AxelarAuthWeightedErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::DuplicateOperators(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidOperators(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidOwner(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidOwnerAddress(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidS(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidSignature(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidSignatureLength(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidThreshold(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidV(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidWeights(element) => ::core::fmt::Display::fmt(element, f),
                Self::LowSignaturesWeight(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MalformedSigners(element) => ::core::fmt::Display::fmt(element, f),
                Self::NotOwner(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for AxelarAuthWeightedErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<DuplicateOperators> for AxelarAuthWeightedErrors {
        fn from(value: DuplicateOperators) -> Self {
            Self::DuplicateOperators(value)
        }
    }
    impl ::core::convert::From<InvalidOperators> for AxelarAuthWeightedErrors {
        fn from(value: InvalidOperators) -> Self {
            Self::InvalidOperators(value)
        }
    }
    impl ::core::convert::From<InvalidOwner> for AxelarAuthWeightedErrors {
        fn from(value: InvalidOwner) -> Self {
            Self::InvalidOwner(value)
        }
    }
    impl ::core::convert::From<InvalidOwnerAddress> for AxelarAuthWeightedErrors {
        fn from(value: InvalidOwnerAddress) -> Self {
            Self::InvalidOwnerAddress(value)
        }
    }
    impl ::core::convert::From<InvalidS> for AxelarAuthWeightedErrors {
        fn from(value: InvalidS) -> Self {
            Self::InvalidS(value)
        }
    }
    impl ::core::convert::From<InvalidSignature> for AxelarAuthWeightedErrors {
        fn from(value: InvalidSignature) -> Self {
            Self::InvalidSignature(value)
        }
    }
    impl ::core::convert::From<InvalidSignatureLength> for AxelarAuthWeightedErrors {
        fn from(value: InvalidSignatureLength) -> Self {
            Self::InvalidSignatureLength(value)
        }
    }
    impl ::core::convert::From<InvalidThreshold> for AxelarAuthWeightedErrors {
        fn from(value: InvalidThreshold) -> Self {
            Self::InvalidThreshold(value)
        }
    }
    impl ::core::convert::From<InvalidV> for AxelarAuthWeightedErrors {
        fn from(value: InvalidV) -> Self {
            Self::InvalidV(value)
        }
    }
    impl ::core::convert::From<InvalidWeights> for AxelarAuthWeightedErrors {
        fn from(value: InvalidWeights) -> Self {
            Self::InvalidWeights(value)
        }
    }
    impl ::core::convert::From<LowSignaturesWeight> for AxelarAuthWeightedErrors {
        fn from(value: LowSignaturesWeight) -> Self {
            Self::LowSignaturesWeight(value)
        }
    }
    impl ::core::convert::From<MalformedSigners> for AxelarAuthWeightedErrors {
        fn from(value: MalformedSigners) -> Self {
            Self::MalformedSigners(value)
        }
    }
    impl ::core::convert::From<NotOwner> for AxelarAuthWeightedErrors {
        fn from(value: NotOwner) -> Self {
            Self::NotOwner(value)
        }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "OperatorshipTransferred",
        abi = "OperatorshipTransferred(address[],uint256[],uint256)"
    )]
    pub struct OperatorshipTransferredFilter {
        pub new_operators: ::std::vec::Vec<::ethers::core::types::Address>,
        pub new_weights: ::std::vec::Vec<::ethers::core::types::U256>,
        pub new_threshold: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "OwnershipTransferStarted",
        abi = "OwnershipTransferStarted(address)"
    )]
    pub struct OwnershipTransferStartedFilter {
        #[ethevent(indexed)]
        pub new_owner: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "OwnershipTransferred", abi = "OwnershipTransferred(address)")]
    pub struct OwnershipTransferredFilter {
        #[ethevent(indexed)]
        pub new_owner: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum AxelarAuthWeightedEvents {
        OperatorshipTransferredFilter(OperatorshipTransferredFilter),
        OwnershipTransferStartedFilter(OwnershipTransferStartedFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
    }
    impl ::ethers::contract::EthLogDecode for AxelarAuthWeightedEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = OperatorshipTransferredFilter::decode_log(log) {
                return Ok(
                    AxelarAuthWeightedEvents::OperatorshipTransferredFilter(decoded),
                );
            }
            if let Ok(decoded) = OwnershipTransferStartedFilter::decode_log(log) {
                return Ok(
                    AxelarAuthWeightedEvents::OwnershipTransferStartedFilter(decoded),
                );
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(AxelarAuthWeightedEvents::OwnershipTransferredFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for AxelarAuthWeightedEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::OperatorshipTransferredFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OwnershipTransferStartedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OwnershipTransferredFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<OperatorshipTransferredFilter>
    for AxelarAuthWeightedEvents {
        fn from(value: OperatorshipTransferredFilter) -> Self {
            Self::OperatorshipTransferredFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferStartedFilter>
    for AxelarAuthWeightedEvents {
        fn from(value: OwnershipTransferStartedFilter) -> Self {
            Self::OwnershipTransferStartedFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter> for AxelarAuthWeightedEvents {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
        }
    }
    ///Container type for all input parameters for the `acceptOwnership` function with signature `acceptOwnership()` and selector `0x79ba5097`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "acceptOwnership", abi = "acceptOwnership()")]
    pub struct AcceptOwnershipCall;
    ///Container type for all input parameters for the `currentEpoch` function with signature `currentEpoch()` and selector `0x76671808`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "currentEpoch", abi = "currentEpoch()")]
    pub struct CurrentEpochCall;
    ///Container type for all input parameters for the `epochForHash` function with signature `epochForHash(bytes32)` and selector `0xf1501c89`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "epochForHash", abi = "epochForHash(bytes32)")]
    pub struct EpochForHashCall(pub [u8; 32]);
    ///Container type for all input parameters for the `hashForEpoch` function with signature `hashForEpoch(uint256)` and selector `0xba6742e5`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "hashForEpoch", abi = "hashForEpoch(uint256)")]
    pub struct HashForEpochCall(pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `owner` function with signature `owner()` and selector `0x8da5cb5b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
    ///Container type for all input parameters for the `pendingOwner` function with signature `pendingOwner()` and selector `0xe30c3978`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "pendingOwner", abi = "pendingOwner()")]
    pub struct PendingOwnerCall;
    ///Container type for all input parameters for the `proposeOwnership` function with signature `proposeOwnership(address)` and selector `0x710bf322`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "proposeOwnership", abi = "proposeOwnership(address)")]
    pub struct ProposeOwnershipCall {
        pub new_owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `transferOperatorship` function with signature `transferOperatorship(bytes)` and selector `0xd289d1cb`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "transferOperatorship", abi = "transferOperatorship(bytes)")]
    pub struct TransferOperatorshipCall {
        pub params: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `transferOwnership` function with signature `transferOwnership(address)` and selector `0xf2fde38b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "transferOwnership", abi = "transferOwnership(address)")]
    pub struct TransferOwnershipCall {
        pub new_owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `validateProof` function with signature `validateProof(bytes32,bytes)` and selector `0x73e3d66a`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "validateProof", abi = "validateProof(bytes32,bytes)")]
    pub struct ValidateProofCall {
        pub message_hash: [u8; 32],
        pub proof: ::ethers::core::types::Bytes,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum AxelarAuthWeightedCalls {
        AcceptOwnership(AcceptOwnershipCall),
        CurrentEpoch(CurrentEpochCall),
        EpochForHash(EpochForHashCall),
        HashForEpoch(HashForEpochCall),
        Owner(OwnerCall),
        PendingOwner(PendingOwnerCall),
        ProposeOwnership(ProposeOwnershipCall),
        TransferOperatorship(TransferOperatorshipCall),
        TransferOwnership(TransferOwnershipCall),
        ValidateProof(ValidateProofCall),
    }
    impl ::ethers::core::abi::AbiDecode for AxelarAuthWeightedCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <AcceptOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AcceptOwnership(decoded));
            }
            if let Ok(decoded) = <CurrentEpochCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CurrentEpoch(decoded));
            }
            if let Ok(decoded) = <EpochForHashCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EpochForHash(decoded));
            }
            if let Ok(decoded) = <HashForEpochCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::HashForEpoch(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded) = <PendingOwnerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PendingOwner(decoded));
            }
            if let Ok(decoded) = <ProposeOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ProposeOwnership(decoded));
            }
            if let Ok(decoded) = <TransferOperatorshipCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TransferOperatorship(decoded));
            }
            if let Ok(decoded) = <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TransferOwnership(decoded));
            }
            if let Ok(decoded) = <ValidateProofCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ValidateProof(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for AxelarAuthWeightedCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AcceptOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CurrentEpoch(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EpochForHash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::HashForEpoch(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PendingOwner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ProposeOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferOperatorship(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ValidateProof(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for AxelarAuthWeightedCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AcceptOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::CurrentEpoch(element) => ::core::fmt::Display::fmt(element, f),
                Self::EpochForHash(element) => ::core::fmt::Display::fmt(element, f),
                Self::HashForEpoch(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::PendingOwner(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProposeOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOperatorship(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::ValidateProof(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AcceptOwnershipCall> for AxelarAuthWeightedCalls {
        fn from(value: AcceptOwnershipCall) -> Self {
            Self::AcceptOwnership(value)
        }
    }
    impl ::core::convert::From<CurrentEpochCall> for AxelarAuthWeightedCalls {
        fn from(value: CurrentEpochCall) -> Self {
            Self::CurrentEpoch(value)
        }
    }
    impl ::core::convert::From<EpochForHashCall> for AxelarAuthWeightedCalls {
        fn from(value: EpochForHashCall) -> Self {
            Self::EpochForHash(value)
        }
    }
    impl ::core::convert::From<HashForEpochCall> for AxelarAuthWeightedCalls {
        fn from(value: HashForEpochCall) -> Self {
            Self::HashForEpoch(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for AxelarAuthWeightedCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<PendingOwnerCall> for AxelarAuthWeightedCalls {
        fn from(value: PendingOwnerCall) -> Self {
            Self::PendingOwner(value)
        }
    }
    impl ::core::convert::From<ProposeOwnershipCall> for AxelarAuthWeightedCalls {
        fn from(value: ProposeOwnershipCall) -> Self {
            Self::ProposeOwnership(value)
        }
    }
    impl ::core::convert::From<TransferOperatorshipCall> for AxelarAuthWeightedCalls {
        fn from(value: TransferOperatorshipCall) -> Self {
            Self::TransferOperatorship(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for AxelarAuthWeightedCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    impl ::core::convert::From<ValidateProofCall> for AxelarAuthWeightedCalls {
        fn from(value: ValidateProofCall) -> Self {
            Self::ValidateProof(value)
        }
    }
    ///Container type for all return fields from the `currentEpoch` function with signature `currentEpoch()` and selector `0x76671808`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct CurrentEpochReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `epochForHash` function with signature `epochForHash(bytes32)` and selector `0xf1501c89`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct EpochForHashReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `hashForEpoch` function with signature `hashForEpoch(uint256)` and selector `0xba6742e5`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct HashForEpochReturn(pub [u8; 32]);
    ///Container type for all return fields from the `owner` function with signature `owner()` and selector `0x8da5cb5b`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct OwnerReturn {
        pub owner: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `pendingOwner` function with signature `pendingOwner()` and selector `0xe30c3978`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct PendingOwnerReturn {
        pub owner: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `validateProof` function with signature `validateProof(bytes32,bytes)` and selector `0x73e3d66a`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct ValidateProofReturn(pub bool);
}

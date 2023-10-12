pub use axelar_executable::*;
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
pub mod axelar_executable {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("gateway_"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("execute"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("execute"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("commandId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("sourceChain"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("sourceAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("payload"),
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
                    ::std::borrow::ToOwned::to_owned("executeWithToken"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("executeWithToken"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("commandId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("sourceChain"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("sourceAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("payload"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("tokenSymbol"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
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
                    ::std::borrow::ToOwned::to_owned("gateway"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("gateway"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IAxelarGateway"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("InvalidAddress"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidAddress"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NotApprovedByGateway"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "NotApprovedByGateway",
                            ),
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
    pub static AXELAREXECUTABLE_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xA0`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa\x06\xA38\x03\x80a\x06\xA3\x839\x81\x01`@\x81\x90Ra\0/\x91a\0gV[`\x01`\x01`\xA0\x1B\x03\x81\x16a\0VW`@Qc\xE6\xC4${`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x16`\x80Ra\0\x97V[`\0` \x82\x84\x03\x12\x15a\0yW`\0\x80\xFD[\x81Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0\x90W`\0\x80\xFD[\x93\x92PPPV[`\x80Qa\x05\xE4a\0\xBF`\09`\0\x81\x81`K\x01R\x81\x81a\x01\x15\x01Ra\x02-\x01Ra\x05\xE4`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0AW`\x005`\xE0\x1C\x80c\x11a\x91\xB6\x14a\0FW\x80c\x1A\x98\xB2\xE0\x14a\0\x96W\x80cI\x16\x06X\x14a\0\xABW[`\0\x80\xFD[a\0m\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90\xF3[a\0\xA9a\0\xA46`\x04a\x03.V[a\0\xBEV[\0[a\0\xA9a\0\xB96`\x04a\x04\x08V[a\x01\xD6V[`\0\x85\x85`@Qa\0\xD0\x92\x91\x90a\x04\xACV[`@Q\x90\x81\x90\x03\x81 \x7F\x18v\xEE\xD9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R\x91Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x18v\xEE\xD9\x90a\x01Z\x90\x8E\x90\x8E\x90\x8E\x90\x8E\x90\x8E\x90\x89\x90\x8D\x90\x8D\x90\x8D\x90`\x04\x01a\x04\xE5V[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x01tW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x01\x88W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\xAC\x91\x90a\x05DV[a\x01\xC9W`@Qc\x14\x03\x11-`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPPPPPPPPV[`\0\x82\x82`@Qa\x01\xE8\x92\x91\x90a\x04\xACV[`@Q\x90\x81\x90\x03\x81 \x7F_ip\xC3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R\x91Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c_ip\xC3\x90a\x02l\x90\x8B\x90\x8B\x90\x8B\x90\x8B\x90\x8B\x90\x89\x90`\x04\x01a\x05mV[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x02\x86W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x02\x9AW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\xBE\x91\x90a\x05DV[a\x02\xDBW`@Qc\x14\x03\x11-`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPPPPPV[`\0\x80\x83`\x1F\x84\x01\x12a\x02\xF7W`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x03\x0FW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\x03'W`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`\0\x80`\0\x80`\0\x80`\xC0\x8B\x8D\x03\x12\x15a\x03MW`\0\x80\xFD[\x8A5\x99P` \x8B\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x03lW`\0\x80\xFD[a\x03x\x8E\x83\x8F\x01a\x02\xE5V[\x90\x9BP\x99P`@\x8D\x015\x91P\x80\x82\x11\x15a\x03\x91W`\0\x80\xFD[a\x03\x9D\x8E\x83\x8F\x01a\x02\xE5V[\x90\x99P\x97P``\x8D\x015\x91P\x80\x82\x11\x15a\x03\xB6W`\0\x80\xFD[a\x03\xC2\x8E\x83\x8F\x01a\x02\xE5V[\x90\x97P\x95P`\x80\x8D\x015\x91P\x80\x82\x11\x15a\x03\xDBW`\0\x80\xFD[Pa\x03\xE8\x8D\x82\x8E\x01a\x02\xE5V[\x91P\x80\x94PP\x80\x92PP`\xA0\x8B\x015\x90P\x92\x95\x98\x9B\x91\x94\x97\x9AP\x92\x95\x98PV[`\0\x80`\0\x80`\0\x80`\0`\x80\x88\x8A\x03\x12\x15a\x04#W`\0\x80\xFD[\x875\x96P` \x88\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x04BW`\0\x80\xFD[a\x04N\x8B\x83\x8C\x01a\x02\xE5V[\x90\x98P\x96P`@\x8A\x015\x91P\x80\x82\x11\x15a\x04gW`\0\x80\xFD[a\x04s\x8B\x83\x8C\x01a\x02\xE5V[\x90\x96P\x94P``\x8A\x015\x91P\x80\x82\x11\x15a\x04\x8CW`\0\x80\xFD[Pa\x04\x99\x8A\x82\x8B\x01a\x02\xE5V[\x98\x9B\x97\x9AP\x95\x98P\x93\x96\x92\x95\x92\x93PPPV[\x81\x83\x827`\0\x91\x01\x90\x81R\x91\x90PV[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[\x89\x81R`\xC0` \x82\x01R`\0a\x04\xFF`\xC0\x83\x01\x8A\x8Ca\x04\xBCV[\x82\x81\x03`@\x84\x01Ra\x05\x12\x81\x89\x8Ba\x04\xBCV[\x90P\x86``\x84\x01R\x82\x81\x03`\x80\x84\x01Ra\x05-\x81\x86\x88a\x04\xBCV[\x91PP\x82`\xA0\x83\x01R\x9A\x99PPPPPPPPPPV[`\0` \x82\x84\x03\x12\x15a\x05VW`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x05fW`\0\x80\xFD[\x93\x92PPPV[\x86\x81R`\x80` \x82\x01R`\0a\x05\x87`\x80\x83\x01\x87\x89a\x04\xBCV[\x82\x81\x03`@\x84\x01Ra\x05\x9A\x81\x86\x88a\x04\xBCV[\x91PP\x82``\x83\x01R\x97\x96PPPPPPPV\xFE\xA2dipfsX\"\x12 $\x91<\xCBT\xB4A\xCAVzJ\x85\xD8pH\xF1\xED\xDD\xEE\xB1?H\xBC\xAA\x80\x80\xF7\x07O\x03cfdsolcC\0\x08\t\x003";
    /// The bytecode of the contract.
    pub static AXELAREXECUTABLE_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0AW`\x005`\xE0\x1C\x80c\x11a\x91\xB6\x14a\0FW\x80c\x1A\x98\xB2\xE0\x14a\0\x96W\x80cI\x16\x06X\x14a\0\xABW[`\0\x80\xFD[a\0m\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01`@Q\x80\x91\x03\x90\xF3[a\0\xA9a\0\xA46`\x04a\x03.V[a\0\xBEV[\0[a\0\xA9a\0\xB96`\x04a\x04\x08V[a\x01\xD6V[`\0\x85\x85`@Qa\0\xD0\x92\x91\x90a\x04\xACV[`@Q\x90\x81\x90\x03\x81 \x7F\x18v\xEE\xD9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R\x91Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\x18v\xEE\xD9\x90a\x01Z\x90\x8E\x90\x8E\x90\x8E\x90\x8E\x90\x8E\x90\x89\x90\x8D\x90\x8D\x90\x8D\x90`\x04\x01a\x04\xE5V[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x01tW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x01\x88W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\xAC\x91\x90a\x05DV[a\x01\xC9W`@Qc\x14\x03\x11-`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPPPPPPPPV[`\0\x82\x82`@Qa\x01\xE8\x92\x91\x90a\x04\xACV[`@Q\x90\x81\x90\x03\x81 \x7F_ip\xC3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82R\x91Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c_ip\xC3\x90a\x02l\x90\x8B\x90\x8B\x90\x8B\x90\x8B\x90\x8B\x90\x89\x90`\x04\x01a\x05mV[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x02\x86W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x02\x9AW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x02\xBE\x91\x90a\x05DV[a\x02\xDBW`@Qc\x14\x03\x11-`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPPPPPV[`\0\x80\x83`\x1F\x84\x01\x12a\x02\xF7W`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x03\x0FW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\x03'W`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80`\0\x80`\0\x80`\0\x80`\0\x80`\xC0\x8B\x8D\x03\x12\x15a\x03MW`\0\x80\xFD[\x8A5\x99P` \x8B\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x03lW`\0\x80\xFD[a\x03x\x8E\x83\x8F\x01a\x02\xE5V[\x90\x9BP\x99P`@\x8D\x015\x91P\x80\x82\x11\x15a\x03\x91W`\0\x80\xFD[a\x03\x9D\x8E\x83\x8F\x01a\x02\xE5V[\x90\x99P\x97P``\x8D\x015\x91P\x80\x82\x11\x15a\x03\xB6W`\0\x80\xFD[a\x03\xC2\x8E\x83\x8F\x01a\x02\xE5V[\x90\x97P\x95P`\x80\x8D\x015\x91P\x80\x82\x11\x15a\x03\xDBW`\0\x80\xFD[Pa\x03\xE8\x8D\x82\x8E\x01a\x02\xE5V[\x91P\x80\x94PP\x80\x92PP`\xA0\x8B\x015\x90P\x92\x95\x98\x9B\x91\x94\x97\x9AP\x92\x95\x98PV[`\0\x80`\0\x80`\0\x80`\0`\x80\x88\x8A\x03\x12\x15a\x04#W`\0\x80\xFD[\x875\x96P` \x88\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x04BW`\0\x80\xFD[a\x04N\x8B\x83\x8C\x01a\x02\xE5V[\x90\x98P\x96P`@\x8A\x015\x91P\x80\x82\x11\x15a\x04gW`\0\x80\xFD[a\x04s\x8B\x83\x8C\x01a\x02\xE5V[\x90\x96P\x94P``\x8A\x015\x91P\x80\x82\x11\x15a\x04\x8CW`\0\x80\xFD[Pa\x04\x99\x8A\x82\x8B\x01a\x02\xE5V[\x98\x9B\x97\x9AP\x95\x98P\x93\x96\x92\x95\x92\x93PPPV[\x81\x83\x827`\0\x91\x01\x90\x81R\x91\x90PV[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[\x89\x81R`\xC0` \x82\x01R`\0a\x04\xFF`\xC0\x83\x01\x8A\x8Ca\x04\xBCV[\x82\x81\x03`@\x84\x01Ra\x05\x12\x81\x89\x8Ba\x04\xBCV[\x90P\x86``\x84\x01R\x82\x81\x03`\x80\x84\x01Ra\x05-\x81\x86\x88a\x04\xBCV[\x91PP\x82`\xA0\x83\x01R\x9A\x99PPPPPPPPPPV[`\0` \x82\x84\x03\x12\x15a\x05VW`\0\x80\xFD[\x81Q\x80\x15\x15\x81\x14a\x05fW`\0\x80\xFD[\x93\x92PPPV[\x86\x81R`\x80` \x82\x01R`\0a\x05\x87`\x80\x83\x01\x87\x89a\x04\xBCV[\x82\x81\x03`@\x84\x01Ra\x05\x9A\x81\x86\x88a\x04\xBCV[\x91PP\x82``\x83\x01R\x97\x96PPPPPPPV\xFE\xA2dipfsX\"\x12 $\x91<\xCBT\xB4A\xCAVzJ\x85\xD8pH\xF1\xED\xDD\xEE\xB1?H\xBC\xAA\x80\x80\xF7\x07O\x03cfdsolcC\0\x08\t\x003";
    /// The deployed bytecode of the contract.
    pub static AXELAREXECUTABLE_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct AxelarExecutable<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for AxelarExecutable<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for AxelarExecutable<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for AxelarExecutable<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for AxelarExecutable<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(AxelarExecutable))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> AxelarExecutable<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    AXELAREXECUTABLE_ABI.clone(),
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
                AXELAREXECUTABLE_ABI.clone(),
                AXELAREXECUTABLE_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `execute` (0x49160658) function
        pub fn execute(
            &self,
            command_id: [u8; 32],
            source_chain: ::std::string::String,
            source_address: ::std::string::String,
            payload: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [73, 22, 6, 88],
                    (command_id, source_chain, source_address, payload),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `executeWithToken` (0x1a98b2e0) function
        pub fn execute_with_token(
            &self,
            command_id: [u8; 32],
            source_chain: ::std::string::String,
            source_address: ::std::string::String,
            payload: ::ethers::core::types::Bytes,
            token_symbol: ::std::string::String,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [26, 152, 178, 224],
                    (
                        command_id,
                        source_chain,
                        source_address,
                        payload,
                        token_symbol,
                        amount,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `gateway` (0x116191b6) function
        pub fn gateway(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([17, 97, 145, 182], ())
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for AxelarExecutable<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `InvalidAddress` with signature `InvalidAddress()` and selector `0xe6c4247b`
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
    #[etherror(name = "InvalidAddress", abi = "InvalidAddress()")]
    pub struct InvalidAddress;
    ///Custom Error type `NotApprovedByGateway` with signature `NotApprovedByGateway()` and selector `0x500c44b4`
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
    #[etherror(name = "NotApprovedByGateway", abi = "NotApprovedByGateway()")]
    pub struct NotApprovedByGateway;
    ///Container type for all of the contract's custom errors
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum AxelarExecutableErrors {
        InvalidAddress(InvalidAddress),
        NotApprovedByGateway(NotApprovedByGateway),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for AxelarExecutableErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <InvalidAddress as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidAddress(decoded));
            }
            if let Ok(decoded) = <NotApprovedByGateway as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::NotApprovedByGateway(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for AxelarExecutableErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::InvalidAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NotApprovedByGateway(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for AxelarExecutableErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <InvalidAddress as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NotApprovedByGateway as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for AxelarExecutableErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::InvalidAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::NotApprovedByGateway(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for AxelarExecutableErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<InvalidAddress> for AxelarExecutableErrors {
        fn from(value: InvalidAddress) -> Self {
            Self::InvalidAddress(value)
        }
    }
    impl ::core::convert::From<NotApprovedByGateway> for AxelarExecutableErrors {
        fn from(value: NotApprovedByGateway) -> Self {
            Self::NotApprovedByGateway(value)
        }
    }
    ///Container type for all input parameters for the `execute` function with signature `execute(bytes32,string,string,bytes)` and selector `0x49160658`
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
    #[ethcall(name = "execute", abi = "execute(bytes32,string,string,bytes)")]
    pub struct ExecuteCall {
        pub command_id: [u8; 32],
        pub source_chain: ::std::string::String,
        pub source_address: ::std::string::String,
        pub payload: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `executeWithToken` function with signature `executeWithToken(bytes32,string,string,bytes,string,uint256)` and selector `0x1a98b2e0`
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
    #[ethcall(
        name = "executeWithToken",
        abi = "executeWithToken(bytes32,string,string,bytes,string,uint256)"
    )]
    pub struct ExecuteWithTokenCall {
        pub command_id: [u8; 32],
        pub source_chain: ::std::string::String,
        pub source_address: ::std::string::String,
        pub payload: ::ethers::core::types::Bytes,
        pub token_symbol: ::std::string::String,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `gateway` function with signature `gateway()` and selector `0x116191b6`
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
    #[ethcall(name = "gateway", abi = "gateway()")]
    pub struct GatewayCall;
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum AxelarExecutableCalls {
        Execute(ExecuteCall),
        ExecuteWithToken(ExecuteWithTokenCall),
        Gateway(GatewayCall),
    }
    impl ::ethers::core::abi::AbiDecode for AxelarExecutableCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <ExecuteCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Execute(decoded));
            }
            if let Ok(decoded) = <ExecuteWithTokenCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExecuteWithToken(decoded));
            }
            if let Ok(decoded) = <GatewayCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Gateway(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for AxelarExecutableCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Execute(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ExecuteWithToken(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Gateway(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for AxelarExecutableCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Execute(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExecuteWithToken(element) => ::core::fmt::Display::fmt(element, f),
                Self::Gateway(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ExecuteCall> for AxelarExecutableCalls {
        fn from(value: ExecuteCall) -> Self {
            Self::Execute(value)
        }
    }
    impl ::core::convert::From<ExecuteWithTokenCall> for AxelarExecutableCalls {
        fn from(value: ExecuteWithTokenCall) -> Self {
            Self::ExecuteWithToken(value)
        }
    }
    impl ::core::convert::From<GatewayCall> for AxelarExecutableCalls {
        fn from(value: GatewayCall) -> Self {
            Self::Gateway(value)
        }
    }
    ///Container type for all return fields from the `gateway` function with signature `gateway()` and selector `0x116191b6`
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
    pub struct GatewayReturn(pub ::ethers::core::types::Address);
}

pub use message_receiver::*;
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
pub mod message_receiver {
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
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_gasReceiver"),
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
                (
                    ::std::borrow::ToOwned::to_owned("message"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("message"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("sourceChain"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("sourceChain"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
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
                    ::std::borrow::ToOwned::to_owned("Executed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Executed"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("_from"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("_value"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
            ]),
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
    pub static MESSAGERECEIVER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xA0`@R`@Q\x80`@\x01`@R\x80`\x07\x81R` \x01\x7Fno data\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81RP`\0\x90\x81b\0\0J\x91\x90b\0\x03\x9DV[P4\x80\x15b\0\0XW`\0\x80\xFD[P`@Qb\0\x16z8\x03\x80b\0\x16z\x839\x81\x81\x01`@R\x81\x01\x90b\0\0~\x91\x90b\0\x04\xEEV[\x81`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03b\0\0\xE6W`@Q\x7F\xE6\xC4${\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x80\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPPPPPb\0\x055V[`\0\x81Q\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\"`\x04R`$`\0\xFD[`\0`\x02\x82\x04\x90P`\x01\x82\x16\x80b\0\x01\xA5W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03b\0\x01\xBBWb\0\x01\xBAb\0\x01]V[[P\x91\x90PV[`\0\x81\x90P\x81`\0R` `\0 \x90P\x91\x90PV[`\0` `\x1F\x83\x01\x04\x90P\x91\x90PV[`\0\x82\x82\x1B\x90P\x92\x91PPV[`\0`\x08\x83\x02b\0\x02%\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82b\0\x01\xE6V[b\0\x021\x86\x83b\0\x01\xE6V[\x95P\x80\x19\x84\x16\x93P\x80\x86\x16\x84\x17\x92PPP\x93\x92PPPV[`\0\x81\x90P\x91\x90PV[`\0\x81\x90P\x91\x90PV[`\0b\0\x02~b\0\x02xb\0\x02r\x84b\0\x02IV[b\0\x02SV[b\0\x02IV[\x90P\x91\x90PV[`\0\x81\x90P\x91\x90PV[b\0\x02\x9A\x83b\0\x02]V[b\0\x02\xB2b\0\x02\xA9\x82b\0\x02\x85V[\x84\x84Tb\0\x01\xF3V[\x82UPPPPV[`\0\x90V[b\0\x02\xC9b\0\x02\xBAV[b\0\x02\xD6\x81\x84\x84b\0\x02\x8FV[PPPV[[\x81\x81\x10\x15b\0\x02\xFEWb\0\x02\xF2`\0\x82b\0\x02\xBFV[`\x01\x81\x01\x90Pb\0\x02\xDCV[PPV[`\x1F\x82\x11\x15b\0\x03MWb\0\x03\x17\x81b\0\x01\xC1V[b\0\x03\"\x84b\0\x01\xD6V[\x81\x01` \x85\x10\x15b\0\x032W\x81\x90P[b\0\x03Jb\0\x03A\x85b\0\x01\xD6V[\x83\x01\x82b\0\x02\xDBV[PP[PPPV[`\0\x82\x82\x1C\x90P\x92\x91PPV[`\0b\0\x03r`\0\x19\x84`\x08\x02b\0\x03RV[\x19\x80\x83\x16\x91PP\x92\x91PPV[`\0b\0\x03\x8D\x83\x83b\0\x03_V[\x91P\x82`\x02\x02\x82\x17\x90P\x92\x91PPV[b\0\x03\xA8\x82b\0\x01#V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15b\0\x03\xC4Wb\0\x03\xC3b\0\x01.V[[b\0\x03\xD0\x82Tb\0\x01\x8CV[b\0\x03\xDD\x82\x82\x85b\0\x03\x02V[`\0` \x90P`\x1F\x83\x11`\x01\x81\x14b\0\x04\x15W`\0\x84\x15b\0\x04\0W\x82\x87\x01Q\x90P[b\0\x04\x0C\x85\x82b\0\x03\x7FV[\x86UPb\0\x04|V[`\x1F\x19\x84\x16b\0\x04%\x86b\0\x01\xC1V[`\0[\x82\x81\x10\x15b\0\x04OW\x84\x89\x01Q\x82U`\x01\x82\x01\x91P` \x85\x01\x94P` \x81\x01\x90Pb\0\x04(V[\x86\x83\x10\x15b\0\x04oW\x84\x89\x01Qb\0\x04k`\x1F\x89\x16\x82b\0\x03_V[\x83UP[`\x01`\x02\x88\x02\x01\x88UPPP[PPPPPPV[`\0\x80\xFD[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[`\0b\0\x04\xB6\x82b\0\x04\x89V[\x90P\x91\x90PV[b\0\x04\xC8\x81b\0\x04\xA9V[\x81\x14b\0\x04\xD4W`\0\x80\xFD[PV[`\0\x81Q\x90Pb\0\x04\xE8\x81b\0\x04\xBDV[\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15b\0\x05\x08Wb\0\x05\x07b\0\x04\x84V[[`\0b\0\x05\x18\x85\x82\x86\x01b\0\x04\xD7V[\x92PP` b\0\x05+\x85\x82\x86\x01b\0\x04\xD7V[\x91PP\x92P\x92\x90PV[`\x80Qa\x11\x1Cb\0\x05^`\09`\0\x81\x81`\xF0\x01R\x81\x81a\x010\x01Ra\x02\xDA\x01Ra\x11\x1C`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0WW`\x005`\xE0\x1C\x80c\x11a\x91\xB6\x14a\0\\W\x80c\x1A\x98\xB2\xE0\x14a\0zW\x80c\x1Co\xFAF\x14a\0\x96W\x80cI\x16\x06X\x14a\0\xB4W\x80c\xE2\x1F7\xCE\x14a\0\xD0W[`\0\x80\xFD[a\0da\0\xEEV[`@Qa\0q\x91\x90a\x05kV[`@Q\x80\x91\x03\x90\xF3[a\0\x94`\x04\x806\x03\x81\x01\x90a\0\x8F\x91\x90a\x06\xC1V[a\x01\x12V[\0[a\0\x9Ea\x02.V[`@Qa\0\xAB\x91\x90a\x08aV[`@Q\x80\x91\x03\x90\xF3[a\0\xCE`\x04\x806\x03\x81\x01\x90a\0\xC9\x91\x90a\x08\x83V[a\x02\xBCV[\0[a\0\xD8a\x03\xCCV[`@Qa\0\xE5\x91\x90a\x08aV[`@Q\x80\x91\x03\x90\xF3[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\0\x85\x85`@Qa\x01$\x92\x91\x90a\t\x8BV[`@Q\x80\x91\x03\x90 \x90P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x18v\xEE\xD9\x8C\x8C\x8C\x8C\x8C\x87\x8B\x8B\x8B`@Q\x8Ac\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x01\x97\x99\x98\x97\x96\x95\x94\x93\x92\x91\x90a\t\xEFV[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x01\xB6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\xDA\x91\x90a\n\xA3V[a\x02\x10W`@Q\x7FP\x0CD\xB4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x02!\x8A\x8A\x8A\x8A\x8A\x8A\x8A\x8A\x8Aa\x04ZV[PPPPPPPPPPPV[`\x01\x80Ta\x02;\x90a\n\xFFV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x02g\x90a\n\xFFV[\x80\x15a\x02\xB4W\x80`\x1F\x10a\x02\x89Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x02\xB4V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x02\x97W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[`\0\x82\x82`@Qa\x02\xCE\x92\x91\x90a\t\x8BV[`@Q\x80\x91\x03\x90 \x90P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c_ip\xC3\x89\x89\x89\x89\x89\x87`@Q\x87c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x03;\x96\x95\x94\x93\x92\x91\x90a\x0B0V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x03ZW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03~\x91\x90a\n\xA3V[a\x03\xB4W`@Q\x7FP\x0CD\xB4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x03\xC2\x87\x87\x87\x87\x87\x87a\x04eV[PPPPPPPPV[`\0\x80Ta\x03\xD9\x90a\n\xFFV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x04\x05\x90a\n\xFFV[\x80\x15a\x04RW\x80`\x1F\x10a\x04'Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x04RV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x045W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[PPPPPPPPPV[\x81\x81\x81\x01\x90a\x04t\x91\x90a\x0C\xA8V[`\0\x90\x81a\x04\x82\x91\x90a\x0E\x93V[P\x85\x85`\x01\x91\x82a\x04\x94\x92\x91\x90a\x0FpV[P3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x1EO\x9Fb\x9E\x9B\xF6\xA2\xE8\"\x99M\xB0E\x17\r\x14h\xC3\x1E\xE7\xFCvt+\xA5'm\xE1\x01\xC5\xFA`\0`@Qa\x04\xDC\x91\x90a\x10\xC4V[`@Q\x80\x91\x03\x90\xA2PPPPPPV[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[`\0\x81\x90P\x91\x90PV[`\0a\x051a\x05,a\x05'\x84a\x04\xECV[a\x05\x0CV[a\x04\xECV[\x90P\x91\x90PV[`\0a\x05C\x82a\x05\x16V[\x90P\x91\x90PV[`\0a\x05U\x82a\x058V[\x90P\x91\x90PV[a\x05e\x81a\x05JV[\x82RPPV[`\0` \x82\x01\x90Pa\x05\x80`\0\x83\x01\x84a\x05\\V[\x92\x91PPV[`\0`@Q\x90P\x90V[`\0\x80\xFD[`\0\x80\xFD[`\0\x81\x90P\x91\x90PV[a\x05\xAD\x81a\x05\x9AV[\x81\x14a\x05\xB8W`\0\x80\xFD[PV[`\0\x815\x90Pa\x05\xCA\x81a\x05\xA4V[\x92\x91PPV[`\0\x80\xFD[`\0\x80\xFD[`\0\x80\xFD[`\0\x80\x83`\x1F\x84\x01\x12a\x05\xF5Wa\x05\xF4a\x05\xD0V[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x06\x12Wa\x06\x11a\x05\xD5V[[` \x83\x01\x91P\x83`\x01\x82\x02\x83\x01\x11\x15a\x06.Wa\x06-a\x05\xDAV[[\x92P\x92\x90PV[`\0\x80\x83`\x1F\x84\x01\x12a\x06KWa\x06Ja\x05\xD0V[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x06hWa\x06ga\x05\xD5V[[` \x83\x01\x91P\x83`\x01\x82\x02\x83\x01\x11\x15a\x06\x84Wa\x06\x83a\x05\xDAV[[\x92P\x92\x90PV[`\0\x81\x90P\x91\x90PV[a\x06\x9E\x81a\x06\x8BV[\x81\x14a\x06\xA9W`\0\x80\xFD[PV[`\0\x815\x90Pa\x06\xBB\x81a\x06\x95V[\x92\x91PPV[`\0\x80`\0\x80`\0\x80`\0\x80`\0\x80`\xC0\x8B\x8D\x03\x12\x15a\x06\xE4Wa\x06\xE3a\x05\x90V[[`\0a\x06\xF2\x8D\x82\x8E\x01a\x05\xBBV[\x9APP` \x8B\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x07\x13Wa\x07\x12a\x05\x95V[[a\x07\x1F\x8D\x82\x8E\x01a\x05\xDFV[\x99P\x99PP`@\x8B\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x07BWa\x07Aa\x05\x95V[[a\x07N\x8D\x82\x8E\x01a\x05\xDFV[\x97P\x97PP``\x8B\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x07qWa\x07pa\x05\x95V[[a\x07}\x8D\x82\x8E\x01a\x065V[\x95P\x95PP`\x80\x8B\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x07\xA0Wa\x07\x9Fa\x05\x95V[[a\x07\xAC\x8D\x82\x8E\x01a\x05\xDFV[\x93P\x93PP`\xA0a\x07\xBF\x8D\x82\x8E\x01a\x06\xACV[\x91PP\x92\x95\x98\x9B\x91\x94\x97\x9AP\x92\x95\x98PV[`\0\x81Q\x90P\x91\x90PV[`\0\x82\x82R` \x82\x01\x90P\x92\x91PPV[`\0[\x83\x81\x10\x15a\x08\x0BW\x80\x82\x01Q\x81\x84\x01R` \x81\x01\x90Pa\x07\xF0V[`\0\x84\x84\x01RPPPPV[`\0`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[`\0a\x083\x82a\x07\xD1V[a\x08=\x81\x85a\x07\xDCV[\x93Pa\x08M\x81\x85` \x86\x01a\x07\xEDV[a\x08V\x81a\x08\x17V[\x84\x01\x91PP\x92\x91PPV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x08{\x81\x84a\x08(V[\x90P\x92\x91PPV[`\0\x80`\0\x80`\0\x80`\0`\x80\x88\x8A\x03\x12\x15a\x08\xA2Wa\x08\xA1a\x05\x90V[[`\0a\x08\xB0\x8A\x82\x8B\x01a\x05\xBBV[\x97PP` \x88\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x08\xD1Wa\x08\xD0a\x05\x95V[[a\x08\xDD\x8A\x82\x8B\x01a\x05\xDFV[\x96P\x96PP`@\x88\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\t\0Wa\x08\xFFa\x05\x95V[[a\t\x0C\x8A\x82\x8B\x01a\x05\xDFV[\x94P\x94PP``\x88\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\t/Wa\t.a\x05\x95V[[a\t;\x8A\x82\x8B\x01a\x065V[\x92P\x92PP\x92\x95\x98\x91\x94\x97P\x92\x95PV[`\0\x81\x90P\x92\x91PPV[\x82\x81\x837`\0\x83\x83\x01RPPPV[`\0a\tr\x83\x85a\tLV[\x93Pa\t\x7F\x83\x85\x84a\tWV[\x82\x84\x01\x90P\x93\x92PPPV[`\0a\t\x98\x82\x84\x86a\tfV[\x91P\x81\x90P\x93\x92PPPV[a\t\xAD\x81a\x05\x9AV[\x82RPPV[`\0a\t\xBF\x83\x85a\x07\xDCV[\x93Pa\t\xCC\x83\x85\x84a\tWV[a\t\xD5\x83a\x08\x17V[\x84\x01\x90P\x93\x92PPPV[a\t\xE9\x81a\x06\x8BV[\x82RPPV[`\0`\xC0\x82\x01\x90Pa\n\x04`\0\x83\x01\x8Ca\t\xA4V[\x81\x81\x03` \x83\x01Ra\n\x17\x81\x8A\x8Ca\t\xB3V[\x90P\x81\x81\x03`@\x83\x01Ra\n,\x81\x88\x8Aa\t\xB3V[\x90Pa\n;``\x83\x01\x87a\t\xA4V[\x81\x81\x03`\x80\x83\x01Ra\nN\x81\x85\x87a\t\xB3V[\x90Pa\n]`\xA0\x83\x01\x84a\t\xE0V[\x9A\x99PPPPPPPPPPV[`\0\x81\x15\x15\x90P\x91\x90PV[a\n\x80\x81a\nkV[\x81\x14a\n\x8BW`\0\x80\xFD[PV[`\0\x81Q\x90Pa\n\x9D\x81a\nwV[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\n\xB9Wa\n\xB8a\x05\x90V[[`\0a\n\xC7\x84\x82\x85\x01a\n\x8EV[\x91PP\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\"`\x04R`$`\0\xFD[`\0`\x02\x82\x04\x90P`\x01\x82\x16\x80a\x0B\x17W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x0B*Wa\x0B)a\n\xD0V[[P\x91\x90PV[`\0`\x80\x82\x01\x90Pa\x0BE`\0\x83\x01\x89a\t\xA4V[\x81\x81\x03` \x83\x01Ra\x0BX\x81\x87\x89a\t\xB3V[\x90P\x81\x81\x03`@\x83\x01Ra\x0Bm\x81\x85\x87a\t\xB3V[\x90Pa\x0B|``\x83\x01\x84a\t\xA4V[\x97\x96PPPPPPPV[`\0\x80\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[a\x0B\xC4\x82a\x08\x17V[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x0B\xE3Wa\x0B\xE2a\x0B\x8CV[[\x80`@RPPPV[`\0a\x0B\xF6a\x05\x86V[\x90Pa\x0C\x02\x82\x82a\x0B\xBBV[\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x0C\"Wa\x0C!a\x0B\x8CV[[a\x0C+\x82a\x08\x17V[\x90P` \x81\x01\x90P\x91\x90PV[`\0a\x0CKa\x0CF\x84a\x0C\x07V[a\x0B\xECV[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15a\x0CgWa\x0Cfa\x0B\x87V[[a\x0Cr\x84\x82\x85a\tWV[P\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12a\x0C\x8FWa\x0C\x8Ea\x05\xD0V[[\x815a\x0C\x9F\x84\x82` \x86\x01a\x0C8V[\x91PP\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x0C\xBEWa\x0C\xBDa\x05\x90V[[`\0\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0C\xDCWa\x0C\xDBa\x05\x95V[[a\x0C\xE8\x84\x82\x85\x01a\x0CzV[\x91PP\x92\x91PPV[`\0\x81\x90P\x81`\0R` `\0 \x90P\x91\x90PV[`\0` `\x1F\x83\x01\x04\x90P\x91\x90PV[`\0\x82\x82\x1B\x90P\x92\x91PPV[`\0`\x08\x83\x02a\rS\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82a\r\x16V[a\r]\x86\x83a\r\x16V[\x95P\x80\x19\x84\x16\x93P\x80\x86\x16\x84\x17\x92PPP\x93\x92PPPV[`\0a\r\x90a\r\x8Ba\r\x86\x84a\x06\x8BV[a\x05\x0CV[a\x06\x8BV[\x90P\x91\x90PV[`\0\x81\x90P\x91\x90PV[a\r\xAA\x83a\ruV[a\r\xBEa\r\xB6\x82a\r\x97V[\x84\x84Ta\r#V[\x82UPPPPV[`\0\x90V[a\r\xD3a\r\xC6V[a\r\xDE\x81\x84\x84a\r\xA1V[PPPV[[\x81\x81\x10\x15a\x0E\x02Wa\r\xF7`\0\x82a\r\xCBV[`\x01\x81\x01\x90Pa\r\xE4V[PPV[`\x1F\x82\x11\x15a\x0EGWa\x0E\x18\x81a\x0C\xF1V[a\x0E!\x84a\r\x06V[\x81\x01` \x85\x10\x15a\x0E0W\x81\x90P[a\x0EDa\x0E<\x85a\r\x06V[\x83\x01\x82a\r\xE3V[PP[PPPV[`\0\x82\x82\x1C\x90P\x92\x91PPV[`\0a\x0Ej`\0\x19\x84`\x08\x02a\x0ELV[\x19\x80\x83\x16\x91PP\x92\x91PPV[`\0a\x0E\x83\x83\x83a\x0EYV[\x91P\x82`\x02\x02\x82\x17\x90P\x92\x91PPV[a\x0E\x9C\x82a\x07\xD1V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0E\xB5Wa\x0E\xB4a\x0B\x8CV[[a\x0E\xBF\x82Ta\n\xFFV[a\x0E\xCA\x82\x82\x85a\x0E\x06V[`\0` \x90P`\x1F\x83\x11`\x01\x81\x14a\x0E\xFDW`\0\x84\x15a\x0E\xEBW\x82\x87\x01Q\x90P[a\x0E\xF5\x85\x82a\x0EwV[\x86UPa\x0F]V[`\x1F\x19\x84\x16a\x0F\x0B\x86a\x0C\xF1V[`\0[\x82\x81\x10\x15a\x0F3W\x84\x89\x01Q\x82U`\x01\x82\x01\x91P` \x85\x01\x94P` \x81\x01\x90Pa\x0F\x0EV[\x86\x83\x10\x15a\x0FPW\x84\x89\x01Qa\x0FL`\x1F\x89\x16\x82a\x0EYV[\x83UP[`\x01`\x02\x88\x02\x01\x88UPPP[PPPPPPV[`\0\x82\x90P\x92\x91PPV[a\x0Fz\x83\x83a\x0FeV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0F\x93Wa\x0F\x92a\x0B\x8CV[[a\x0F\x9D\x82Ta\n\xFFV[a\x0F\xA8\x82\x82\x85a\x0E\x06V[`\0`\x1F\x83\x11`\x01\x81\x14a\x0F\xD7W`\0\x84\x15a\x0F\xC5W\x82\x87\x015\x90P[a\x0F\xCF\x85\x82a\x0EwV[\x86UPa\x107V[`\x1F\x19\x84\x16a\x0F\xE5\x86a\x0C\xF1V[`\0[\x82\x81\x10\x15a\x10\rW\x84\x89\x015\x82U`\x01\x82\x01\x91P` \x85\x01\x94P` \x81\x01\x90Pa\x0F\xE8V[\x86\x83\x10\x15a\x10*W\x84\x89\x015a\x10&`\x1F\x89\x16\x82a\x0EYV[\x83UP[`\x01`\x02\x88\x02\x01\x88UPPP[PPPPPPPV[`\0\x81Ta\x10M\x81a\n\xFFV[a\x10W\x81\x86a\x07\xDCV[\x94P`\x01\x82\x16`\0\x81\x14a\x10rW`\x01\x81\x14a\x10\x88Wa\x10\xBBV[`\xFF\x19\x83\x16\x86R\x81\x15\x15` \x02\x86\x01\x93Pa\x10\xBBV[a\x10\x91\x85a\x0C\xF1V[`\0[\x83\x81\x10\x15a\x10\xB3W\x81T\x81\x89\x01R`\x01\x82\x01\x91P` \x81\x01\x90Pa\x10\x94V[\x80\x88\x01\x95PPP[PPP\x92\x91PPV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x10\xDE\x81\x84a\x10@V[\x90P\x92\x91PPV\xFE\xA2dipfsX\"\x12 \xF2\xD4\x18hWi\xF3,\x1B\xFDOD\xA9\x13$\x85\x14\xB0\xC1z-\xD8\xEA\x9F?\xAB\t\x7F^\x80\xDD0dsolcC\0\x08\x13\x003";
    /// The bytecode of the contract.
    pub static MESSAGERECEIVER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0WW`\x005`\xE0\x1C\x80c\x11a\x91\xB6\x14a\0\\W\x80c\x1A\x98\xB2\xE0\x14a\0zW\x80c\x1Co\xFAF\x14a\0\x96W\x80cI\x16\x06X\x14a\0\xB4W\x80c\xE2\x1F7\xCE\x14a\0\xD0W[`\0\x80\xFD[a\0da\0\xEEV[`@Qa\0q\x91\x90a\x05kV[`@Q\x80\x91\x03\x90\xF3[a\0\x94`\x04\x806\x03\x81\x01\x90a\0\x8F\x91\x90a\x06\xC1V[a\x01\x12V[\0[a\0\x9Ea\x02.V[`@Qa\0\xAB\x91\x90a\x08aV[`@Q\x80\x91\x03\x90\xF3[a\0\xCE`\x04\x806\x03\x81\x01\x90a\0\xC9\x91\x90a\x08\x83V[a\x02\xBCV[\0[a\0\xD8a\x03\xCCV[`@Qa\0\xE5\x91\x90a\x08aV[`@Q\x80\x91\x03\x90\xF3[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\0\x85\x85`@Qa\x01$\x92\x91\x90a\t\x8BV[`@Q\x80\x91\x03\x90 \x90P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x18v\xEE\xD9\x8C\x8C\x8C\x8C\x8C\x87\x8B\x8B\x8B`@Q\x8Ac\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x01\x97\x99\x98\x97\x96\x95\x94\x93\x92\x91\x90a\t\xEFV[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x01\xB6W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01\xDA\x91\x90a\n\xA3V[a\x02\x10W`@Q\x7FP\x0CD\xB4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x02!\x8A\x8A\x8A\x8A\x8A\x8A\x8A\x8A\x8Aa\x04ZV[PPPPPPPPPPPV[`\x01\x80Ta\x02;\x90a\n\xFFV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x02g\x90a\n\xFFV[\x80\x15a\x02\xB4W\x80`\x1F\x10a\x02\x89Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x02\xB4V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x02\x97W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[`\0\x82\x82`@Qa\x02\xCE\x92\x91\x90a\t\x8BV[`@Q\x80\x91\x03\x90 \x90P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c_ip\xC3\x89\x89\x89\x89\x89\x87`@Q\x87c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x03;\x96\x95\x94\x93\x92\x91\x90a\x0B0V[` `@Q\x80\x83\x03\x81`\0\x87Z\xF1\x15\x80\x15a\x03ZW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x03~\x91\x90a\n\xA3V[a\x03\xB4W`@Q\x7FP\x0CD\xB4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x03\xC2\x87\x87\x87\x87\x87\x87a\x04eV[PPPPPPPPV[`\0\x80Ta\x03\xD9\x90a\n\xFFV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x04\x05\x90a\n\xFFV[\x80\x15a\x04RW\x80`\x1F\x10a\x04'Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x04RV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x045W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[PPPPPPPPPV[\x81\x81\x81\x01\x90a\x04t\x91\x90a\x0C\xA8V[`\0\x90\x81a\x04\x82\x91\x90a\x0E\x93V[P\x85\x85`\x01\x91\x82a\x04\x94\x92\x91\x90a\x0FpV[P3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x1EO\x9Fb\x9E\x9B\xF6\xA2\xE8\"\x99M\xB0E\x17\r\x14h\xC3\x1E\xE7\xFCvt+\xA5'm\xE1\x01\xC5\xFA`\0`@Qa\x04\xDC\x91\x90a\x10\xC4V[`@Q\x80\x91\x03\x90\xA2PPPPPPV[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[`\0\x81\x90P\x91\x90PV[`\0a\x051a\x05,a\x05'\x84a\x04\xECV[a\x05\x0CV[a\x04\xECV[\x90P\x91\x90PV[`\0a\x05C\x82a\x05\x16V[\x90P\x91\x90PV[`\0a\x05U\x82a\x058V[\x90P\x91\x90PV[a\x05e\x81a\x05JV[\x82RPPV[`\0` \x82\x01\x90Pa\x05\x80`\0\x83\x01\x84a\x05\\V[\x92\x91PPV[`\0`@Q\x90P\x90V[`\0\x80\xFD[`\0\x80\xFD[`\0\x81\x90P\x91\x90PV[a\x05\xAD\x81a\x05\x9AV[\x81\x14a\x05\xB8W`\0\x80\xFD[PV[`\0\x815\x90Pa\x05\xCA\x81a\x05\xA4V[\x92\x91PPV[`\0\x80\xFD[`\0\x80\xFD[`\0\x80\xFD[`\0\x80\x83`\x1F\x84\x01\x12a\x05\xF5Wa\x05\xF4a\x05\xD0V[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x06\x12Wa\x06\x11a\x05\xD5V[[` \x83\x01\x91P\x83`\x01\x82\x02\x83\x01\x11\x15a\x06.Wa\x06-a\x05\xDAV[[\x92P\x92\x90PV[`\0\x80\x83`\x1F\x84\x01\x12a\x06KWa\x06Ja\x05\xD0V[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x06hWa\x06ga\x05\xD5V[[` \x83\x01\x91P\x83`\x01\x82\x02\x83\x01\x11\x15a\x06\x84Wa\x06\x83a\x05\xDAV[[\x92P\x92\x90PV[`\0\x81\x90P\x91\x90PV[a\x06\x9E\x81a\x06\x8BV[\x81\x14a\x06\xA9W`\0\x80\xFD[PV[`\0\x815\x90Pa\x06\xBB\x81a\x06\x95V[\x92\x91PPV[`\0\x80`\0\x80`\0\x80`\0\x80`\0\x80`\xC0\x8B\x8D\x03\x12\x15a\x06\xE4Wa\x06\xE3a\x05\x90V[[`\0a\x06\xF2\x8D\x82\x8E\x01a\x05\xBBV[\x9APP` \x8B\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x07\x13Wa\x07\x12a\x05\x95V[[a\x07\x1F\x8D\x82\x8E\x01a\x05\xDFV[\x99P\x99PP`@\x8B\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x07BWa\x07Aa\x05\x95V[[a\x07N\x8D\x82\x8E\x01a\x05\xDFV[\x97P\x97PP``\x8B\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x07qWa\x07pa\x05\x95V[[a\x07}\x8D\x82\x8E\x01a\x065V[\x95P\x95PP`\x80\x8B\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x07\xA0Wa\x07\x9Fa\x05\x95V[[a\x07\xAC\x8D\x82\x8E\x01a\x05\xDFV[\x93P\x93PP`\xA0a\x07\xBF\x8D\x82\x8E\x01a\x06\xACV[\x91PP\x92\x95\x98\x9B\x91\x94\x97\x9AP\x92\x95\x98PV[`\0\x81Q\x90P\x91\x90PV[`\0\x82\x82R` \x82\x01\x90P\x92\x91PPV[`\0[\x83\x81\x10\x15a\x08\x0BW\x80\x82\x01Q\x81\x84\x01R` \x81\x01\x90Pa\x07\xF0V[`\0\x84\x84\x01RPPPPV[`\0`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[`\0a\x083\x82a\x07\xD1V[a\x08=\x81\x85a\x07\xDCV[\x93Pa\x08M\x81\x85` \x86\x01a\x07\xEDV[a\x08V\x81a\x08\x17V[\x84\x01\x91PP\x92\x91PPV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x08{\x81\x84a\x08(V[\x90P\x92\x91PPV[`\0\x80`\0\x80`\0\x80`\0`\x80\x88\x8A\x03\x12\x15a\x08\xA2Wa\x08\xA1a\x05\x90V[[`\0a\x08\xB0\x8A\x82\x8B\x01a\x05\xBBV[\x97PP` \x88\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x08\xD1Wa\x08\xD0a\x05\x95V[[a\x08\xDD\x8A\x82\x8B\x01a\x05\xDFV[\x96P\x96PP`@\x88\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\t\0Wa\x08\xFFa\x05\x95V[[a\t\x0C\x8A\x82\x8B\x01a\x05\xDFV[\x94P\x94PP``\x88\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\t/Wa\t.a\x05\x95V[[a\t;\x8A\x82\x8B\x01a\x065V[\x92P\x92PP\x92\x95\x98\x91\x94\x97P\x92\x95PV[`\0\x81\x90P\x92\x91PPV[\x82\x81\x837`\0\x83\x83\x01RPPPV[`\0a\tr\x83\x85a\tLV[\x93Pa\t\x7F\x83\x85\x84a\tWV[\x82\x84\x01\x90P\x93\x92PPPV[`\0a\t\x98\x82\x84\x86a\tfV[\x91P\x81\x90P\x93\x92PPPV[a\t\xAD\x81a\x05\x9AV[\x82RPPV[`\0a\t\xBF\x83\x85a\x07\xDCV[\x93Pa\t\xCC\x83\x85\x84a\tWV[a\t\xD5\x83a\x08\x17V[\x84\x01\x90P\x93\x92PPPV[a\t\xE9\x81a\x06\x8BV[\x82RPPV[`\0`\xC0\x82\x01\x90Pa\n\x04`\0\x83\x01\x8Ca\t\xA4V[\x81\x81\x03` \x83\x01Ra\n\x17\x81\x8A\x8Ca\t\xB3V[\x90P\x81\x81\x03`@\x83\x01Ra\n,\x81\x88\x8Aa\t\xB3V[\x90Pa\n;``\x83\x01\x87a\t\xA4V[\x81\x81\x03`\x80\x83\x01Ra\nN\x81\x85\x87a\t\xB3V[\x90Pa\n]`\xA0\x83\x01\x84a\t\xE0V[\x9A\x99PPPPPPPPPPV[`\0\x81\x15\x15\x90P\x91\x90PV[a\n\x80\x81a\nkV[\x81\x14a\n\x8BW`\0\x80\xFD[PV[`\0\x81Q\x90Pa\n\x9D\x81a\nwV[\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\n\xB9Wa\n\xB8a\x05\x90V[[`\0a\n\xC7\x84\x82\x85\x01a\n\x8EV[\x91PP\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\"`\x04R`$`\0\xFD[`\0`\x02\x82\x04\x90P`\x01\x82\x16\x80a\x0B\x17W`\x7F\x82\x16\x91P[` \x82\x10\x81\x03a\x0B*Wa\x0B)a\n\xD0V[[P\x91\x90PV[`\0`\x80\x82\x01\x90Pa\x0BE`\0\x83\x01\x89a\t\xA4V[\x81\x81\x03` \x83\x01Ra\x0BX\x81\x87\x89a\t\xB3V[\x90P\x81\x81\x03`@\x83\x01Ra\x0Bm\x81\x85\x87a\t\xB3V[\x90Pa\x0B|``\x83\x01\x84a\t\xA4V[\x97\x96PPPPPPPV[`\0\x80\xFD[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`A`\x04R`$`\0\xFD[a\x0B\xC4\x82a\x08\x17V[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x0B\xE3Wa\x0B\xE2a\x0B\x8CV[[\x80`@RPPPV[`\0a\x0B\xF6a\x05\x86V[\x90Pa\x0C\x02\x82\x82a\x0B\xBBV[\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\x0C\"Wa\x0C!a\x0B\x8CV[[a\x0C+\x82a\x08\x17V[\x90P` \x81\x01\x90P\x91\x90PV[`\0a\x0CKa\x0CF\x84a\x0C\x07V[a\x0B\xECV[\x90P\x82\x81R` \x81\x01\x84\x84\x84\x01\x11\x15a\x0CgWa\x0Cfa\x0B\x87V[[a\x0Cr\x84\x82\x85a\tWV[P\x93\x92PPPV[`\0\x82`\x1F\x83\x01\x12a\x0C\x8FWa\x0C\x8Ea\x05\xD0V[[\x815a\x0C\x9F\x84\x82` \x86\x01a\x0C8V[\x91PP\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a\x0C\xBEWa\x0C\xBDa\x05\x90V[[`\0\x82\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0C\xDCWa\x0C\xDBa\x05\x95V[[a\x0C\xE8\x84\x82\x85\x01a\x0CzV[\x91PP\x92\x91PPV[`\0\x81\x90P\x81`\0R` `\0 \x90P\x91\x90PV[`\0` `\x1F\x83\x01\x04\x90P\x91\x90PV[`\0\x82\x82\x1B\x90P\x92\x91PPV[`\0`\x08\x83\x02a\rS\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82a\r\x16V[a\r]\x86\x83a\r\x16V[\x95P\x80\x19\x84\x16\x93P\x80\x86\x16\x84\x17\x92PPP\x93\x92PPPV[`\0a\r\x90a\r\x8Ba\r\x86\x84a\x06\x8BV[a\x05\x0CV[a\x06\x8BV[\x90P\x91\x90PV[`\0\x81\x90P\x91\x90PV[a\r\xAA\x83a\ruV[a\r\xBEa\r\xB6\x82a\r\x97V[\x84\x84Ta\r#V[\x82UPPPPV[`\0\x90V[a\r\xD3a\r\xC6V[a\r\xDE\x81\x84\x84a\r\xA1V[PPPV[[\x81\x81\x10\x15a\x0E\x02Wa\r\xF7`\0\x82a\r\xCBV[`\x01\x81\x01\x90Pa\r\xE4V[PPV[`\x1F\x82\x11\x15a\x0EGWa\x0E\x18\x81a\x0C\xF1V[a\x0E!\x84a\r\x06V[\x81\x01` \x85\x10\x15a\x0E0W\x81\x90P[a\x0EDa\x0E<\x85a\r\x06V[\x83\x01\x82a\r\xE3V[PP[PPPV[`\0\x82\x82\x1C\x90P\x92\x91PPV[`\0a\x0Ej`\0\x19\x84`\x08\x02a\x0ELV[\x19\x80\x83\x16\x91PP\x92\x91PPV[`\0a\x0E\x83\x83\x83a\x0EYV[\x91P\x82`\x02\x02\x82\x17\x90P\x92\x91PPV[a\x0E\x9C\x82a\x07\xD1V[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0E\xB5Wa\x0E\xB4a\x0B\x8CV[[a\x0E\xBF\x82Ta\n\xFFV[a\x0E\xCA\x82\x82\x85a\x0E\x06V[`\0` \x90P`\x1F\x83\x11`\x01\x81\x14a\x0E\xFDW`\0\x84\x15a\x0E\xEBW\x82\x87\x01Q\x90P[a\x0E\xF5\x85\x82a\x0EwV[\x86UPa\x0F]V[`\x1F\x19\x84\x16a\x0F\x0B\x86a\x0C\xF1V[`\0[\x82\x81\x10\x15a\x0F3W\x84\x89\x01Q\x82U`\x01\x82\x01\x91P` \x85\x01\x94P` \x81\x01\x90Pa\x0F\x0EV[\x86\x83\x10\x15a\x0FPW\x84\x89\x01Qa\x0FL`\x1F\x89\x16\x82a\x0EYV[\x83UP[`\x01`\x02\x88\x02\x01\x88UPPP[PPPPPPV[`\0\x82\x90P\x92\x91PPV[a\x0Fz\x83\x83a\x0FeV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0F\x93Wa\x0F\x92a\x0B\x8CV[[a\x0F\x9D\x82Ta\n\xFFV[a\x0F\xA8\x82\x82\x85a\x0E\x06V[`\0`\x1F\x83\x11`\x01\x81\x14a\x0F\xD7W`\0\x84\x15a\x0F\xC5W\x82\x87\x015\x90P[a\x0F\xCF\x85\x82a\x0EwV[\x86UPa\x107V[`\x1F\x19\x84\x16a\x0F\xE5\x86a\x0C\xF1V[`\0[\x82\x81\x10\x15a\x10\rW\x84\x89\x015\x82U`\x01\x82\x01\x91P` \x85\x01\x94P` \x81\x01\x90Pa\x0F\xE8V[\x86\x83\x10\x15a\x10*W\x84\x89\x015a\x10&`\x1F\x89\x16\x82a\x0EYV[\x83UP[`\x01`\x02\x88\x02\x01\x88UPPP[PPPPPPPV[`\0\x81Ta\x10M\x81a\n\xFFV[a\x10W\x81\x86a\x07\xDCV[\x94P`\x01\x82\x16`\0\x81\x14a\x10rW`\x01\x81\x14a\x10\x88Wa\x10\xBBV[`\xFF\x19\x83\x16\x86R\x81\x15\x15` \x02\x86\x01\x93Pa\x10\xBBV[a\x10\x91\x85a\x0C\xF1V[`\0[\x83\x81\x10\x15a\x10\xB3W\x81T\x81\x89\x01R`\x01\x82\x01\x91P` \x81\x01\x90Pa\x10\x94V[\x80\x88\x01\x95PPP[PPP\x92\x91PPV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x10\xDE\x81\x84a\x10@V[\x90P\x92\x91PPV\xFE\xA2dipfsX\"\x12 \xF2\xD4\x18hWi\xF3,\x1B\xFDOD\xA9\x13$\x85\x14\xB0\xC1z-\xD8\xEA\x9F?\xAB\t\x7F^\x80\xDD0dsolcC\0\x08\x13\x003";
    /// The deployed bytecode of the contract.
    pub static MESSAGERECEIVER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct MessageReceiver<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for MessageReceiver<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for MessageReceiver<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for MessageReceiver<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for MessageReceiver<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(MessageReceiver))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> MessageReceiver<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    MESSAGERECEIVER_ABI.clone(),
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
                MESSAGERECEIVER_ABI.clone(),
                MESSAGERECEIVER_BYTECODE.clone().into(),
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
        ///Calls the contract's `message` (0xe21f37ce) function
        pub fn message(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([226, 31, 55, 206], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `sourceChain` (0x1c6ffa46) function
        pub fn source_chain(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([28, 111, 250, 70], ())
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `Executed` event
        pub fn executed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ExecutedFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ExecutedFilter,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for MessageReceiver<M> {
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
    pub enum MessageReceiverErrors {
        InvalidAddress(InvalidAddress),
        NotApprovedByGateway(NotApprovedByGateway),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for MessageReceiverErrors {
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
    impl ::ethers::core::abi::AbiEncode for MessageReceiverErrors {
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
    impl ::ethers::contract::ContractRevert for MessageReceiverErrors {
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
    impl ::core::fmt::Display for MessageReceiverErrors {
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
    impl ::core::convert::From<::std::string::String> for MessageReceiverErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<InvalidAddress> for MessageReceiverErrors {
        fn from(value: InvalidAddress) -> Self {
            Self::InvalidAddress(value)
        }
    }
    impl ::core::convert::From<NotApprovedByGateway> for MessageReceiverErrors {
        fn from(value: NotApprovedByGateway) -> Self {
            Self::NotApprovedByGateway(value)
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
    #[ethevent(name = "Executed", abi = "Executed(address,string)")]
    pub struct ExecutedFilter {
        #[ethevent(indexed)]
        pub from: ::ethers::core::types::Address,
        pub value: ::std::string::String,
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
    ///Container type for all input parameters for the `message` function with signature `message()` and selector `0xe21f37ce`
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
    #[ethcall(name = "message", abi = "message()")]
    pub struct MessageCall;
    ///Container type for all input parameters for the `sourceChain` function with signature `sourceChain()` and selector `0x1c6ffa46`
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
    #[ethcall(name = "sourceChain", abi = "sourceChain()")]
    pub struct SourceChainCall;
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum MessageReceiverCalls {
        Execute(ExecuteCall),
        ExecuteWithToken(ExecuteWithTokenCall),
        Gateway(GatewayCall),
        Message(MessageCall),
        SourceChain(SourceChainCall),
    }
    impl ::ethers::core::abi::AbiDecode for MessageReceiverCalls {
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
            if let Ok(decoded) = <MessageCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Message(decoded));
            }
            if let Ok(decoded) = <SourceChainCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SourceChain(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for MessageReceiverCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Execute(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ExecuteWithToken(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Gateway(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Message(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SourceChain(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for MessageReceiverCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Execute(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExecuteWithToken(element) => ::core::fmt::Display::fmt(element, f),
                Self::Gateway(element) => ::core::fmt::Display::fmt(element, f),
                Self::Message(element) => ::core::fmt::Display::fmt(element, f),
                Self::SourceChain(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ExecuteCall> for MessageReceiverCalls {
        fn from(value: ExecuteCall) -> Self {
            Self::Execute(value)
        }
    }
    impl ::core::convert::From<ExecuteWithTokenCall> for MessageReceiverCalls {
        fn from(value: ExecuteWithTokenCall) -> Self {
            Self::ExecuteWithToken(value)
        }
    }
    impl ::core::convert::From<GatewayCall> for MessageReceiverCalls {
        fn from(value: GatewayCall) -> Self {
            Self::Gateway(value)
        }
    }
    impl ::core::convert::From<MessageCall> for MessageReceiverCalls {
        fn from(value: MessageCall) -> Self {
            Self::Message(value)
        }
    }
    impl ::core::convert::From<SourceChainCall> for MessageReceiverCalls {
        fn from(value: SourceChainCall) -> Self {
            Self::SourceChain(value)
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
    ///Container type for all return fields from the `message` function with signature `message()` and selector `0xe21f37ce`
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
    pub struct MessageReturn(pub ::std::string::String);
    ///Container type for all return fields from the `sourceChain` function with signature `sourceChain()` and selector `0x1c6ffa46`
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
    pub struct SourceChainReturn(pub ::std::string::String);
}

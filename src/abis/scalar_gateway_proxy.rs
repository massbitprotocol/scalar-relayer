pub use scalar_gateway_proxy::*;
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
pub mod scalar_gateway_proxy {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("gatewayImplementation"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("params"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("bytes"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("getAddress"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getAddress"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("key"),
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
                    ::std::borrow::ToOwned::to_owned("getBool"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getBool"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("key"),
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
                (
                    ::std::borrow::ToOwned::to_owned("getBytes"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getBytes"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("key"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getInt"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getInt"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("key"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Int(256usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("int256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("getString"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getString"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("key"),
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
                    ::std::borrow::ToOwned::to_owned("getUint"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("getUint"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("key"),
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
                    ::std::borrow::ToOwned::to_owned("setup"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setup"),
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
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("InvalidImplementation"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InvalidImplementation",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NativeCurrencyNotAccepted"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "NativeCurrencyNotAccepted",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("SetupFailed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("SetupFailed"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
            ]),
            receive: true,
            fallback: true,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static SCALARGATEWAYPROXY_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa\x07\xE98\x03\x80a\x07\xE9\x839\x81\x01`@\x81\x90Ra\0/\x91a\x01\xCDV[\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC`\0R`\x02` R\x7F\x11\x14\x1FFli\xFD@\x9E\x19\x90\xE0c\xB4\x9C\xD6\xD6\x1E\xD2\xEC\xFF'\xA2\xE4\x02\xE2Y\xCAk\x9A\x01\xA3\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x84\x16\x17\x90U`\x01`\x01`\xA0\x1B\x03\x82\x16;a\0\xBAW`@Qc4\n\xAF\xCD`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x82`\x01`\x01`\xA0\x1B\x03\x16c\x9D\xED\x06\xDF`\xE0\x1B\x83`@Q`$\x01a\0\xDF\x91\x90a\x02\x9BV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16`\x01`\x01`\xE0\x1B\x03\x19\x90\x94\x16\x93\x90\x93\x17\x90\x92R\x90Qa\x01\x1D\x91\x90a\x02\xCEV[`\0`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80`\0\x81\x14a\x01XW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x01]V[``\x91P[PP\x90P\x80a\x01\x7FW`@Qc\x97\x90]\xFB`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPa\x02\xEAV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`\0[\x83\x81\x10\x15a\x01\xB8W\x81\x81\x01Q\x83\x82\x01R` \x01a\x01\xA0V[\x83\x81\x11\x15a\x01\xC7W`\0\x84\x84\x01R[PPPPV[`\0\x80`@\x83\x85\x03\x12\x15a\x01\xE0W`\0\x80\xFD[\x82Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x01\xF7W`\0\x80\xFD[` \x84\x01Q\x90\x92P`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a\x02\x14W`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x02(W`\0\x80\xFD[\x81Q\x81\x81\x11\x15a\x02:Wa\x02:a\x01\x87V[`@Q`\x1F\x82\x01`\x1F\x19\x90\x81\x16`?\x01\x16\x81\x01\x90\x83\x82\x11\x81\x83\x10\x17\x15a\x02bWa\x02ba\x01\x87V[\x81`@R\x82\x81R\x88` \x84\x87\x01\x01\x11\x15a\x02{W`\0\x80\xFD[a\x02\x8C\x83` \x83\x01` \x88\x01a\x01\x9DV[\x80\x95PPPPPP\x92P\x92\x90PV[` \x81R`\0\x82Q\x80` \x84\x01Ra\x02\xBA\x81`@\x85\x01` \x87\x01a\x01\x9DV[`\x1F\x01`\x1F\x19\x16\x91\x90\x91\x01`@\x01\x92\x91PPV[`\0\x82Qa\x02\xE0\x81\x84` \x87\x01a\x01\x9DV[\x91\x90\x91\x01\x92\x91PPV[a\x04\xF0\x80a\x02\xF9`\09`\0\xF3\xFE`\x80`@R`\x046\x10a\0tW`\x005`\xE0\x1C\x80c\x9D\xED\x06\xDF\x11a\0NW\x80c\x9D\xED\x06\xDF\x14a\x02\x0CW\x80c\xBD\x02\xD0\xF5\x14a\x02-W\x80c\xC01\xA1\x80\x14a\x02hW\x80c\xDC\x97\xD9b\x14a\x02\x88Wa\0\xABV[\x80c!\xF8\xA7!\x14a\x012W\x80cz\xE1\xCF\xCA\x14a\x01\x9FW\x80c\x98ny\x1A\x14a\x01\xDFWa\0\xABV[6a\0\xABW`@Q\x7F\x85\x8Dp\xBD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC`\0\x90\x81R`\x02` R\x7F\x11\x14\x1FFli\xFD@\x9E\x19\x90\xE0c\xB4\x9C\xD6\xD6\x1E\xD2\xEC\xFF'\xA2\xE4\x02\xE2Y\xCAk\x9A\x01\xA3Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x906\x90\x807`\0\x806`\0\x84Z\xF4=`\0\x80>\x80\x80\x15a\x01-W=`\0\xF3[=`\0\xFD[4\x80\x15a\x01>W`\0\x80\xFD[Pa\x01ua\x01M6`\x04a\x03tV[`\0\x90\x81R`\x02` R`@\x90 Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01\xABW`\0\x80\xFD[Pa\x01\xCFa\x01\xBA6`\x04a\x03tV[`\0\x90\x81R`\x04` R`@\x90 T`\xFF\x16\x90V[`@Q\x90\x15\x15\x81R` \x01a\x01\x96V[4\x80\x15a\x01\xEBW`\0\x80\xFD[Pa\x01\xFFa\x01\xFA6`\x04a\x03tV[a\x02\xB5V[`@Qa\x01\x96\x91\x90a\x03\xDAV[4\x80\x15a\x02\x18W`\0\x80\xFD[Pa\x02+a\x02'6`\x04a\x03\xF4V[PPV[\0[4\x80\x15a\x029W`\0\x80\xFD[Pa\x02Za\x02H6`\x04a\x03tV[`\0\x90\x81R` \x81\x90R`@\x90 T\x90V[`@Q\x90\x81R` \x01a\x01\x96V[4\x80\x15a\x02tW`\0\x80\xFD[Pa\x01\xFFa\x02\x836`\x04a\x03tV[a\x03WV[4\x80\x15a\x02\x94W`\0\x80\xFD[Pa\x02Za\x02\xA36`\x04a\x03tV[`\0\x90\x81R`\x05` R`@\x90 T\x90V[`\0\x81\x81R`\x01` R`@\x90 \x80T``\x91\x90a\x02\xD2\x90a\x04fV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x02\xFE\x90a\x04fV[\x80\x15a\x03KW\x80`\x1F\x10a\x03 Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x03KV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x03.W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x91\x90PV[`\0\x81\x81R`\x03` R`@\x90 \x80T``\x91\x90a\x02\xD2\x90a\x04fV[`\0` \x82\x84\x03\x12\x15a\x03\x86W`\0\x80\xFD[P5\x91\x90PV[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a\x03\xB3W` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a\x03\x97V[\x81\x81\x11\x15a\x03\xC5W`\0` \x83\x87\x01\x01R[P`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a\x03\xED` \x83\x01\x84a\x03\x8DV[\x93\x92PPPV[`\0\x80` \x83\x85\x03\x12\x15a\x04\x07W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x04\x1FW`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x043W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x04BW`\0\x80\xFD[\x86` \x82\x85\x01\x01\x11\x15a\x04TW`\0\x80\xFD[` \x92\x90\x92\x01\x96\x91\x95P\x90\x93PPPPV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x04zW`\x7F\x82\x16\x91P[` \x82\x10\x81\x14\x15a\x04\xB4W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV\xFE\xA2dipfsX\"\x12 \x87\x86\xCF\x0C\xB0q\xE0\x1FL\xA7\x9F^\x90\xE3j3q\xFF+\x96\xF8\x82\xCF\x10\x03\xDF\xBF\xDCD!i\xFBdsolcC\0\x08\t\x003";
    /// The bytecode of the contract.
    pub static SCALARGATEWAYPROXY_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\0tW`\x005`\xE0\x1C\x80c\x9D\xED\x06\xDF\x11a\0NW\x80c\x9D\xED\x06\xDF\x14a\x02\x0CW\x80c\xBD\x02\xD0\xF5\x14a\x02-W\x80c\xC01\xA1\x80\x14a\x02hW\x80c\xDC\x97\xD9b\x14a\x02\x88Wa\0\xABV[\x80c!\xF8\xA7!\x14a\x012W\x80cz\xE1\xCF\xCA\x14a\x01\x9FW\x80c\x98ny\x1A\x14a\x01\xDFWa\0\xABV[6a\0\xABW`@Q\x7F\x85\x8Dp\xBD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC`\0\x90\x81R`\x02` R\x7F\x11\x14\x1FFli\xFD@\x9E\x19\x90\xE0c\xB4\x9C\xD6\xD6\x1E\xD2\xEC\xFF'\xA2\xE4\x02\xE2Y\xCAk\x9A\x01\xA3Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x906\x90\x807`\0\x806`\0\x84Z\xF4=`\0\x80>\x80\x80\x15a\x01-W=`\0\xF3[=`\0\xFD[4\x80\x15a\x01>W`\0\x80\xFD[Pa\x01ua\x01M6`\x04a\x03tV[`\0\x90\x81R`\x02` R`@\x90 Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x01\xABW`\0\x80\xFD[Pa\x01\xCFa\x01\xBA6`\x04a\x03tV[`\0\x90\x81R`\x04` R`@\x90 T`\xFF\x16\x90V[`@Q\x90\x15\x15\x81R` \x01a\x01\x96V[4\x80\x15a\x01\xEBW`\0\x80\xFD[Pa\x01\xFFa\x01\xFA6`\x04a\x03tV[a\x02\xB5V[`@Qa\x01\x96\x91\x90a\x03\xDAV[4\x80\x15a\x02\x18W`\0\x80\xFD[Pa\x02+a\x02'6`\x04a\x03\xF4V[PPV[\0[4\x80\x15a\x029W`\0\x80\xFD[Pa\x02Za\x02H6`\x04a\x03tV[`\0\x90\x81R` \x81\x90R`@\x90 T\x90V[`@Q\x90\x81R` \x01a\x01\x96V[4\x80\x15a\x02tW`\0\x80\xFD[Pa\x01\xFFa\x02\x836`\x04a\x03tV[a\x03WV[4\x80\x15a\x02\x94W`\0\x80\xFD[Pa\x02Za\x02\xA36`\x04a\x03tV[`\0\x90\x81R`\x05` R`@\x90 T\x90V[`\0\x81\x81R`\x01` R`@\x90 \x80T``\x91\x90a\x02\xD2\x90a\x04fV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x02\xFE\x90a\x04fV[\x80\x15a\x03KW\x80`\x1F\x10a\x03 Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x03KV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x03.W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x91\x90PV[`\0\x81\x81R`\x03` R`@\x90 \x80T``\x91\x90a\x02\xD2\x90a\x04fV[`\0` \x82\x84\x03\x12\x15a\x03\x86W`\0\x80\xFD[P5\x91\x90PV[`\0\x81Q\x80\x84R`\0[\x81\x81\x10\x15a\x03\xB3W` \x81\x85\x01\x81\x01Q\x86\x83\x01\x82\x01R\x01a\x03\x97V[\x81\x81\x11\x15a\x03\xC5W`\0` \x83\x87\x01\x01R[P`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a\x03\xED` \x83\x01\x84a\x03\x8DV[\x93\x92PPPV[`\0\x80` \x83\x85\x03\x12\x15a\x04\x07W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x04\x1FW`\0\x80\xFD[\x81\x85\x01\x91P\x85`\x1F\x83\x01\x12a\x043W`\0\x80\xFD[\x815\x81\x81\x11\x15a\x04BW`\0\x80\xFD[\x86` \x82\x85\x01\x01\x11\x15a\x04TW`\0\x80\xFD[` \x92\x90\x92\x01\x96\x91\x95P\x90\x93PPPPV[`\x01\x81\x81\x1C\x90\x82\x16\x80a\x04zW`\x7F\x82\x16\x91P[` \x82\x10\x81\x14\x15a\x04\xB4W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV\xFE\xA2dipfsX\"\x12 \x87\x86\xCF\x0C\xB0q\xE0\x1FL\xA7\x9F^\x90\xE3j3q\xFF+\x96\xF8\x82\xCF\x10\x03\xDF\xBF\xDCD!i\xFBdsolcC\0\x08\t\x003";
    /// The deployed bytecode of the contract.
    pub static SCALARGATEWAYPROXY_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct ScalarGatewayProxy<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for ScalarGatewayProxy<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for ScalarGatewayProxy<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for ScalarGatewayProxy<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for ScalarGatewayProxy<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(ScalarGatewayProxy))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> ScalarGatewayProxy<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    SCALARGATEWAYPROXY_ABI.clone(),
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
                SCALARGATEWAYPROXY_ABI.clone(),
                SCALARGATEWAYPROXY_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `getAddress` (0x21f8a721) function
        pub fn get_address(
            &self,
            key: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([33, 248, 167, 33], key)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getBool` (0x7ae1cfca) function
        pub fn get_bool(
            &self,
            key: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([122, 225, 207, 202], key)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getBytes` (0xc031a180) function
        pub fn get_bytes(
            &self,
            key: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Bytes,
        > {
            self.0
                .method_hash([192, 49, 161, 128], key)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getInt` (0xdc97d962) function
        pub fn get_int(
            &self,
            key: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::I256> {
            self.0
                .method_hash([220, 151, 217, 98], key)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getString` (0x986e791a) function
        pub fn get_string(
            &self,
            key: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([152, 110, 121, 26], key)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getUint` (0xbd02d0f5) function
        pub fn get_uint(
            &self,
            key: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([189, 2, 208, 245], key)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setup` (0x9ded06df) function
        pub fn setup(
            &self,
            params: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([157, 237, 6, 223], params)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for ScalarGatewayProxy<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `InvalidImplementation` with signature `InvalidImplementation()` and selector `0x68155f9a`
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
    #[etherror(name = "InvalidImplementation", abi = "InvalidImplementation()")]
    pub struct InvalidImplementation;
    ///Custom Error type `NativeCurrencyNotAccepted` with signature `NativeCurrencyNotAccepted()` and selector `0x858d70bd`
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
    #[etherror(name = "NativeCurrencyNotAccepted", abi = "NativeCurrencyNotAccepted()")]
    pub struct NativeCurrencyNotAccepted;
    ///Custom Error type `SetupFailed` with signature `SetupFailed()` and selector `0x97905dfb`
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
    #[etherror(name = "SetupFailed", abi = "SetupFailed()")]
    pub struct SetupFailed;
    ///Container type for all of the contract's custom errors
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum ScalarGatewayProxyErrors {
        InvalidImplementation(InvalidImplementation),
        NativeCurrencyNotAccepted(NativeCurrencyNotAccepted),
        SetupFailed(SetupFailed),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for ScalarGatewayProxyErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <InvalidImplementation as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidImplementation(decoded));
            }
            if let Ok(decoded) = <NativeCurrencyNotAccepted as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::NativeCurrencyNotAccepted(decoded));
            }
            if let Ok(decoded) = <SetupFailed as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetupFailed(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ScalarGatewayProxyErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::InvalidImplementation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NativeCurrencyNotAccepted(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetupFailed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for ScalarGatewayProxyErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <InvalidImplementation as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NativeCurrencyNotAccepted as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <SetupFailed as ::ethers::contract::EthError>::selector() => true,
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for ScalarGatewayProxyErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::InvalidImplementation(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NativeCurrencyNotAccepted(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SetupFailed(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for ScalarGatewayProxyErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<InvalidImplementation> for ScalarGatewayProxyErrors {
        fn from(value: InvalidImplementation) -> Self {
            Self::InvalidImplementation(value)
        }
    }
    impl ::core::convert::From<NativeCurrencyNotAccepted> for ScalarGatewayProxyErrors {
        fn from(value: NativeCurrencyNotAccepted) -> Self {
            Self::NativeCurrencyNotAccepted(value)
        }
    }
    impl ::core::convert::From<SetupFailed> for ScalarGatewayProxyErrors {
        fn from(value: SetupFailed) -> Self {
            Self::SetupFailed(value)
        }
    }
    ///Container type for all input parameters for the `getAddress` function with signature `getAddress(bytes32)` and selector `0x21f8a721`
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
    #[ethcall(name = "getAddress", abi = "getAddress(bytes32)")]
    pub struct GetAddressCall {
        pub key: [u8; 32],
    }
    ///Container type for all input parameters for the `getBool` function with signature `getBool(bytes32)` and selector `0x7ae1cfca`
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
    #[ethcall(name = "getBool", abi = "getBool(bytes32)")]
    pub struct GetBoolCall {
        pub key: [u8; 32],
    }
    ///Container type for all input parameters for the `getBytes` function with signature `getBytes(bytes32)` and selector `0xc031a180`
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
    #[ethcall(name = "getBytes", abi = "getBytes(bytes32)")]
    pub struct GetBytesCall {
        pub key: [u8; 32],
    }
    ///Container type for all input parameters for the `getInt` function with signature `getInt(bytes32)` and selector `0xdc97d962`
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
    #[ethcall(name = "getInt", abi = "getInt(bytes32)")]
    pub struct GetIntCall {
        pub key: [u8; 32],
    }
    ///Container type for all input parameters for the `getString` function with signature `getString(bytes32)` and selector `0x986e791a`
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
    #[ethcall(name = "getString", abi = "getString(bytes32)")]
    pub struct GetStringCall {
        pub key: [u8; 32],
    }
    ///Container type for all input parameters for the `getUint` function with signature `getUint(bytes32)` and selector `0xbd02d0f5`
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
    #[ethcall(name = "getUint", abi = "getUint(bytes32)")]
    pub struct GetUintCall {
        pub key: [u8; 32],
    }
    ///Container type for all input parameters for the `setup` function with signature `setup(bytes)` and selector `0x9ded06df`
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
    #[ethcall(name = "setup", abi = "setup(bytes)")]
    pub struct SetupCall {
        pub params: ::ethers::core::types::Bytes,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum ScalarGatewayProxyCalls {
        GetAddress(GetAddressCall),
        GetBool(GetBoolCall),
        GetBytes(GetBytesCall),
        GetInt(GetIntCall),
        GetString(GetStringCall),
        GetUint(GetUintCall),
        Setup(SetupCall),
    }
    impl ::ethers::core::abi::AbiDecode for ScalarGatewayProxyCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <GetAddressCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetAddress(decoded));
            }
            if let Ok(decoded) = <GetBoolCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetBool(decoded));
            }
            if let Ok(decoded) = <GetBytesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetBytes(decoded));
            }
            if let Ok(decoded) = <GetIntCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetInt(decoded));
            }
            if let Ok(decoded) = <GetStringCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetString(decoded));
            }
            if let Ok(decoded) = <GetUintCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetUint(decoded));
            }
            if let Ok(decoded) = <SetupCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Setup(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ScalarGatewayProxyCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::GetAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetBool(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetBytes(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetInt(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetString(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetUint(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Setup(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for ScalarGatewayProxyCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::GetAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetBool(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetBytes(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetInt(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetString(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetUint(element) => ::core::fmt::Display::fmt(element, f),
                Self::Setup(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<GetAddressCall> for ScalarGatewayProxyCalls {
        fn from(value: GetAddressCall) -> Self {
            Self::GetAddress(value)
        }
    }
    impl ::core::convert::From<GetBoolCall> for ScalarGatewayProxyCalls {
        fn from(value: GetBoolCall) -> Self {
            Self::GetBool(value)
        }
    }
    impl ::core::convert::From<GetBytesCall> for ScalarGatewayProxyCalls {
        fn from(value: GetBytesCall) -> Self {
            Self::GetBytes(value)
        }
    }
    impl ::core::convert::From<GetIntCall> for ScalarGatewayProxyCalls {
        fn from(value: GetIntCall) -> Self {
            Self::GetInt(value)
        }
    }
    impl ::core::convert::From<GetStringCall> for ScalarGatewayProxyCalls {
        fn from(value: GetStringCall) -> Self {
            Self::GetString(value)
        }
    }
    impl ::core::convert::From<GetUintCall> for ScalarGatewayProxyCalls {
        fn from(value: GetUintCall) -> Self {
            Self::GetUint(value)
        }
    }
    impl ::core::convert::From<SetupCall> for ScalarGatewayProxyCalls {
        fn from(value: SetupCall) -> Self {
            Self::Setup(value)
        }
    }
    ///Container type for all return fields from the `getAddress` function with signature `getAddress(bytes32)` and selector `0x21f8a721`
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
    pub struct GetAddressReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `getBool` function with signature `getBool(bytes32)` and selector `0x7ae1cfca`
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
    pub struct GetBoolReturn(pub bool);
    ///Container type for all return fields from the `getBytes` function with signature `getBytes(bytes32)` and selector `0xc031a180`
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
    pub struct GetBytesReturn(pub ::ethers::core::types::Bytes);
    ///Container type for all return fields from the `getInt` function with signature `getInt(bytes32)` and selector `0xdc97d962`
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
    pub struct GetIntReturn(pub ::ethers::core::types::I256);
    ///Container type for all return fields from the `getString` function with signature `getString(bytes32)` and selector `0x986e791a`
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
    pub struct GetStringReturn(pub ::std::string::String);
    ///Container type for all return fields from the `getUint` function with signature `getUint(bytes32)` and selector `0xbd02d0f5`
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
    pub struct GetUintReturn(pub ::ethers::core::types::U256);
}

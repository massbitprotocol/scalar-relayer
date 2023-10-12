pub use scalar_gateway::*;
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
pub mod scalar_gateway {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("authModule_"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("tokenDeployer_"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("adminEpoch"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("adminEpoch"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("adminThreshold"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("adminThreshold"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("admins"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("admins"),
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
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address[]"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("allTokensFrozen"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("allTokensFrozen"),
                            inputs: ::std::vec![],
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("approveContractCall"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "approveContractCall",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("params"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("commandId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
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
                    ::std::borrow::ToOwned::to_owned("approveContractCallWithMint"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "approveContractCallWithMint",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("params"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("commandId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
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
                    ::std::borrow::ToOwned::to_owned("authModule"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("authModule"),
                            inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("burnToken"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("burnToken"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("params"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
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
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("callContract"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("callContract"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("destinationChain"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "destinationContractAddress",
                                    ),
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
                    ::std::borrow::ToOwned::to_owned("callContractWithToken"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "callContractWithToken",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("destinationChain"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "destinationContractAddress",
                                    ),
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
                                    name: ::std::borrow::ToOwned::to_owned("symbol"),
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
                    ::std::borrow::ToOwned::to_owned("contractId"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("contractId"),
                            inputs: ::std::vec![],
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("deployToken"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("deployToken"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("params"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
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
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("execute"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("execute"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("input"),
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
                    ::std::borrow::ToOwned::to_owned("governance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("governance"),
                            inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("implementation"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("implementation"),
                            inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("isCommandExecuted"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("isCommandExecuted"),
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
                    ::std::borrow::ToOwned::to_owned("isContractCallAndMintApproved"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "isContractCallAndMintApproved",
                            ),
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
                                    name: ::std::borrow::ToOwned::to_owned("contractAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("payloadHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("symbol"),
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
                    ::std::borrow::ToOwned::to_owned("isContractCallApproved"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "isContractCallApproved",
                            ),
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
                                    name: ::std::borrow::ToOwned::to_owned("contractAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("payloadHash"),
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
                    ::std::borrow::ToOwned::to_owned("mintLimiter"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("mintLimiter"),
                            inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("mintToken"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("mintToken"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("params"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
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
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("resetOperatorship"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("resetOperatorship"),
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
                    ::std::borrow::ToOwned::to_owned("sendToken"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("sendToken"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("destinationChain"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "destinationAddress",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("symbol"),
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
                    ::std::borrow::ToOwned::to_owned("setTokenMintLimits"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setTokenMintLimits"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("symbols"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::String,
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("limits"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256[]"),
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
                (
                    ::std::borrow::ToOwned::to_owned("tokenAddresses"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("tokenAddresses"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("symbol"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
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
                    ::std::borrow::ToOwned::to_owned("tokenDeployer"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("tokenDeployer"),
                            inputs: ::std::vec![],
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
                    ::std::borrow::ToOwned::to_owned("tokenFrozen"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("tokenFrozen"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
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
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Pure,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("tokenMintAmount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("tokenMintAmount"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("symbol"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
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
                    ::std::borrow::ToOwned::to_owned("tokenMintLimit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("tokenMintLimit"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("symbol"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
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
                    ::std::borrow::ToOwned::to_owned("transferGovernance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("transferGovernance"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newGovernance"),
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
                    ::std::borrow::ToOwned::to_owned("transferMintLimiter"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "transferMintLimiter",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newMintLimiter"),
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
                                    name: ::std::borrow::ToOwned::to_owned("newOperatorsData"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
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
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("upgrade"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("upgrade"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newImplementation"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "newImplementationCodeHash",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("setupParams"),
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
                    ::std::borrow::ToOwned::to_owned("validateContractCall"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "validateContractCall",
                            ),
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
                                    name: ::std::borrow::ToOwned::to_owned("payloadHash"),
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
                                    name: ::std::borrow::ToOwned::to_owned("valid"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("validateContractCallAndMint"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "validateContractCallAndMint",
                            ),
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
                                    name: ::std::borrow::ToOwned::to_owned("payloadHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("symbol"),
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
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("valid"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("ContractCall"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("ContractCall"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("destinationChain"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "destinationContractAddress",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("payloadHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("payload"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ContractCallApproved"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ContractCallApproved",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("commandId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("sourceChain"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("sourceAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("contractAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("payloadHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("sourceTxHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("sourceEventIndex"),
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
                    ::std::borrow::ToOwned::to_owned("ContractCallApprovedWithMint"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ContractCallApprovedWithMint",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("commandId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("sourceChain"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("sourceAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("contractAddress"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("payloadHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("symbol"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("sourceTxHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("sourceEventIndex"),
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
                    ::std::borrow::ToOwned::to_owned("ContractCallWithToken"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "ContractCallWithToken",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("destinationChain"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "destinationContractAddress",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("payloadHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("payload"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("symbol"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
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
                    ::std::borrow::ToOwned::to_owned("Executed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Executed"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("commandId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("GovernanceTransferred"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "GovernanceTransferred",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "previousGovernance",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newGovernance"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("MintLimiterTransferred"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "MintLimiterTransferred",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "previousGovernance",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newGovernance"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OperatorshipTransferred"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OperatorshipTransferred",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newOperatorsData"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("TokenDeployed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("TokenDeployed"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("symbol"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("tokenAddresses"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("TokenMintLimitUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "TokenMintLimitUpdated",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("symbol"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("limit"),
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
                    ::std::borrow::ToOwned::to_owned("TokenSent"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("TokenSent"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("sender"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("destinationChain"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "destinationAddress",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("symbol"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("amount"),
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
                    ::std::borrow::ToOwned::to_owned("Upgraded"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Upgraded"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("implementation"),
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
                    ::std::borrow::ToOwned::to_owned("BurnFailed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("BurnFailed"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("symbol"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("ExceedMintLimit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("ExceedMintLimit"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("symbol"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidAmount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidAmount"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidAuthModule"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidAuthModule"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidChainId"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidChainId"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidCodeHash"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidCodeHash"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidCommands"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidCommands"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidGovernance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidGovernance"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
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
                    ::std::borrow::ToOwned::to_owned("InvalidMintLimiter"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("InvalidMintLimiter"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidSetMintLimitsParams"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InvalidSetMintLimitsParams",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("InvalidTokenDeployer"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "InvalidTokenDeployer",
                            ),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("MintFailed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("MintFailed"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("symbol"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NotGovernance"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("NotGovernance"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NotMintLimiter"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("NotMintLimiter"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NotProxy"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("NotProxy"),
                            inputs: ::std::vec![],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NotSelf"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("NotSelf"),
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
                (
                    ::std::borrow::ToOwned::to_owned("TokenAlreadyExists"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("TokenAlreadyExists"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("symbol"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("TokenContractDoesNotExist"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "TokenContractDoesNotExist",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("token"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("TokenDeployFailed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("TokenDeployFailed"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("symbol"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("TokenDoesNotExist"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned("TokenDoesNotExist"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("symbol"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("TokenTransferFailed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::AbiError {
                            name: ::std::borrow::ToOwned::to_owned(
                                "TokenTransferFailed",
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
    pub static SCALARGATEWAY_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xE0`@R0`\xC0R4\x80\x15b\0\0\x15W`\0\x80\xFD[P`@Qb\0H\x828\x03\x80b\0H\x82\x839\x81\x01`@\x81\x90Rb\0\08\x91b\0\0\xBFV[`\x01`\x01`\xA0\x1B\x03\x82\x16;b\0\0aW`@QcsS&\xAB`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16;b\0\0\x8AW`@Qc\x0C\x84\xDA\xBF`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\x80R\x16`\xA0Rb\0\0\xF7V[\x80Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\0\xBAW`\0\x80\xFD[\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15b\0\0\xD3W`\0\x80\xFD[b\0\0\xDE\x83b\0\0\xA2V[\x91Pb\0\0\xEE` \x84\x01b\0\0\xA2V[\x90P\x92P\x92\x90PV[`\x80Q`\xA0Q`\xC0QaG8b\0\x01J`\09`\0a\x1C\xAB\x01R`\0\x81\x81a\x03\xF9\x01Ra\x19J\x01R`\0\x81\x81a\x05%\x01R\x81\x81a\x07\x93\x01R\x81\x81a\x08+\x01R\x81\x81a\x1B\x90\x01Ra\x1D\x9F\x01RaG8`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x02\xD2W`\x005`\xE0\x1C\x80c{\x1Bv\x9E\x11a\x01\x86W\x80c\xB5Ap\x84\x11a\0\xE3W\x80c\xCE\xC7\xB3Y\x11a\0\x97W\x80c\xDC\x97\xD9b\x11a\0qW\x80c\xDC\x97\xD9b\x14a\x076W\x80c\xF6\xA5\xF9\xF5\x14a\x07VW\x80c\xFB\xE0\xA3\x1B\x14a\x07iW`\0\x80\xFD[\x80c\xCE\xC7\xB3Y\x14a\x06\xFDW\x80c\xD2o\xF2\x10\x14a\x07\x10W\x80c\xD3\x8B\xFF\xF4\x14a\x07#W`\0\x80\xFD[\x80c\xBD\x02\xD0\xF5\x11a\0\xC8W\x80c\xBD\x02\xD0\xF5\x14a\x06qW\x80c\xC01\xA1\x80\x14a\x06\x91W\x80c\xC8/\xE8z\x14a\x06\xA4W`\0\x80\xFD[\x80c\xB5Ap\x84\x14a\x06KW\x80c\xBC\0\xC2\x16\x14a\x06^W`\0\x80\xFD[\x80c\x97\xB8{\xA6\x11a\x01:W\x80c\x9D\xED\x06\xDF\x11a\x01\x1FW\x80c\x9D\xED\x06\xDF\x14a\x06\x1EW\x80c\xA3I\x9Cs\x14a\x061W\x80c\xAA\x1E\x1F\n\x14a\x06DW`\0\x80\xFD[\x80c\x97\xB8{\xA6\x14a\x05\xEBW\x80c\x98ny\x1A\x14a\x05\xFEW`\0\x80\xFD[\x80c\x88jb]\x11a\x01kW\x80c\x88jb]\x14a\x05\xB7W\x80c\x88\xB3\x05\x87\x14a\x05\xCAW\x80c\x93[\x13\xF6\x14a\x05\xD8W`\0\x80\xFD[\x80c{\x1Bv\x9E\x14a\x05}W\x80c\x82\x91(l\x14a\x05\x91W`\0\x80\xFD[\x80c6I@\xD8\x11a\x024W\x80c\\`\xDA\x1B\x11a\x01\xE8W\x80cd\x94\x0CV\x11a\x01\xCDW\x80cd\x94\x0CV\x14a\x05 W\x80cg\xAC\xE8\xEB\x14a\x05GW\x80cz\xE1\xCF\xCA\x14a\x05ZW`\0\x80\xFD[\x80c\\`\xDA\x1B\x14a\x04\xB4W\x80c_ip\xC3\x14a\x05\rW`\0\x80\xFD[\x80cFV\xAE.\x11a\x02\x19W\x80cFV\xAE.\x14a\x045W\x80cXZ\x9F\xD4\x14a\x04HW\x80cZ\xA6\xE6u\x14a\x04[W`\0\x80\xFD[\x80c6I@\xD8\x14a\x04\x1BW\x80cA\xD8\xF2k\x14a\x04\"W`\0\x80\xFD[\x80c\x1C\x92\x11_\x11a\x02\x8BW\x80c&\x9E\xB6^\x11a\x02pW\x80c&\x9E\xB6^\x14a\x03\xC0W\x80c&\xEFi\x9D\x14a\x03\xE1W\x80c*-\xAE\n\x14a\x03\xF4W`\0\x80\xFD[\x80c\x1C\x92\x11_\x14a\x03lW\x80c!\xF8\xA7!\x14a\x03\x7FW`\0\x80\xFD[\x80c\x14n-x\x11a\x02\xBCW\x80c\x14n-x\x14a\x02\xFFW\x80c\x14\xBF\xD6\xD0\x14a\x03\x12W\x80c\x18v\xEE\xD9\x14a\x03IW`\0\x80\xFD[\x80b\xF7\xD4\x9F\x14a\x02\xD7W\x80c\t\xC5\xEA\xBE\x14a\x02\xECW[`\0\x80\xFD[a\x02\xEAa\x02\xE56`\x04a0\x0BV[a\x07|V[\0[a\x02\xEAa\x02\xFA6`\x04a0\x0BV[a\x08\0V[a\x02\xEAa\x03\r6`\x04a0MV[a\r\x1AV[a\x033a\x03 6`\x04a0\x99V[P`@\x80Q`\0\x81R` \x81\x01\x90\x91R\x90V[`@Qa\x03@\x91\x90a0\xB2V[`@Q\x80\x91\x03\x90\xF3[a\x03\\a\x03W6`\x04a0\xFFV[a\r[V[`@Q\x90\x15\x15\x81R` \x01a\x03@V[a\x02\xEAa\x03z6`\x04a1\xB4V[a\x0E\x8FV[a\x03\xA8a\x03\x8D6`\x04a0\x99V[`\0\x90\x81R`\x02` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x90V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x03@V[a\x03\xD3a\x03\xCE6`\x04a3\x13V[a\x0E\xFAV[`@Q\x90\x81R` \x01a\x03@V[a\x02\xEAa\x03\xEF6`\x04a3PV[a\x0F\x0EV[a\x03\xA8\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\0a\x03\xD3V[a\x02\xEAa\x0406`\x04a4\x08V[a\x0F\xA6V[a\x02\xEAa\x04C6`\x04a0MV[a\x10\xC6V[a\x02\xEAa\x04V6`\x04a0MV[a\x15\rV[\x7F\xAB\xEAo\xD3\xDBV\xA6\xE6\xD0$!\x11\xB4>\xBB\x13\xD1\xC4'\te\x1C\x03,x\x94\x96 #\xA1\xF9\t`\0R`\x02` R\x7FP\xEB\xFB\xBFQM\xA2\t\xFD\x91\x83\x95/\xC6\x1A\x81\x93\xCD\xFD7\xE0Z)H\xDB$\x99\x0E&JEaT`\x01`\x01`\xA0\x1B\x03\x16a\x03\xA8V[\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC`\0R`\x02` R\x7F\x11\x14\x1FFli\xFD@\x9E\x19\x90\xE0c\xB4\x9C\xD6\xD6\x1E\xD2\xEC\xFF'\xA2\xE4\x02\xE2Y\xCAk\x9A\x01\xA3T`\x01`\x01`\xA0\x1B\x03\x16a\x03\xA8V[a\x03\\a\x05\x1B6`\x04a4%V[a\x15\xBBV[a\x03\xA8\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02\xEAa\x05U6`\x04a4\xECV[a\x16tV[a\x03\\a\x05h6`\x04a0\x99V[`\0\x90\x81R`\x04` R`@\x90 T`\xFF\x16\x90V[a\x03\\a\x05\x8B6`\x04a3\x13V[P`\0\x90V[\x7F*ep\x9D\xA5\xD4\xD3\x08\xD6\x7F\xA5Zm\xC3M\xD7\x996\xF8\xF1\x83\xE6\xFB\x9D$\x81\x14@M\xD8\x1A\xABa\x03\xD3V[a\x02\xEAa\x05\xC56`\x04a0MV[a\x18bV[a\x03\xD3a\x05\x8B6`\x04a0\x99V[a\x03\xA8a\x05\xE66`\x04a3\x13V[a\x1B\x12V[a\x02\xEAa\x05\xF96`\x04a0MV[a\x1B V[a\x06\x11a\x06\x0C6`\x04a0\x99V[a\x1B\xFEV[`@Qa\x03@\x91\x90a5\xB0V[a\x02\xEAa\x06,6`\x04a0\x0BV[a\x1C\xA0V[a\x02\xEAa\x06?6`\x04a5\xC3V[a\x1E\tV[`\0a\x03\\V[a\x02\xEAa\x06Y6`\x04a6\x13V[a!\x1FV[a\x03\\a\x06l6`\x04a6\xDFV[a!\xD5V[a\x03\xD3a\x06\x7F6`\x04a0\x99V[`\0\x90\x81R` \x81\x90R`@\x90 T\x90V[a\x06\x11a\x06\x9F6`\x04a0\x99V[a\"\xA8V[\x7Fb\x7F\x0C\x11s(7\xB3$\n-\xE8\x9C\x0BcCQ(\x86\xDDP\x97\x8B\x99\xC7jh\xC6AjM\x92`\0R`\x02` R\x7FT\x81\xD7!\x19B\x86\x87\xFE=\xCB?\xA9\xE7\xCD0\xAB8\x06\xD1H\xEE\xEBW\xED\xEC\x06\xEB\xE9\x14\x0C\x8BT`\x01`\x01`\xA0\x1B\x03\x16a\x03\xA8V[a\x03\xD3a\x07\x0B6`\x04a3\x13V[a\"\xC5V[a\x03\\a\x07\x1E6`\x04a0\x99V[a\"\xDFV[a\x02\xEAa\x0716`\x04a4\x08V[a\"\xEDV[a\x03\xD3a\x07D6`\x04a0\x99V[`\0\x90\x81R`\x05` R`@\x90 T\x90V[a\x03\\a\x07d6`\x04a7\xADV[a#\xA8V[a\x02\xEAa\x07w6`\x04a0MV[a$1V[`@Qc\xD2\x89\xD1\xCB`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xD2\x89\xD1\xCB\x90a\x07\xCA\x90\x85\x90\x85\x90`\x04\x01a8mV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x07\xE4W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x07\xF8W=`\0\x80>=`\0\xFD[PPPPPPV[`\0\x80a\x08\x0F\x83\x85\x01\x85a8\x81V[\x91P\x91P`\0a\x08%\x83\x80Q\x90` \x01 a$\xD1V[\x90P`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16cs\xE3\xD6j\x83\x85`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x08w\x92\x91\x90a8\xE5V[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x08\x91W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x08\xA5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\xC9\x91\x90a9\x13V[\x90P`\0``\x80``\x87\x80` \x01\x90Q\x81\x01\x90a\x08\xE6\x91\x90a:\xB7V[\x92\x96P\x90\x94P\x92P\x90PF\x84\x14a\t)W`@Q\x7FzG\xC9\xA2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82Q\x82Q\x81\x14\x15\x80a\t<WP\x81Q\x81\x14\x15[\x15a\tsW`@Q\x7F\xCA\x9A(\xF5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0[\x81\x81\x10\x15a\r\x0CW`\0\x85\x82\x81Q\x81\x10a\t\x92Wa\t\x92a;\xA1V[` \x02` \x01\x01Q\x90Pa\t\xA5\x81a\"\xDFV[\x15a\t\xB0WPa\x0C\xFCV[`\0\x80\x86\x84\x81Q\x81\x10a\t\xC5Wa\t\xC5a;\xA1V[` \x02` \x01\x01Q`@Q` \x01a\t\xDD\x91\x90a;\xB7V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x7FWc\x81K\x98\xA3\xAA\x86\xF2\x12yz\xF3'8h\xB5\xDDn*S-vJy\xB9\x8C\xA8Y\xE7\xBB\xAD\x81\x14\x15a\nEW\x7F\x88jb]\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91Pa\x0B\xEEV[\x7F\xECx\xD9\xC2,\x08\xBB\x9F\x0E\xCD]\x95W\x1A\xE8>?\"!\x9CZ\x92x\xC3'\x06\x91\xD5\n\xBF\xD9\x1B\x81\x14\x15a\n\x95W\x7F\x14n-x\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91Pa\x0B\xEEV[\x7F7\xAC\x16\xAA\xBCM\x87T\x0ES\x15\x1B+qbe\xCF\xD6\xB1\x95\xDB\x96\xA9\xDA\xF8\xE8\x93\xC8)\xBB\xD23\x81\x14\x15a\n\xE5W\x7F\xFB\xE0\xA3\x1B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91Pa\x0B\xEEV[\x7F\xF4\x15\x04%[\x91\x1B0B\xEEO\x87\x86\xFD\xF7\xCFK\xCF$\xAC\xE03\xFA\x16\xAF<\x85t\xE0%\xE46\x81\x14\x15a\x0B5W\x7FXZ\x9F\xD4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91Pa\x0B\xEEV[\x7F\xDA\x19\x9C\x0Ev\xF6e\xE0E\0 y\x1C\x7F\x8E\xAC\xC7_<\xDB\xAC\xE3\x13',(\xF9>S\x90\xB6,\x81\x14\x15a\x0B\x85W\x7FFV\xAE.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91Pa\x0B\xEEV[\x7F\xB4`\xDC\xB6\xFDW\x97\xFC\x0E~\xA0\xF14\x06\xC8\r0p+\xA7\xF7:B\xBD\x919Gu\xDC\xBC\xA7\x18\x81\x14\x15a\x0B\xE6W\x89a\x0B\xBAWPPPa\x0C\xFCV[`\0\x99P\x7F\x97\xB8{\xA6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91Pa\x0B\xEEV[PPPa\x0C\xFCV[a\x0B\xF9\x83`\x01a%%V[`\x000`\x01`\x01`\xA0\x1B\x03\x16\x83\x88\x87\x81Q\x81\x10a\x0C\x18Wa\x0C\x18a;\xA1V[` \x02` \x01\x01Q\x86`@Q`$\x01a\x0C2\x92\x91\x90a;\xD3V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16`\x01`\x01`\xE0\x1B\x03\x19\x90\x94\x16\x93\x90\x93\x17\x90\x92R\x90Qa\x0Cp\x91\x90a;\xB7V[`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a\x0C\xADW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x0C\xB2V[``\x91P[PP\x90P\x80\x15a\x0C\xECW`@Q\x84\x90\x7F\xA7L\x88G\xD5\x13\xFE\xBA\"\xA0\xF0\xCB8\xD50\x81\xAB\xF9ub\xCD\xB2\x93\x92k\xA2Ch\x9E|A\xCA\x90`\0\x90\xA2a\x0C\xF7V[a\x0C\xF7\x84`\0a%%V[PPPP[a\r\x05\x81a<\x0BV[\x90Pa\tvV[PPPPPPPPPPPPV[30\x14a\r:W`@Qc\x14\xE1\xDB\xF7`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80\x80a\rJ\x85\x87\x01\x87a<1V[\x92P\x92P\x92Pa\x07\xF8\x83\x83\x83a%QV[`\0\x80a\x0E\x0C\x8B\x8B\x8B\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPP`@\x80Q` `\x1F\x8F\x01\x81\x90\x04\x81\x02\x82\x01\x81\x01\x90\x92R\x8D\x81R\x92P\x8D\x91P\x8C\x90\x81\x90\x84\x01\x83\x82\x80\x82\x847`\0\x92\x01\x91\x90\x91RPP`@\x80Q` `\x1F\x8D\x01\x81\x90\x04\x81\x02\x82\x01\x81\x01\x90\x92R\x8B\x81R3\x93P\x8D\x92P\x90\x8C\x90\x8C\x90\x81\x90\x84\x01\x83\x82\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x8C\x92Pa&`\x91PPV[`\0\x81\x81R`\x04` R`@\x90 T`\xFF\x16\x92P\x90P\x81\x15a\x0E\x81W`\0\x81\x81R`\x04` R`@\x90 \x80T`\xFF\x19\x16\x90Ua\x0E\x81\x85\x85\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RP3\x92P\x87\x91Pa%Q\x90PV[P\x99\x98PPPPPPPPPV[\x81\x81`@Qa\x0E\x9F\x92\x91\x90a<\x8BV[`@Q\x80\x91\x03\x90 3`\x01`\x01`\xA0\x1B\x03\x16\x7F0\xAEl\xC7\x8C'\xE6Qt[\xF2\xAD\x08\xA1\x1D\xE89\x10\xAC\x1E4zR\xF7\xAC\x89\x8C\x0F\xBE\xF9M\xAE\x88\x88\x88\x88\x88\x88`@Qa\x0E\xEA\x96\x95\x94\x93\x92\x91\x90a<\x9BV[`@Q\x80\x91\x03\x90\xA3PPPPPPV[`\0a\x0F\x08a\x06\x7F\x83a&\xC4V[\x92\x91PPV[a\x0FP3\x84\x84\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x86\x92Pa&\xF9\x91PPV[3`\x01`\x01`\xA0\x1B\x03\x16\x7Fe\x1D\x93\xF6lC)c\x0E\x8D\x0FbH\x8E\xFFY\x9E;\xE4\x84\xDAXs5\xE8\xDC\x0F\xCFF\x06'&\x88\x88\x88\x88\x88\x88\x88`@Qa\x0F\x95\x97\x96\x95\x94\x93\x92\x91\x90a<\xE4V[`@Q\x80\x91\x03\x90\xA2PPPPPPPV[\x7Fb\x7F\x0C\x11s(7\xB3$\n-\xE8\x9C\x0BcCQ(\x86\xDDP\x97\x8B\x99\xC7jh\xC6AjM\x92`\0R`\x02` R\x7FT\x81\xD7!\x19B\x86\x87\xFE=\xCB?\xA9\xE7\xCD0\xAB8\x06\xD1H\xEE\xEBW\xED\xEC\x06\xEB\xE9\x14\x0C\x8BT`\x01`\x01`\xA0\x1B\x03\x163\x14\x80\x15\x90a\x10\\WP\x7F\xAB\xEAo\xD3\xDBV\xA6\xE6\xD0$!\x11\xB4>\xBB\x13\xD1\xC4'\te\x1C\x03,x\x94\x96 #\xA1\xF9\t`\0R`\x02` R\x7FP\xEB\xFB\xBFQM\xA2\t\xFD\x91\x83\x95/\xC6\x1A\x81\x93\xCD\xFD7\xE0Z)H\xDB$\x99\x0E&JEaT`\x01`\x01`\xA0\x1B\x03\x163\x14\x15[\x15a\x10zW`@Qc\":\xA8=`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x10\xBAW`@Q\x7F\xD7\x9Dw,\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x10\xC3\x81a)\x04V[PV[30\x14a\x10\xE6W`@Qc\x14\xE1\xDB\xF7`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80a\x10\xF5\x84\x86\x01\x86a=5V[\x91P\x91P`\0a\x11\x04\x83a\x1B\x12V[\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16a\x118W\x82`@Qc9]\t\xBF`\xE1\x1B\x81R`\x04\x01a\x11/\x91\x90a5\xB0V[`@Q\x80\x91\x03\x90\xFD[`\x02a\x11C\x84a)\xF9V[`\x02\x81\x11\x15a\x11TWa\x11Ta=zV[\x14\x15a\x14\xAAW`\0a\x12\x11\x83`@Q\x80` \x01a\x11p\x90a/\xB5V[`\x1F\x19\x82\x82\x03\x81\x01\x83R`\x1F\x90\x91\x01\x16`@\x81\x90Ra\x11\x92\x91\x90` \x01a;\xB7V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84\x83\x01Rk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x190``\x1B\x16`!\x85\x01R`5\x84\x01\x94\x90\x94R`U\x80\x84\x01\x94\x90\x94R\x81Q\x80\x84\x03\x90\x94\x01\x84R`u\x90\x92\x01\x90R\x81Q\x91\x01 \x90V[\x90Pa\x12%\x81`\x01`\x01`\xA0\x1B\x03\x16a*\x18V[\x15a\x123WPPPPPPPV[`\0\x83`@Qa\x12B\x90a/\xB5V[\x81\x90`@Q\x80\x91\x03\x90`\0\xF5\x90P\x80\x15\x80\x15a\x12bW=`\0\x80>=`\0\xFD[P`@Q\x7Fp\xA0\x821\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x80\x83\x16`\x04\x83\x01\x81\x90R\x92\x93P`\0\x92\x83\x92\x90\x91c\x1C\xFFy\xCD\x91\x88\x91c\xA9\x05\x9C\xBB`\xE0\x1B\x910\x91\x84\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x12\xDCW`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x12\xF0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\x14\x91\x90a=\x90V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`$\x83\x01R`D\x82\x01R`d\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16`\x01`\x01`\xE0\x1B\x03\x19\x94\x85\x16\x17\x90RQ`\xE0\x85\x90\x1B\x90\x92\x16\x82Ra\x13u\x92\x91`\x04\x01a=\xA9V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x13\x8FW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x13\xA3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x13\xCB\x91\x90\x81\x01\x90a=\xCBV[\x91P\x91P\x81\x15\x80a\x13\xF8WP\x80Q\x15\x80\x15\x90a\x13\xF8WP\x80\x80` \x01\x90Q\x81\x01\x90a\x13\xF6\x91\x90a9\x13V[\x15[\x15a\x141W\x86`@Q\x7F\xE2\x17\xB0\xAD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x11/\x91\x90a5\xB0V[`@Q~\xF5]\x9D\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x84\x16\x90b\xF5]\x9D\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x14\x89W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x14\x9DW=`\0\x80>=`\0\xFD[PPPPPPPPa\x07\xF8V[`@Qc\x08\xA1\xEE\xE1`\xE0\x1B\x81R`\x04\x81\x01\x83\x90R`\x01`\x01`\xA0\x1B\x03\x82\x16\x90c\x08\xA1\xEE\xE1\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x14\xECW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x15\0W=`\0\x80>=`\0\xFD[PPPPPPP[PPPV[30\x14a\x15-W`@Qc\x14\xE1\xDB\xF7`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80\x80\x80\x80\x80\x80\x80a\x15B\x8A\x8C\x01\x8Ca>\x0FV[\x97P\x97P\x97P\x97P\x97P\x97P\x97P\x97Pa\x15a\x89\x89\x89\x89\x89\x89\x89a*[V[\x84\x86`\x01`\x01`\xA0\x1B\x03\x16\x8A\x7F\x99\x91\xFA\xA1\xF45gQY\xFF\xAEd\xB6m~\xCF\xDBU\xC2\x97U\x86\x9A\x18\xDB\x84\x97\xB49#G\xE0\x8B\x8B\x89\x89\x89\x89`@Qa\x15\xA6\x96\x95\x94\x93\x92\x91\x90a>\xCDV[`@Q\x80\x91\x03\x90\xA4PPPPPPPPPPPV[`\0\x80a\x166\x88\x88\x88\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPP`@\x80Q` `\x1F\x8C\x01\x81\x90\x04\x81\x02\x82\x01\x81\x01\x90\x92R\x8A\x81R\x92P\x8A\x91P\x89\x90\x81\x90\x84\x01\x83\x82\x80\x82\x847`\0\x92\x01\x91\x90\x91RP3\x92P\x89\x91Pa*\x88\x90PV[`\0\x81\x81R`\x04` R`@\x90 T`\xFF\x16\x92P\x90P\x81\x15a\x16iW`\0\x81\x81R`\x04` R`@\x90 \x80T`\xFF\x19\x16\x90U[P\x96\x95PPPPPPV[\x7Fb\x7F\x0C\x11s(7\xB3$\n-\xE8\x9C\x0BcCQ(\x86\xDDP\x97\x8B\x99\xC7jh\xC6AjM\x92`\0R`\x02` R\x7FT\x81\xD7!\x19B\x86\x87\xFE=\xCB?\xA9\xE7\xCD0\xAB8\x06\xD1H\xEE\xEBW\xED\xEC\x06\xEB\xE9\x14\x0C\x8BT`\x01`\x01`\xA0\x1B\x03\x163\x14\x80\x15\x90a\x17*WP\x7F\xAB\xEAo\xD3\xDBV\xA6\xE6\xD0$!\x11\xB4>\xBB\x13\xD1\xC4'\te\x1C\x03,x\x94\x96 #\xA1\xF9\t`\0R`\x02` R\x7FP\xEB\xFB\xBFQM\xA2\t\xFD\x91\x83\x95/\xC6\x1A\x81\x93\xCD\xFD7\xE0Z)H\xDB$\x99\x0E&JEaT`\x01`\x01`\xA0\x1B\x03\x163\x14\x15[\x15a\x17HW`@Qc\":\xA8=`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82\x81\x81\x14a\x17\x82W`@Q\x7F\x14\xA2'_\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0[\x81\x81\x10\x15a\x07\xF8W`\0\x86\x86\x83\x81\x81\x10a\x17\xA1Wa\x17\xA1a;\xA1V[\x90P` \x02\x81\x01\x90a\x17\xB3\x91\x90a?%V[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x82\x90RP\x93\x94P\x88\x92P\x87\x91P\x85\x90P\x81\x81\x10a\x17\xFEWa\x17\xFEa;\xA1V[\x90P` \x02\x015\x90P`\0`\x01`\x01`\xA0\x1B\x03\x16a\x18\x1B\x83a\x1B\x12V[`\x01`\x01`\xA0\x1B\x03\x16\x14\x15a\x18EW\x81`@Qc9]\t\xBF`\xE1\x1B\x81R`\x04\x01a\x11/\x91\x90a5\xB0V[a\x18O\x82\x82a*\xE6V[PP\x80a\x18[\x90a<\x0BV[\x90Pa\x17\x85V[30\x14a\x18\x82W`@Qc\x14\xE1\xDB\xF7`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80\x80\x80\x80\x80a\x18\x95\x88\x8A\x01\x8Aa?lV[\x95P\x95P\x95P\x95P\x95P\x95P`\0`\x01`\x01`\xA0\x1B\x03\x16a\x18\xB5\x86a\x1B\x12V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x18\xF7W\x84`@Q\x7F\xAA~\x8B2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x11/\x91\x90a5\xB0V[a\x19\x01\x85\x82a*\xE6V[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x1AmW`\0\x85`@Q` \x01a\x19\"\x91\x90a;\xB7V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90Pa\x19E\x86`\x01a+>V[`\0\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16co\xC9[4`\xE0\x1B\x8A\x8A\x8A\x8A\x88`@Q`$\x01a\x19\x93\x95\x94\x93\x92\x91\x90a@\x0FV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16`\x01`\x01`\xE0\x1B\x03\x19\x90\x94\x16\x93\x90\x93\x17\x90\x92R\x90Qa\x19\xD1\x91\x90a;\xB7V[`\0`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80`\0\x81\x14a\x1A\x0CW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x1A\x11V[``\x91P[P\x91P\x91P\x81a\x1AOW\x87`@Q\x7F\x86\xD5'C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x11/\x91\x90a5\xB0V[\x80\x80` \x01\x90Q\x81\x01\x90a\x1Ac\x91\x90a@UV[\x94PPPPa\x1A\xC4V[`\x01`\x01`\xA0\x1B\x03\x82\x16;a\x1A\xB9W`@Q\x7F\xC5\xCC\xDD\xDE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16`\x04\x82\x01R`$\x01a\x11/V[a\x1A\xC4\x85`\x02a+>V[\x7F\xBF\x90\xB5\xA1\xEC\x97c\xE8\xBFK\x92E\xCE\xF0\xC2\x8D\xB9+\xAB0\x9F\xC2\xC5\x17\x7F\x17\x81O8$i8\x85\x83`@Qa\x1A\xF5\x92\x91\x90a@rV[`@Q\x80\x91\x03\x90\xA1a\x1B\x07\x85\x83a+nV[PPPPPPPPPV[`\0a\x0F\x08a\x03\x8D\x83a+\xB0V[30\x14a\x1B@W`@Qc\x14\xE1\xDB\xF7`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x7F\x19.u\x9EU\xF3Y\xCD\x982\xB5\xC0\xC6\xE3\x8EKm\xF5\xC5\xCA3\xF3\xBD\\\x90s\x8E\x86ZR\x18r\x83\x83`@Qa\x1Bq\x92\x91\x90a8mV[`@Q\x80\x91\x03\x90\xA1`@Qc\xD2\x89\xD1\xCB`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xD2\x89\xD1\xCB\x90a\x1B\xC7\x90\x86\x90\x86\x90`\x04\x01a8mV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1B\xE1W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x1B\xF5W=`\0\x80>=`\0\xFD[PPPPPPPV[`\0\x81\x81R`\x01` R`@\x90 \x80T``\x91\x90a\x1C\x1B\x90a@\x9DV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1CG\x90a@\x9DV[\x80\x15a\x1C\x94W\x80`\x1F\x10a\x1CiWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1C\x94V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1CwW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x91\x90PV[0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x15a\x1D\x03W`@Q\x7F\xBF\x10\xDD:\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80\x80a\x1D\x13\x84\x86\x01\x86a@\xD8V[\x91\x94P\x92P\x90P`\x01`\x01`\xA0\x1B\x03\x83\x16\x15a\x1D2Wa\x1D2\x83a+\xE5V[`\x01`\x01`\xA0\x1B\x03\x82\x16\x15a\x1DJWa\x1DJ\x82a)\x04V[\x80Q\x15a\x1E\x02W\x7F\x19.u\x9EU\xF3Y\xCD\x982\xB5\xC0\xC6\xE3\x8EKm\xF5\xC5\xCA3\xF3\xBD\\\x90s\x8E\x86ZR\x18r\x81`@Qa\x1D\x80\x91\x90a5\xB0V[`@Q\x80\x91\x03\x90\xA1`@Qc\xD2\x89\xD1\xCB`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xD2\x89\xD1\xCB\x90a\x1D\xD4\x90\x84\x90`\x04\x01a5\xB0V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1D\xEEW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x1B\x07W=`\0\x80>=`\0\xFD[PPPPPV[\x7F\xAB\xEAo\xD3\xDBV\xA6\xE6\xD0$!\x11\xB4>\xBB\x13\xD1\xC4'\te\x1C\x03,x\x94\x96 #\xA1\xF9\t`\0R`\x02` R\x7FP\xEB\xFB\xBFQM\xA2\t\xFD\x91\x83\x95/\xC6\x1A\x81\x93\xCD\xFD7\xE0Z)H\xDB$\x99\x0E&JEaT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1E|W`@Qc-[\xE4\xCB`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x83`\x01`\x01`\xA0\x1B\x03\x16?\x83\x14a\x1E\xBFW`@Q\x7F\x8F\x84\xFB$\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x83`\x01`\x01`\xA0\x1B\x03\x16c\x82\x91(l`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x1E\xF8W`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x1F\x0CW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1F0\x91\x90a=\x90V[\x7F*ep\x9D\xA5\xD4\xD3\x08\xD6\x7F\xA5Zm\xC3M\xD7\x996\xF8\xF1\x83\xE6\xFB\x9D$\x81\x14@M\xD8\x1A\xAB\x14a\x1F\x88W`@Q\x7Fh\x15_\x9A\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Q`\x01`\x01`\xA0\x1B\x03\x85\x16\x90\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;\x90`\0\x90\xA2a 3\x84\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC`\0R`\x02` R\x7F\x11\x14\x1FFli\xFD@\x9E\x19\x90\xE0c\xB4\x9C\xD6\xD6\x1E\xD2\xEC\xFF'\xA2\xE4\x02\xE2Y\xCAk\x9A\x01\xA3\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x17\x90UPV[\x80\x15a!\x19W`\0\x84`\x01`\x01`\xA0\x1B\x03\x16c\x9D\xED\x06\xDF`\xE0\x1B\x84\x84`@Q`$\x01a `\x92\x91\x90a8mV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16`\x01`\x01`\xE0\x1B\x03\x19\x90\x94\x16\x93\x90\x93\x17\x90\x92R\x90Qa \x9E\x91\x90a;\xB7V[`\0`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80`\0\x81\x14a \xD9W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a \xDEV[``\x91P[PP\x90P\x80a\x1E\x02W`@Q\x7F\x97\x90]\xFB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPV[a!a3\x84\x84\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x86\x92Pa&\xF9\x91PPV[\x84\x84`@Qa!q\x92\x91\x90a<\x8BV[`@Q\x80\x91\x03\x90 3`\x01`\x01`\xA0\x1B\x03\x16\x7F~PV\x9D&\xBEd;\xDAwWr\"\x91\xECf\xB1\xBEf\xD8(4t\xAE?\xABZ\x98\xF8x\xA7\xA2\x8B\x8B\x8B\x8B\x8B\x8B\x8B\x8B\x8B`@Qa!\xC2\x99\x98\x97\x96\x95\x94\x93\x92\x91\x90aA:V[`@Q\x80\x91\x03\x90\xA3PPPPPPPPPV[`\0a\"\x99a\x05h\x8C\x8C\x8C\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x8B\x8B\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPP`@\x80Q` `\x1F\x8D\x01\x81\x90\x04\x81\x02\x82\x01\x81\x01\x90\x92R\x8B\x81R\x8E\x93P\x8D\x92P\x90\x8C\x90\x8C\x90\x81\x90\x84\x01\x83\x82\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x8C\x92Pa&`\x91PPV[\x9B\x9APPPPPPPPPPPV[`\0\x81\x81R`\x03` R`@\x90 \x80T``\x91\x90a\x1C\x1B\x90a@\x9DV[`\0a\x0F\x08a\x06\x7F\x83a\"\xDAaT`BaA\xA2V[a,\xDAV[`\0a\x0F\x08a\x05h\x83a-/V[\x7F\xAB\xEAo\xD3\xDBV\xA6\xE6\xD0$!\x11\xB4>\xBB\x13\xD1\xC4'\te\x1C\x03,x\x94\x96 #\xA1\xF9\t`\0R`\x02` R\x7FP\xEB\xFB\xBFQM\xA2\t\xFD\x91\x83\x95/\xC6\x1A\x81\x93\xCD\xFD7\xE0Z)H\xDB$\x99\x0E&JEaT`\x01`\x01`\xA0\x1B\x03\x163\x14a#`W`@Qc-[\xE4\xCB`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16a#\x9FW`@Q~c\x18l\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x10\xC3\x81a+\xE5V[`\0a$%a\x05h\x89\x89\x89\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPP`@\x80Q` `\x1F\x8D\x01\x81\x90\x04\x81\x02\x82\x01\x81\x01\x90\x92R\x8B\x81R\x92P\x8B\x91P\x8A\x90\x81\x90\x84\x01\x83\x82\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x8A\x92P\x89\x91Pa*\x88\x90PV[\x98\x97PPPPPPPPV[30\x14a$QW`@Qc\x14\xE1\xDB\xF7`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80\x80\x80\x80\x80a$d\x88\x8A\x01\x8AaA\xC4V[\x95P\x95P\x95P\x95P\x95P\x95Pa$}\x87\x87\x87\x87\x87a-jV[\x82\x84`\x01`\x01`\xA0\x1B\x03\x16\x88\x7FD\xE4\xF8\xF6\xBDh,Z:\xEB\xA96\x01\xAB\x07\xCBM\x1F!\xB2\xAA\xB1\xAEH\x80\xD9Wy\x190\x9A\xA4\x89\x89\x87\x87`@Qa$\xBE\x94\x93\x92\x91\x90aBVV[`@Q\x80\x91\x03\x90\xA4PPPPPPPPPV[`@Q\x7F\x19Ethereum Signed Message:\n32\0\0\0\0` \x82\x01R`<\x81\x01\x82\x90R`\0\x90`\\\x01[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[a%Ma%1\x83a-/V[`\0\x90\x81R`\x04` R`@\x90 \x80T`\xFF\x19\x16\x83\x15\x15\x17\x90UV[PPV[`\0a%\\\x84a\x1B\x12V[\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16a%\x87W\x83`@Qc9]\t\xBF`\xE1\x1B\x81R`\x04\x01a\x11/\x91\x90a5\xB0V[a%\xA4\x84\x83a%\x95\x87a\"\xC5V[a%\x9F\x91\x90aB\x8FV[a-zV[`\x02a%\xAF\x85a)\xF9V[`\x02\x81\x11\x15a%\xC0Wa%\xC0a=zV[\x14\x15a%\xDFWa%\xDA`\x01`\x01`\xA0\x1B\x03\x82\x16\x84\x84a-\xF5V[a!\x19V[`@Q\x7F@\xC1\x0F\x19\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\x04\x83\x01R`$\x82\x01\x84\x90R\x82\x16\x90c@\xC1\x0F\x19\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a&BW`\0\x80\xFD[PZ\xF1\x15\x80\x15a&VW=`\0\x80>=`\0\xFD[PPPPPPPPV[`\0\x7F\xB7\xAD\x97+qGX`a=\xB3\xBA\x1F\xE6\x99\xB8\x86\xC8x\xF9\0*\t%\r\xC2^v\x9E\xB1\x9A\x10\x88\x88\x88\x88\x88\x88\x88`@Q` \x01a&\xA1\x98\x97\x96\x95\x94\x93\x92\x91\x90aB\xA7V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x97\x96PPPPPPPV[`\0\x7F\xEE\xE94\x8BJ\xAB\xA3d{\x16\x12\xB2rO\x18\xE9;\x92\x99\xDA&\xFB2\x1C{?\xDA\x13]}\xEA\x87\x82`@Q` \x01a%\x08\x92\x91\x90aC\x1AV[`\0a'\x04\x83a\x1B\x12V[\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16a'/W\x82`@Qc9]\t\xBF`\xE1\x1B\x81R`\x04\x01a\x11/\x91\x90a5\xB0V[\x81a'fW`@Q\x7F,R\x11\xC6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a'q\x84a)\xF9V[\x90P`\x02\x81`\x02\x81\x11\x15a'\x87Wa'\x87a=zV[\x14\x15a'\xA7Wa'\xA2`\x01`\x01`\xA0\x1B\x03\x83\x16\x860\x86a.XV[a\x1E\x02V[`\x01\x81`\x02\x81\x11\x15a'\xBBWa'\xBBa=zV[\x14\x15a(.W`@\x80Q`\x01`\x01`\xA0\x1B\x03\x87\x81\x16`$\x83\x01R`D\x80\x83\x01\x87\x90R\x83Q\x80\x84\x03\x90\x91\x01\x81R`d\x90\x92\x01\x90\x92R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16\x7Fy\xCCg\x90\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90Ra'\xA2\x91\x84\x16\x90a.\xA9V[`@Q\x7F1\xEE\xCA\xF4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\0`\x04\x82\x01Ra(\xD7\x90\x86\x90`\x01`\x01`\xA0\x1B\x03\x85\x16\x90c1\xEE\xCA\xF4\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a(\x8DW`\0\x80\xFD[PZ\xFA\x15\x80\x15a(\xA1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a(\xC5\x91\x90a@UV[`\x01`\x01`\xA0\x1B\x03\x85\x16\x91\x90\x86a.XV[`@Qc\x08\xA1\xEE\xE1`\xE0\x1B\x81R`\0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\x08\xA1\xEE\xE1\x90`$\x01a\x1D\xD4V[\x7Fb\x7F\x0C\x11s(7\xB3$\n-\xE8\x9C\x0BcCQ(\x86\xDDP\x97\x8B\x99\xC7jh\xC6AjM\x92`\0\x90\x81R`\x02` R\x7FT\x81\xD7!\x19B\x86\x87\xFE=\xCB?\xA9\xE7\xCD0\xAB8\x06\xD1H\xEE\xEBW\xED\xEC\x06\xEB\xE9\x14\x0C\x8BT`@Q`\x01`\x01`\xA0\x1B\x03\x84\x81\x16\x93\x92\x16\x91\x7F\xA90<\x86\x0C]\xE3\xC0\xC8f\xC3T\xD2\x81x\\\x89w\x8A\xC5\xCA-\xFF\xDF\x12\x84\x1CE\xCDN\x1En\x91\xA3\x7Fb\x7F\x0C\x11s(7\xB3$\n-\xE8\x9C\x0BcCQ(\x86\xDDP\x97\x8B\x99\xC7jh\xC6AjM\x92`\0R`\x02` R\x7FT\x81\xD7!\x19B\x86\x87\xFE=\xCB?\xA9\xE7\xCD0\xAB8\x06\xD1H\xEE\xEBW\xED\xEC\x06\xEB\xE9\x14\x0C\x8B\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x17\x90UPV[`\0a*\x07a\x06\x7F\x83a/\x80V[`\x02\x81\x11\x15a\x0F\x08Wa\x0F\x08a=zV[`\0`\x01`\x01`\xA0\x1B\x03\x82\x16?\x80\x15\x80\x15\x90a*TWP\x7F\xC5\xD2F\x01\x86\xF7#<\x92~}\xB2\xDC\xC7\x03\xC0\xE5\0\xB6S\xCA\x82';{\xFA\xD8\x04]\x85\xA4p\x81\x14\x15[\x93\x92PPPV[a\x1B\xF5a*m\x88\x88\x88\x88\x88\x88\x88a&`V[`\0\x90\x81R`\x04` R`@\x90 \x80T`\xFF\x19\x16`\x01\x17\x90UV[`\0\x7F\x07\xB0\xD40O\x82\x01+\xD3\xB7\x0B\x1DS\x1C\x16\x0E2`g\xC9\x08)\xE2\xA3\xD3\x86r*\xD1\x0B\x89\xC3\x86\x86\x86\x86\x86`@Q` \x01a*\xC5\x96\x95\x94\x93\x92\x91\x90aC@V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x95\x94PPPPPV[\x7F\xD9\x94F\xC1\xD7c\x85\xBBU\x19\xCC\xFBRt\xAB\xCF\xD5\x89m\xFC\"@^@\x01\x0F\xDE!\x7F\x01\x8A\x18\x82\x82`@Qa+\x17\x92\x91\x90a;\xD3V[`@Q\x80\x91\x03\x90\xA1a%Ma++\x83a&\xC4V[\x82`\0\x91\x82R` \x82\x90R`@\x90\x91 UV[a%Ma+J\x83a/\x80V[\x82`\x02\x81\x11\x15a+\\Wa+\\a=zV[`\0\x91\x82R` \x82\x90R`@\x90\x91 UV[a%Ma+z\x83a+\xB0V[`\0\x90\x81R`\x02` R`@\x90 \x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`\xA0\x1B\x03\x84\x16\x17\x90UV[`\0\x7F\xC4\xE62w\x9Ajx8sm\xD7\xE5\xE6\xA0\xEA\xDF\x17\x1D\xD3}\xFBb0r\x0E&Uv\xDF\xCFB\xBB\x82`@Q` \x01a%\x08\x92\x91\x90aC\x1AV[\x7F\xAB\xEAo\xD3\xDBV\xA6\xE6\xD0$!\x11\xB4>\xBB\x13\xD1\xC4'\te\x1C\x03,x\x94\x96 #\xA1\xF9\t`\0\x90\x81R`\x02` R\x7FP\xEB\xFB\xBFQM\xA2\t\xFD\x91\x83\x95/\xC6\x1A\x81\x93\xCD\xFD7\xE0Z)H\xDB$\x99\x0E&JEaT`@Q`\x01`\x01`\xA0\x1B\x03\x84\x81\x16\x93\x92\x16\x91\x7F_V\xBE\xE8\xCF\xFB\xE9\xA7\x86R\xA7J`p^\xDE\xDE\x02\xAF\x10\xB0\xBB\xB8\x88\xCAD\xB7\x9A\rB\xCE\x80\x91\xA3\x7F\xAB\xEAo\xD3\xDBV\xA6\xE6\xD0$!\x11\xB4>\xBB\x13\xD1\xC4'\te\x1C\x03,x\x94\x96 #\xA1\xF9\t`\0R`\x02` R\x7FP\xEB\xFB\xBFQM\xA2\t\xFD\x91\x83\x95/\xC6\x1A\x81\x93\xCD\xFD7\xE0Z)H\xDB$\x99\x0E&JEa\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x17\x90UPV[`\0\x7F/\x88Q\xFE\rmS~U*O%\xB7\xA3\x16}H\xEB\x12\x92b,q\xD8F0\xA2\xA4GW\xBC\xED\x83\x83`@Q` \x01a-\x11\x93\x92\x91\x90aC\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x92\x91PPV[`@\x80Q\x7F\x95w\x05\xA3t2k0\xF4\xA1\x06\x9C\x93msl\xC9\x99>\xD6\xC8 \xB4\xE0\xE2\xFD\x94\xA8\xBE\xCA\r\x1D` \x82\x01R\x90\x81\x01\x82\x90R`\0\x90``\x01a%\x08V[a\x1E\x02a*m\x86\x86\x86\x86\x86a*\x88V[`\0a-\x85\x83a\x0E\xFAV[\x90P`\0\x81\x11\x80\x15a-\x96WP\x80\x82\x11[\x15a-\xCFW\x82`@Q\x7F\x03\x7F`\xE5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x11/\x91\x90a5\xB0V[a\x15\x08a-\xE2\x84a\"\xDAaT`BaA\xA2V[\x83`\0\x91\x82R` \x82\x90R`@\x90\x91 UV[`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16`$\x82\x01R`D\x81\x01\x82\x90Ra\x15\x08\x90\x84\x90c\xA9\x05\x9C\xBB`\xE0\x1B\x90`d\x01[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16`\x01`\x01`\xE0\x1B\x03\x19\x90\x93\x16\x92\x90\x92\x17\x90\x91Ra.\xA9V[`@Q`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`$\x83\x01R\x83\x16`D\x82\x01R`d\x81\x01\x82\x90Ra!\x19\x90\x85\x90\x7F#\xB8r\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90`\x84\x01a.!V[`\0\x80\x83`\x01`\x01`\xA0\x1B\x03\x16\x83`@Qa.\xC4\x91\x90a;\xB7V[`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a/\x01W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a/\x06V[``\x91P[P\x91P\x91P`\0\x82\x80\x15a/2WP\x81Q\x15\x80a/2WP\x81\x80` \x01\x90Q\x81\x01\x90a/2\x91\x90a9\x13V[\x90P\x80\x15\x80a/IWP`\x01`\x01`\xA0\x1B\x03\x85\x16;\x15[\x15a\x1E\x02W`@Q\x7F\x04\\K\x02\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x7F\xA8\r\"Y\xAFU\x89\x06\x18\xEC.\xEB:\xC7-\xE4\xBD\xBA\"R\x9B\xB1HE\xD8\xA3\xD7\x12\xD1\xC3\xF6!\x82`@Q` \x01a%\x08\x92\x91\x90aC\x1AV[a\x03I\x80aC\xBA\x839\x01\x90V[`\0\x80\x83`\x1F\x84\x01\x12a/\xD4W`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a/\xECW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a0\x04W`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80` \x83\x85\x03\x12\x15a0\x1EW`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a05W`\0\x80\xFD[a0A\x85\x82\x86\x01a/\xC2V[\x90\x96\x90\x95P\x93PPPPV[`\0\x80`\0`@\x84\x86\x03\x12\x15a0bW`\0\x80\xFD[\x835g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a0yW`\0\x80\xFD[a0\x85\x86\x82\x87\x01a/\xC2V[\x90\x97\x90\x96P` \x95\x90\x95\x015\x94\x93PPPPV[`\0` \x82\x84\x03\x12\x15a0\xABW`\0\x80\xFD[P5\x91\x90PV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a0\xF3W\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a0\xCEV[P\x90\x96\x95PPPPPPV[`\0\x80`\0\x80`\0\x80`\0\x80`\0`\xC0\x8A\x8C\x03\x12\x15a1\x1DW`\0\x80\xFD[\x895\x98P` \x8A\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a1<W`\0\x80\xFD[a1H\x8D\x83\x8E\x01a/\xC2V[\x90\x9AP\x98P`@\x8C\x015\x91P\x80\x82\x11\x15a1aW`\0\x80\xFD[a1m\x8D\x83\x8E\x01a/\xC2V[\x90\x98P\x96P``\x8C\x015\x95P`\x80\x8C\x015\x91P\x80\x82\x11\x15a1\x8DW`\0\x80\xFD[Pa1\x9A\x8C\x82\x8D\x01a/\xC2V[\x9A\x9D\x99\x9CP\x97\x9A\x96\x99\x95\x98\x94\x97\x96`\xA0\x015\x94\x93PPPPV[`\0\x80`\0\x80`\0\x80``\x87\x89\x03\x12\x15a1\xCDW`\0\x80\xFD[\x865g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a1\xE5W`\0\x80\xFD[a1\xF1\x8A\x83\x8B\x01a/\xC2V[\x90\x98P\x96P` \x89\x015\x91P\x80\x82\x11\x15a2\nW`\0\x80\xFD[a2\x16\x8A\x83\x8B\x01a/\xC2V[\x90\x96P\x94P`@\x89\x015\x91P\x80\x82\x11\x15a2/W`\0\x80\xFD[Pa2<\x89\x82\x8A\x01a/\xC2V[\x97\x9A\x96\x99P\x94\x97P\x92\x95\x93\x94\x92PPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a2\x8DWa2\x8Da2NV[`@R\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a2\xAFWa2\xAFa2NV[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a2\xCEW`\0\x80\xFD[\x815a2\xE1a2\xDC\x82a2\x95V[a2dV[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a2\xF6W`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a3%W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a3<W`\0\x80\xFD[a3H\x84\x82\x85\x01a2\xBDV[\x94\x93PPPPV[`\0\x80`\0\x80`\0\x80`\0`\x80\x88\x8A\x03\x12\x15a3kW`\0\x80\xFD[\x875g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a3\x83W`\0\x80\xFD[a3\x8F\x8B\x83\x8C\x01a/\xC2V[\x90\x99P\x97P` \x8A\x015\x91P\x80\x82\x11\x15a3\xA8W`\0\x80\xFD[a3\xB4\x8B\x83\x8C\x01a/\xC2V[\x90\x97P\x95P`@\x8A\x015\x91P\x80\x82\x11\x15a3\xCDW`\0\x80\xFD[Pa3\xDA\x8A\x82\x8B\x01a/\xC2V[\x98\x9B\x97\x9AP\x95\x98\x94\x97\x95\x96``\x90\x95\x015\x94\x93PPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x10\xC3W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a4\x1AW`\0\x80\xFD[\x815a*T\x81a3\xF3V[`\0\x80`\0\x80`\0\x80`\x80\x87\x89\x03\x12\x15a4>W`\0\x80\xFD[\x865\x95P` \x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a4]W`\0\x80\xFD[a4i\x8A\x83\x8B\x01a/\xC2V[\x90\x97P\x95P`@\x89\x015\x91P\x80\x82\x11\x15a4\x82W`\0\x80\xFD[Pa4\x8F\x89\x82\x8A\x01a/\xC2V[\x97\x9A\x96\x99P\x94\x97\x94\x96\x95``\x90\x95\x015\x94\x93PPPPV[`\0\x80\x83`\x1F\x84\x01\x12a4\xB9W`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a4\xD1W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a0\x04W`\0\x80\xFD[`\0\x80`\0\x80`@\x85\x87\x03\x12\x15a5\x02W`\0\x80\xFD[\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a5\x1AW`\0\x80\xFD[a5&\x88\x83\x89\x01a4\xA7V[\x90\x96P\x94P` \x87\x015\x91P\x80\x82\x11\x15a5?W`\0\x80\xFD[Pa5L\x87\x82\x88\x01a4\xA7V[\x95\x98\x94\x97P\x95PPPPV[`\0[\x83\x81\x10\x15a5sW\x81\x81\x01Q\x83\x82\x01R` \x01a5[V[\x83\x81\x11\x15a!\x19WPP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra5\x9C\x81` \x86\x01` \x86\x01a5XV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a*T` \x83\x01\x84a5\x84V[`\0\x80`\0\x80``\x85\x87\x03\x12\x15a5\xD9W`\0\x80\xFD[\x845a5\xE4\x81a3\xF3V[\x93P` \x85\x015\x92P`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a6\x07W`\0\x80\xFD[a5L\x87\x82\x88\x01a/\xC2V[`\0\x80`\0\x80`\0\x80`\0\x80`\0`\xA0\x8A\x8C\x03\x12\x15a61W`\0\x80\xFD[\x895g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a6IW`\0\x80\xFD[a6U\x8D\x83\x8E\x01a/\xC2V[\x90\x9BP\x99P` \x8C\x015\x91P\x80\x82\x11\x15a6nW`\0\x80\xFD[a6z\x8D\x83\x8E\x01a/\xC2V[\x90\x99P\x97P`@\x8C\x015\x91P\x80\x82\x11\x15a6\x93W`\0\x80\xFD[a6\x9F\x8D\x83\x8E\x01a/\xC2V[\x90\x97P\x95P``\x8C\x015\x91P\x80\x82\x11\x15a6\xB8W`\0\x80\xFD[Pa6\xC5\x8C\x82\x8D\x01a/\xC2V[\x9A\x9D\x99\x9CP\x97\x9A\x96\x99\x95\x98\x94\x97\x96`\x80\x015\x94\x93PPPPV[`\0\x80`\0\x80`\0\x80`\0\x80`\0\x80`\xE0\x8B\x8D\x03\x12\x15a6\xFEW`\0\x80\xFD[\x8A5\x99P` \x8B\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a7\x1DW`\0\x80\xFD[a7)\x8E\x83\x8F\x01a/\xC2V[\x90\x9BP\x99P`@\x8D\x015\x91P\x80\x82\x11\x15a7BW`\0\x80\xFD[a7N\x8E\x83\x8F\x01a/\xC2V[\x90\x99P\x97P``\x8D\x015\x91Pa7c\x82a3\xF3V[\x90\x95P`\x80\x8C\x015\x94P`\xA0\x8C\x015\x90\x80\x82\x11\x15a7\x80W`\0\x80\xFD[Pa7\x8D\x8D\x82\x8E\x01a/\xC2V[\x91P\x80\x94PP\x80\x92PP`\xC0\x8B\x015\x90P\x92\x95\x98\x9B\x91\x94\x97\x9AP\x92\x95\x98PV[`\0\x80`\0\x80`\0\x80`\0`\xA0\x88\x8A\x03\x12\x15a7\xC8W`\0\x80\xFD[\x875\x96P` \x88\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a7\xE7W`\0\x80\xFD[a7\xF3\x8B\x83\x8C\x01a/\xC2V[\x90\x98P\x96P`@\x8A\x015\x91P\x80\x82\x11\x15a8\x0CW`\0\x80\xFD[Pa8\x19\x8A\x82\x8B\x01a/\xC2V[\x90\x95P\x93PP``\x88\x015a8-\x81a3\xF3V[\x80\x92PP`\x80\x88\x015\x90P\x92\x95\x98\x91\x94\x97P\x92\x95PV[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[` \x81R`\0a3H` \x83\x01\x84\x86a8DV[`\0\x80`@\x83\x85\x03\x12\x15a8\x94W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a8\xACW`\0\x80\xFD[a8\xB8\x86\x83\x87\x01a2\xBDV[\x93P` \x85\x015\x91P\x80\x82\x11\x15a8\xCEW`\0\x80\xFD[Pa8\xDB\x85\x82\x86\x01a2\xBDV[\x91PP\x92P\x92\x90PV[\x82\x81R`@` \x82\x01R`\0a3H`@\x83\x01\x84a5\x84V[\x80Q\x80\x15\x15\x81\x14a9\x0EW`\0\x80\xFD[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a9%W`\0\x80\xFD[a*T\x82a8\xFEV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a9HWa9Ha2NV[P`\x05\x1B` \x01\x90V[`\0a9`a2\xDC\x84a2\x95V[\x90P\x82\x81R\x83\x83\x83\x01\x11\x15a9tW`\0\x80\xFD[a*T\x83` \x83\x01\x84a5XV[`\0\x82`\x1F\x83\x01\x12a9\x93W`\0\x80\xFD[\x81Q` a9\xA3a2\xDC\x83a9.V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a9\xC2W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a\x16iW\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a9\xE6W`\0\x80\x81\xFD[\x87\x01`?\x81\x01\x89\x13a9\xF8W`\0\x80\x81\xFD[a:\t\x89\x86\x83\x01Q`@\x84\x01a9RV[\x84RP\x91\x83\x01\x91\x83\x01a9\xC6V[`\0\x82`\x1F\x83\x01\x12a:(W`\0\x80\xFD[a*T\x83\x83Q` \x85\x01a9RV[`\0\x82`\x1F\x83\x01\x12a:HW`\0\x80\xFD[\x81Q` a:Xa2\xDC\x83a9.V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a:wW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a\x16iW\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a:\x9BW`\0\x80\x81\xFD[a:\xA9\x89\x86\x83\x8B\x01\x01a:\x17V[\x84RP\x91\x83\x01\x91\x83\x01a:{V[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a:\xCDW`\0\x80\xFD[\x84Q\x93P` \x80\x86\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a:\xEDW`\0\x80\xFD[\x81\x88\x01\x91P\x88`\x1F\x83\x01\x12a;\x01W`\0\x80\xFD[\x81Qa;\x0Fa2\xDC\x82a9.V[\x81\x81R`\x05\x91\x90\x91\x1B\x83\x01\x84\x01\x90\x84\x81\x01\x90\x8B\x83\x11\x15a;.W`\0\x80\xFD[\x93\x85\x01\x93[\x82\x85\x10\x15a;LW\x84Q\x82R\x93\x85\x01\x93\x90\x85\x01\x90a;3V[`@\x8B\x01Q\x90\x98P\x94PPP\x80\x83\x11\x15a;eW`\0\x80\xFD[a;q\x89\x84\x8A\x01a9\x82V[\x94P``\x88\x01Q\x92P\x80\x83\x11\x15a;\x87W`\0\x80\xFD[PPa;\x95\x87\x82\x88\x01a:7V[\x91PP\x92\x95\x91\x94P\x92PV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0\x82Qa;\xC9\x81\x84` \x87\x01a5XV[\x91\x90\x91\x01\x92\x91PPV[`@\x81R`\0a;\xE6`@\x83\x01\x85a5\x84V[\x90P\x82` \x83\x01R\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\0\x19\x82\x14\x15a<\x1FWa<\x1Fa;\xF5V[P`\x01\x01\x90V[\x805a9\x0E\x81a3\xF3V[`\0\x80`\0``\x84\x86\x03\x12\x15a<FW`\0\x80\xFD[\x835g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a<]W`\0\x80\xFD[a<i\x86\x82\x87\x01a2\xBDV[\x93PP` \x84\x015a<z\x81a3\xF3V[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[\x81\x83\x827`\0\x91\x01\x90\x81R\x91\x90PV[``\x81R`\0a<\xAF``\x83\x01\x88\x8Aa8DV[\x82\x81\x03` \x84\x01Ra<\xC2\x81\x87\x89a8DV[\x90P\x82\x81\x03`@\x84\x01Ra<\xD7\x81\x85\x87a8DV[\x99\x98PPPPPPPPPV[`\x80\x81R`\0a<\xF8`\x80\x83\x01\x89\x8Ba8DV[\x82\x81\x03` \x84\x01Ra=\x0B\x81\x88\x8Aa8DV[\x90P\x82\x81\x03`@\x84\x01Ra= \x81\x86\x88a8DV[\x91PP\x82``\x83\x01R\x98\x97PPPPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a=HW`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a=_W`\0\x80\xFD[a=k\x85\x82\x86\x01a2\xBDV[\x95` \x94\x90\x94\x015\x94PPPPV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a=\xA2W`\0\x80\xFD[PQ\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01R`\0a3H`@\x83\x01\x84a5\x84V[`\0\x80`@\x83\x85\x03\x12\x15a=\xDEW`\0\x80\xFD[a=\xE7\x83a8\xFEV[\x91P` \x83\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a>\x03W`\0\x80\xFD[a8\xDB\x85\x82\x86\x01a:\x17V[`\0\x80`\0\x80`\0\x80`\0\x80a\x01\0\x89\x8B\x03\x12\x15a>,W`\0\x80\xFD[\x885g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a>DW`\0\x80\xFD[a>P\x8C\x83\x8D\x01a2\xBDV[\x99P` \x8B\x015\x91P\x80\x82\x11\x15a>fW`\0\x80\xFD[a>r\x8C\x83\x8D\x01a2\xBDV[\x98Pa>\x80`@\x8C\x01a<&V[\x97P``\x8B\x015\x96P`\x80\x8B\x015\x91P\x80\x82\x11\x15a>\x9DW`\0\x80\xFD[Pa>\xAA\x8B\x82\x8C\x01a2\xBDV[\x98\x9B\x97\x9AP\x95\x98\x94\x97\x96`\xA0\x86\x015\x96P`\xC0\x86\x015\x95`\xE0\x015\x94P\x92PPPV[`\xC0\x81R`\0a>\xE0`\xC0\x83\x01\x89a5\x84V[\x82\x81\x03` \x84\x01Ra>\xF2\x81\x89a5\x84V[\x90P\x82\x81\x03`@\x84\x01Ra?\x06\x81\x88a5\x84V[``\x84\x01\x96\x90\x96RPP`\x80\x81\x01\x92\x90\x92R`\xA0\x90\x91\x01R\x93\x92PPPV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a?<W`\0\x80\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a?WW`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a0\x04W`\0\x80\xFD[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a?\x85W`\0\x80\xFD[\x865g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a?\x9DW`\0\x80\xFD[a?\xA9\x8A\x83\x8B\x01a2\xBDV[\x97P` \x89\x015\x91P\x80\x82\x11\x15a?\xBFW`\0\x80\xFD[Pa?\xCC\x89\x82\x8A\x01a2\xBDV[\x95PP`@\x87\x015`\xFF\x81\x16\x81\x14a?\xE3W`\0\x80\xFD[\x93P``\x87\x015\x92P`\x80\x87\x015a?\xFA\x81a3\xF3V[\x80\x92PP`\xA0\x87\x015\x90P\x92\x95P\x92\x95P\x92\x95V[`\xA0\x81R`\0a@\"`\xA0\x83\x01\x88a5\x84V[\x82\x81\x03` \x84\x01Ra@4\x81\x88a5\x84V[`\xFF\x96\x90\x96\x16`@\x84\x01RPP``\x81\x01\x92\x90\x92R`\x80\x90\x91\x01R\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a@gW`\0\x80\xFD[\x81Qa*T\x81a3\xF3V[`@\x81R`\0a@\x85`@\x83\x01\x85a5\x84V[\x90P`\x01`\x01`\xA0\x1B\x03\x83\x16` \x83\x01R\x93\x92PPPV[`\x01\x81\x81\x1C\x90\x82\x16\x80a@\xB1W`\x7F\x82\x16\x91P[` \x82\x10\x81\x14\x15a@\xD2WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a@\xEDW`\0\x80\xFD[\x835a@\xF8\x81a3\xF3V[\x92P` \x84\x015aA\x08\x81a3\xF3V[\x91P`@\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aA$W`\0\x80\xFD[aA0\x86\x82\x87\x01a2\xBDV[\x91PP\x92P\x92P\x92V[`\xA0\x81R`\0aAN`\xA0\x83\x01\x8B\x8Da8DV[\x82\x81\x03` \x84\x01RaAa\x81\x8A\x8Ca8DV[\x90P\x82\x81\x03`@\x84\x01RaAv\x81\x88\x8Aa8DV[\x90P\x82\x81\x03``\x84\x01RaA\x8B\x81\x86\x88a8DV[\x91PP\x82`\x80\x83\x01R\x9A\x99PPPPPPPPPPV[`\0\x82aA\xBFWcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15aA\xDDW`\0\x80\xFD[\x865g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15aA\xF5W`\0\x80\xFD[aB\x01\x8A\x83\x8B\x01a2\xBDV[\x97P` \x89\x015\x91P\x80\x82\x11\x15aB\x17W`\0\x80\xFD[PaB$\x89\x82\x8A\x01a2\xBDV[\x95PP`@\x87\x015aB5\x81a3\xF3V[\x95\x98\x94\x97P\x94\x95``\x81\x015\x95P`\x80\x81\x015\x94`\xA0\x90\x91\x015\x93P\x91PPV[`\x80\x81R`\0aBi`\x80\x83\x01\x87a5\x84V[\x82\x81\x03` \x84\x01RaB{\x81\x87a5\x84V[`@\x84\x01\x95\x90\x95RPP``\x01R\x92\x91PPV[`\0\x82\x19\x82\x11\x15aB\xA2WaB\xA2a;\xF5V[P\x01\x90V[`\0a\x01\0\x8A\x83R\x89` \x84\x01R\x80`@\x84\x01RaB\xC7\x81\x84\x01\x8Aa5\x84V[\x90P\x82\x81\x03``\x84\x01RaB\xDB\x81\x89a5\x84V[\x90P`\x01`\x01`\xA0\x1B\x03\x87\x16`\x80\x84\x01R\x85`\xA0\x84\x01R\x82\x81\x03`\xC0\x84\x01RaC\x04\x81\x86a5\x84V[\x91PP\x82`\xE0\x83\x01R\x99\x98PPPPPPPPPV[\x82\x81R`\0\x82QaC2\x81` \x85\x01` \x87\x01a5XV[\x91\x90\x91\x01` \x01\x93\x92PPPV[\x86\x81R\x85` \x82\x01R`\xC0`@\x82\x01R`\0aC_`\xC0\x83\x01\x87a5\x84V[\x82\x81\x03``\x84\x01RaCq\x81\x87a5\x84V[`\x01`\x01`\xA0\x1B\x03\x95\x90\x95\x16`\x80\x84\x01RPP`\xA0\x01R\x94\x93PPPPV[\x83\x81R``` \x82\x01R`\0aC\xA9``\x83\x01\x85a5\x84V[\x90P\x82`@\x83\x01R\x94\x93PPPPV\xFE`\x80`@R`\x01`\0U4\x80\x15a\0\x15W`\0\x80\xFD[Pa\x03$\x80a\0%`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x005W`\x005`\xE0\x1C\x80b\xF5]\x9D\x14a\0:W\x80c\x1C\xFFy\xCD\x14a\0OW[`\0\x80\xFD[a\0Ma\0H6`\x04a\x01\xDAV[a\0yV[\0[a\0ba\0]6`\x04a\x01\xFCV[a\0\xBBV[`@Qa\0p\x92\x91\x90a\x02\x7FV[`@Q\x80\x91\x03\x90\xF3[`\x02`\0T\x14\x15a\0\x9DW`@Qc\xCA\xA3\x0FU`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02`\0Us\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\xFF[`\0```\x02`\0T\x14\x15a\0\xE3W`@Qc\xCA\xA3\x0FU`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02`\0Us\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16;a\x016W`@Q\x7Fo|C\xF1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84\x84`@Qa\x01]\x92\x91\x90a\x02\xDEV[`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a\x01\x9AW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x01\x9FV[``\x91P[P`\x01`\0U\x90\x96\x90\x95P\x93PPPPV[\x805s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x01\xD5W`\0\x80\xFD[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x01\xECW`\0\x80\xFD[a\x01\xF5\x82a\x01\xB1V[\x93\x92PPPV[`\0\x80`\0`@\x84\x86\x03\x12\x15a\x02\x11W`\0\x80\xFD[a\x02\x1A\x84a\x01\xB1V[\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x027W`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a\x02KW`\0\x80\xFD[\x815\x81\x81\x11\x15a\x02ZW`\0\x80\xFD[\x87` \x82\x85\x01\x01\x11\x15a\x02lW`\0\x80\xFD[` \x83\x01\x94P\x80\x93PPPP\x92P\x92P\x92V[\x82\x15\x15\x81R`\0` `@\x81\x84\x01R\x83Q\x80`@\x85\x01R`\0[\x81\x81\x10\x15a\x02\xB5W\x85\x81\x01\x83\x01Q\x85\x82\x01``\x01R\x82\x01a\x02\x99V[\x81\x81\x11\x15a\x02\xC7W`\0``\x83\x87\x01\x01R[P`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01``\x01\x94\x93PPPPV[\x81\x83\x827`\0\x91\x01\x90\x81R\x91\x90PV\xFE\xA2dipfsX\"\x12 2\xCB^th\x16\xB7\xFA\xC9R\x05\xC0h\xB3\r\xA3{\xD4\x01\x19\xA5re\xBE3\x1C\x16,\xAEtq$dsolcC\0\x08\t\x003\xA2dipfsX\"\x12 \x82`\x16\xCA\x85\x85\xB5V\x07\x9C\x9B]I\xCA\xA9\xC1,w.\xAD:\x9D\xE6CD\xC0\xCA\xD8\xAB\x01*wdsolcC\0\x08\t\x003";
    /// The bytecode of the contract.
    pub static SCALARGATEWAY_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x02\xD2W`\x005`\xE0\x1C\x80c{\x1Bv\x9E\x11a\x01\x86W\x80c\xB5Ap\x84\x11a\0\xE3W\x80c\xCE\xC7\xB3Y\x11a\0\x97W\x80c\xDC\x97\xD9b\x11a\0qW\x80c\xDC\x97\xD9b\x14a\x076W\x80c\xF6\xA5\xF9\xF5\x14a\x07VW\x80c\xFB\xE0\xA3\x1B\x14a\x07iW`\0\x80\xFD[\x80c\xCE\xC7\xB3Y\x14a\x06\xFDW\x80c\xD2o\xF2\x10\x14a\x07\x10W\x80c\xD3\x8B\xFF\xF4\x14a\x07#W`\0\x80\xFD[\x80c\xBD\x02\xD0\xF5\x11a\0\xC8W\x80c\xBD\x02\xD0\xF5\x14a\x06qW\x80c\xC01\xA1\x80\x14a\x06\x91W\x80c\xC8/\xE8z\x14a\x06\xA4W`\0\x80\xFD[\x80c\xB5Ap\x84\x14a\x06KW\x80c\xBC\0\xC2\x16\x14a\x06^W`\0\x80\xFD[\x80c\x97\xB8{\xA6\x11a\x01:W\x80c\x9D\xED\x06\xDF\x11a\x01\x1FW\x80c\x9D\xED\x06\xDF\x14a\x06\x1EW\x80c\xA3I\x9Cs\x14a\x061W\x80c\xAA\x1E\x1F\n\x14a\x06DW`\0\x80\xFD[\x80c\x97\xB8{\xA6\x14a\x05\xEBW\x80c\x98ny\x1A\x14a\x05\xFEW`\0\x80\xFD[\x80c\x88jb]\x11a\x01kW\x80c\x88jb]\x14a\x05\xB7W\x80c\x88\xB3\x05\x87\x14a\x05\xCAW\x80c\x93[\x13\xF6\x14a\x05\xD8W`\0\x80\xFD[\x80c{\x1Bv\x9E\x14a\x05}W\x80c\x82\x91(l\x14a\x05\x91W`\0\x80\xFD[\x80c6I@\xD8\x11a\x024W\x80c\\`\xDA\x1B\x11a\x01\xE8W\x80cd\x94\x0CV\x11a\x01\xCDW\x80cd\x94\x0CV\x14a\x05 W\x80cg\xAC\xE8\xEB\x14a\x05GW\x80cz\xE1\xCF\xCA\x14a\x05ZW`\0\x80\xFD[\x80c\\`\xDA\x1B\x14a\x04\xB4W\x80c_ip\xC3\x14a\x05\rW`\0\x80\xFD[\x80cFV\xAE.\x11a\x02\x19W\x80cFV\xAE.\x14a\x045W\x80cXZ\x9F\xD4\x14a\x04HW\x80cZ\xA6\xE6u\x14a\x04[W`\0\x80\xFD[\x80c6I@\xD8\x14a\x04\x1BW\x80cA\xD8\xF2k\x14a\x04\"W`\0\x80\xFD[\x80c\x1C\x92\x11_\x11a\x02\x8BW\x80c&\x9E\xB6^\x11a\x02pW\x80c&\x9E\xB6^\x14a\x03\xC0W\x80c&\xEFi\x9D\x14a\x03\xE1W\x80c*-\xAE\n\x14a\x03\xF4W`\0\x80\xFD[\x80c\x1C\x92\x11_\x14a\x03lW\x80c!\xF8\xA7!\x14a\x03\x7FW`\0\x80\xFD[\x80c\x14n-x\x11a\x02\xBCW\x80c\x14n-x\x14a\x02\xFFW\x80c\x14\xBF\xD6\xD0\x14a\x03\x12W\x80c\x18v\xEE\xD9\x14a\x03IW`\0\x80\xFD[\x80b\xF7\xD4\x9F\x14a\x02\xD7W\x80c\t\xC5\xEA\xBE\x14a\x02\xECW[`\0\x80\xFD[a\x02\xEAa\x02\xE56`\x04a0\x0BV[a\x07|V[\0[a\x02\xEAa\x02\xFA6`\x04a0\x0BV[a\x08\0V[a\x02\xEAa\x03\r6`\x04a0MV[a\r\x1AV[a\x033a\x03 6`\x04a0\x99V[P`@\x80Q`\0\x81R` \x81\x01\x90\x91R\x90V[`@Qa\x03@\x91\x90a0\xB2V[`@Q\x80\x91\x03\x90\xF3[a\x03\\a\x03W6`\x04a0\xFFV[a\r[V[`@Q\x90\x15\x15\x81R` \x01a\x03@V[a\x02\xEAa\x03z6`\x04a1\xB4V[a\x0E\x8FV[a\x03\xA8a\x03\x8D6`\x04a0\x99V[`\0\x90\x81R`\x02` R`@\x90 T`\x01`\x01`\xA0\x1B\x03\x16\x90V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x03@V[a\x03\xD3a\x03\xCE6`\x04a3\x13V[a\x0E\xFAV[`@Q\x90\x81R` \x01a\x03@V[a\x02\xEAa\x03\xEF6`\x04a3PV[a\x0F\x0EV[a\x03\xA8\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`\0a\x03\xD3V[a\x02\xEAa\x0406`\x04a4\x08V[a\x0F\xA6V[a\x02\xEAa\x04C6`\x04a0MV[a\x10\xC6V[a\x02\xEAa\x04V6`\x04a0MV[a\x15\rV[\x7F\xAB\xEAo\xD3\xDBV\xA6\xE6\xD0$!\x11\xB4>\xBB\x13\xD1\xC4'\te\x1C\x03,x\x94\x96 #\xA1\xF9\t`\0R`\x02` R\x7FP\xEB\xFB\xBFQM\xA2\t\xFD\x91\x83\x95/\xC6\x1A\x81\x93\xCD\xFD7\xE0Z)H\xDB$\x99\x0E&JEaT`\x01`\x01`\xA0\x1B\x03\x16a\x03\xA8V[\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC`\0R`\x02` R\x7F\x11\x14\x1FFli\xFD@\x9E\x19\x90\xE0c\xB4\x9C\xD6\xD6\x1E\xD2\xEC\xFF'\xA2\xE4\x02\xE2Y\xCAk\x9A\x01\xA3T`\x01`\x01`\xA0\x1B\x03\x16a\x03\xA8V[a\x03\\a\x05\x1B6`\x04a4%V[a\x15\xBBV[a\x03\xA8\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02\xEAa\x05U6`\x04a4\xECV[a\x16tV[a\x03\\a\x05h6`\x04a0\x99V[`\0\x90\x81R`\x04` R`@\x90 T`\xFF\x16\x90V[a\x03\\a\x05\x8B6`\x04a3\x13V[P`\0\x90V[\x7F*ep\x9D\xA5\xD4\xD3\x08\xD6\x7F\xA5Zm\xC3M\xD7\x996\xF8\xF1\x83\xE6\xFB\x9D$\x81\x14@M\xD8\x1A\xABa\x03\xD3V[a\x02\xEAa\x05\xC56`\x04a0MV[a\x18bV[a\x03\xD3a\x05\x8B6`\x04a0\x99V[a\x03\xA8a\x05\xE66`\x04a3\x13V[a\x1B\x12V[a\x02\xEAa\x05\xF96`\x04a0MV[a\x1B V[a\x06\x11a\x06\x0C6`\x04a0\x99V[a\x1B\xFEV[`@Qa\x03@\x91\x90a5\xB0V[a\x02\xEAa\x06,6`\x04a0\x0BV[a\x1C\xA0V[a\x02\xEAa\x06?6`\x04a5\xC3V[a\x1E\tV[`\0a\x03\\V[a\x02\xEAa\x06Y6`\x04a6\x13V[a!\x1FV[a\x03\\a\x06l6`\x04a6\xDFV[a!\xD5V[a\x03\xD3a\x06\x7F6`\x04a0\x99V[`\0\x90\x81R` \x81\x90R`@\x90 T\x90V[a\x06\x11a\x06\x9F6`\x04a0\x99V[a\"\xA8V[\x7Fb\x7F\x0C\x11s(7\xB3$\n-\xE8\x9C\x0BcCQ(\x86\xDDP\x97\x8B\x99\xC7jh\xC6AjM\x92`\0R`\x02` R\x7FT\x81\xD7!\x19B\x86\x87\xFE=\xCB?\xA9\xE7\xCD0\xAB8\x06\xD1H\xEE\xEBW\xED\xEC\x06\xEB\xE9\x14\x0C\x8BT`\x01`\x01`\xA0\x1B\x03\x16a\x03\xA8V[a\x03\xD3a\x07\x0B6`\x04a3\x13V[a\"\xC5V[a\x03\\a\x07\x1E6`\x04a0\x99V[a\"\xDFV[a\x02\xEAa\x0716`\x04a4\x08V[a\"\xEDV[a\x03\xD3a\x07D6`\x04a0\x99V[`\0\x90\x81R`\x05` R`@\x90 T\x90V[a\x03\\a\x07d6`\x04a7\xADV[a#\xA8V[a\x02\xEAa\x07w6`\x04a0MV[a$1V[`@Qc\xD2\x89\xD1\xCB`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xD2\x89\xD1\xCB\x90a\x07\xCA\x90\x85\x90\x85\x90`\x04\x01a8mV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x07\xE4W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x07\xF8W=`\0\x80>=`\0\xFD[PPPPPPV[`\0\x80a\x08\x0F\x83\x85\x01\x85a8\x81V[\x91P\x91P`\0a\x08%\x83\x80Q\x90` \x01 a$\xD1V[\x90P`\0\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16cs\xE3\xD6j\x83\x85`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x08w\x92\x91\x90a8\xE5V[` `@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x08\x91W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x08\xA5W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x08\xC9\x91\x90a9\x13V[\x90P`\0``\x80``\x87\x80` \x01\x90Q\x81\x01\x90a\x08\xE6\x91\x90a:\xB7V[\x92\x96P\x90\x94P\x92P\x90PF\x84\x14a\t)W`@Q\x7FzG\xC9\xA2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82Q\x82Q\x81\x14\x15\x80a\t<WP\x81Q\x81\x14\x15[\x15a\tsW`@Q\x7F\xCA\x9A(\xF5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0[\x81\x81\x10\x15a\r\x0CW`\0\x85\x82\x81Q\x81\x10a\t\x92Wa\t\x92a;\xA1V[` \x02` \x01\x01Q\x90Pa\t\xA5\x81a\"\xDFV[\x15a\t\xB0WPa\x0C\xFCV[`\0\x80\x86\x84\x81Q\x81\x10a\t\xC5Wa\t\xC5a;\xA1V[` \x02` \x01\x01Q`@Q` \x01a\t\xDD\x91\x90a;\xB7V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x7FWc\x81K\x98\xA3\xAA\x86\xF2\x12yz\xF3'8h\xB5\xDDn*S-vJy\xB9\x8C\xA8Y\xE7\xBB\xAD\x81\x14\x15a\nEW\x7F\x88jb]\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91Pa\x0B\xEEV[\x7F\xECx\xD9\xC2,\x08\xBB\x9F\x0E\xCD]\x95W\x1A\xE8>?\"!\x9CZ\x92x\xC3'\x06\x91\xD5\n\xBF\xD9\x1B\x81\x14\x15a\n\x95W\x7F\x14n-x\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91Pa\x0B\xEEV[\x7F7\xAC\x16\xAA\xBCM\x87T\x0ES\x15\x1B+qbe\xCF\xD6\xB1\x95\xDB\x96\xA9\xDA\xF8\xE8\x93\xC8)\xBB\xD23\x81\x14\x15a\n\xE5W\x7F\xFB\xE0\xA3\x1B\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91Pa\x0B\xEEV[\x7F\xF4\x15\x04%[\x91\x1B0B\xEEO\x87\x86\xFD\xF7\xCFK\xCF$\xAC\xE03\xFA\x16\xAF<\x85t\xE0%\xE46\x81\x14\x15a\x0B5W\x7FXZ\x9F\xD4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91Pa\x0B\xEEV[\x7F\xDA\x19\x9C\x0Ev\xF6e\xE0E\0 y\x1C\x7F\x8E\xAC\xC7_<\xDB\xAC\xE3\x13',(\xF9>S\x90\xB6,\x81\x14\x15a\x0B\x85W\x7FFV\xAE.\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91Pa\x0B\xEEV[\x7F\xB4`\xDC\xB6\xFDW\x97\xFC\x0E~\xA0\xF14\x06\xC8\r0p+\xA7\xF7:B\xBD\x919Gu\xDC\xBC\xA7\x18\x81\x14\x15a\x0B\xE6W\x89a\x0B\xBAWPPPa\x0C\xFCV[`\0\x99P\x7F\x97\xB8{\xA6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91Pa\x0B\xEEV[PPPa\x0C\xFCV[a\x0B\xF9\x83`\x01a%%V[`\x000`\x01`\x01`\xA0\x1B\x03\x16\x83\x88\x87\x81Q\x81\x10a\x0C\x18Wa\x0C\x18a;\xA1V[` \x02` \x01\x01Q\x86`@Q`$\x01a\x0C2\x92\x91\x90a;\xD3V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16`\x01`\x01`\xE0\x1B\x03\x19\x90\x94\x16\x93\x90\x93\x17\x90\x92R\x90Qa\x0Cp\x91\x90a;\xB7V[`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a\x0C\xADW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x0C\xB2V[``\x91P[PP\x90P\x80\x15a\x0C\xECW`@Q\x84\x90\x7F\xA7L\x88G\xD5\x13\xFE\xBA\"\xA0\xF0\xCB8\xD50\x81\xAB\xF9ub\xCD\xB2\x93\x92k\xA2Ch\x9E|A\xCA\x90`\0\x90\xA2a\x0C\xF7V[a\x0C\xF7\x84`\0a%%V[PPPP[a\r\x05\x81a<\x0BV[\x90Pa\tvV[PPPPPPPPPPPPV[30\x14a\r:W`@Qc\x14\xE1\xDB\xF7`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80\x80a\rJ\x85\x87\x01\x87a<1V[\x92P\x92P\x92Pa\x07\xF8\x83\x83\x83a%QV[`\0\x80a\x0E\x0C\x8B\x8B\x8B\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPP`@\x80Q` `\x1F\x8F\x01\x81\x90\x04\x81\x02\x82\x01\x81\x01\x90\x92R\x8D\x81R\x92P\x8D\x91P\x8C\x90\x81\x90\x84\x01\x83\x82\x80\x82\x847`\0\x92\x01\x91\x90\x91RPP`@\x80Q` `\x1F\x8D\x01\x81\x90\x04\x81\x02\x82\x01\x81\x01\x90\x92R\x8B\x81R3\x93P\x8D\x92P\x90\x8C\x90\x8C\x90\x81\x90\x84\x01\x83\x82\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x8C\x92Pa&`\x91PPV[`\0\x81\x81R`\x04` R`@\x90 T`\xFF\x16\x92P\x90P\x81\x15a\x0E\x81W`\0\x81\x81R`\x04` R`@\x90 \x80T`\xFF\x19\x16\x90Ua\x0E\x81\x85\x85\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RP3\x92P\x87\x91Pa%Q\x90PV[P\x99\x98PPPPPPPPPV[\x81\x81`@Qa\x0E\x9F\x92\x91\x90a<\x8BV[`@Q\x80\x91\x03\x90 3`\x01`\x01`\xA0\x1B\x03\x16\x7F0\xAEl\xC7\x8C'\xE6Qt[\xF2\xAD\x08\xA1\x1D\xE89\x10\xAC\x1E4zR\xF7\xAC\x89\x8C\x0F\xBE\xF9M\xAE\x88\x88\x88\x88\x88\x88`@Qa\x0E\xEA\x96\x95\x94\x93\x92\x91\x90a<\x9BV[`@Q\x80\x91\x03\x90\xA3PPPPPPV[`\0a\x0F\x08a\x06\x7F\x83a&\xC4V[\x92\x91PPV[a\x0FP3\x84\x84\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x86\x92Pa&\xF9\x91PPV[3`\x01`\x01`\xA0\x1B\x03\x16\x7Fe\x1D\x93\xF6lC)c\x0E\x8D\x0FbH\x8E\xFFY\x9E;\xE4\x84\xDAXs5\xE8\xDC\x0F\xCFF\x06'&\x88\x88\x88\x88\x88\x88\x88`@Qa\x0F\x95\x97\x96\x95\x94\x93\x92\x91\x90a<\xE4V[`@Q\x80\x91\x03\x90\xA2PPPPPPPV[\x7Fb\x7F\x0C\x11s(7\xB3$\n-\xE8\x9C\x0BcCQ(\x86\xDDP\x97\x8B\x99\xC7jh\xC6AjM\x92`\0R`\x02` R\x7FT\x81\xD7!\x19B\x86\x87\xFE=\xCB?\xA9\xE7\xCD0\xAB8\x06\xD1H\xEE\xEBW\xED\xEC\x06\xEB\xE9\x14\x0C\x8BT`\x01`\x01`\xA0\x1B\x03\x163\x14\x80\x15\x90a\x10\\WP\x7F\xAB\xEAo\xD3\xDBV\xA6\xE6\xD0$!\x11\xB4>\xBB\x13\xD1\xC4'\te\x1C\x03,x\x94\x96 #\xA1\xF9\t`\0R`\x02` R\x7FP\xEB\xFB\xBFQM\xA2\t\xFD\x91\x83\x95/\xC6\x1A\x81\x93\xCD\xFD7\xE0Z)H\xDB$\x99\x0E&JEaT`\x01`\x01`\xA0\x1B\x03\x163\x14\x15[\x15a\x10zW`@Qc\":\xA8=`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16a\x10\xBAW`@Q\x7F\xD7\x9Dw,\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x10\xC3\x81a)\x04V[PV[30\x14a\x10\xE6W`@Qc\x14\xE1\xDB\xF7`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80a\x10\xF5\x84\x86\x01\x86a=5V[\x91P\x91P`\0a\x11\x04\x83a\x1B\x12V[\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16a\x118W\x82`@Qc9]\t\xBF`\xE1\x1B\x81R`\x04\x01a\x11/\x91\x90a5\xB0V[`@Q\x80\x91\x03\x90\xFD[`\x02a\x11C\x84a)\xF9V[`\x02\x81\x11\x15a\x11TWa\x11Ta=zV[\x14\x15a\x14\xAAW`\0a\x12\x11\x83`@Q\x80` \x01a\x11p\x90a/\xB5V[`\x1F\x19\x82\x82\x03\x81\x01\x83R`\x1F\x90\x91\x01\x16`@\x81\x90Ra\x11\x92\x91\x90` \x01a;\xB7V[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x82\x82R\x80Q` \x91\x82\x01 \x7F\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84\x83\x01Rk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x190``\x1B\x16`!\x85\x01R`5\x84\x01\x94\x90\x94R`U\x80\x84\x01\x94\x90\x94R\x81Q\x80\x84\x03\x90\x94\x01\x84R`u\x90\x92\x01\x90R\x81Q\x91\x01 \x90V[\x90Pa\x12%\x81`\x01`\x01`\xA0\x1B\x03\x16a*\x18V[\x15a\x123WPPPPPPPV[`\0\x83`@Qa\x12B\x90a/\xB5V[\x81\x90`@Q\x80\x91\x03\x90`\0\xF5\x90P\x80\x15\x80\x15a\x12bW=`\0\x80>=`\0\xFD[P`@Q\x7Fp\xA0\x821\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x80\x83\x16`\x04\x83\x01\x81\x90R\x92\x93P`\0\x92\x83\x92\x90\x91c\x1C\xFFy\xCD\x91\x88\x91c\xA9\x05\x9C\xBB`\xE0\x1B\x910\x91\x84\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x12\xDCW`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x12\xF0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x13\x14\x91\x90a=\x90V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x92\x16`$\x83\x01R`D\x82\x01R`d\x01`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16`\x01`\x01`\xE0\x1B\x03\x19\x94\x85\x16\x17\x90RQ`\xE0\x85\x90\x1B\x90\x92\x16\x82Ra\x13u\x92\x91`\x04\x01a=\xA9V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x13\x8FW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x13\xA3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x13\xCB\x91\x90\x81\x01\x90a=\xCBV[\x91P\x91P\x81\x15\x80a\x13\xF8WP\x80Q\x15\x80\x15\x90a\x13\xF8WP\x80\x80` \x01\x90Q\x81\x01\x90a\x13\xF6\x91\x90a9\x13V[\x15[\x15a\x141W\x86`@Q\x7F\xE2\x17\xB0\xAD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x11/\x91\x90a5\xB0V[`@Q~\xF5]\x9D\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x84\x16\x90b\xF5]\x9D\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x14\x89W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x14\x9DW=`\0\x80>=`\0\xFD[PPPPPPPPa\x07\xF8V[`@Qc\x08\xA1\xEE\xE1`\xE0\x1B\x81R`\x04\x81\x01\x83\x90R`\x01`\x01`\xA0\x1B\x03\x82\x16\x90c\x08\xA1\xEE\xE1\x90`$\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x14\xECW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x15\0W=`\0\x80>=`\0\xFD[PPPPPPP[PPPV[30\x14a\x15-W`@Qc\x14\xE1\xDB\xF7`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80\x80\x80\x80\x80\x80\x80a\x15B\x8A\x8C\x01\x8Ca>\x0FV[\x97P\x97P\x97P\x97P\x97P\x97P\x97P\x97Pa\x15a\x89\x89\x89\x89\x89\x89\x89a*[V[\x84\x86`\x01`\x01`\xA0\x1B\x03\x16\x8A\x7F\x99\x91\xFA\xA1\xF45gQY\xFF\xAEd\xB6m~\xCF\xDBU\xC2\x97U\x86\x9A\x18\xDB\x84\x97\xB49#G\xE0\x8B\x8B\x89\x89\x89\x89`@Qa\x15\xA6\x96\x95\x94\x93\x92\x91\x90a>\xCDV[`@Q\x80\x91\x03\x90\xA4PPPPPPPPPPPV[`\0\x80a\x166\x88\x88\x88\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPP`@\x80Q` `\x1F\x8C\x01\x81\x90\x04\x81\x02\x82\x01\x81\x01\x90\x92R\x8A\x81R\x92P\x8A\x91P\x89\x90\x81\x90\x84\x01\x83\x82\x80\x82\x847`\0\x92\x01\x91\x90\x91RP3\x92P\x89\x91Pa*\x88\x90PV[`\0\x81\x81R`\x04` R`@\x90 T`\xFF\x16\x92P\x90P\x81\x15a\x16iW`\0\x81\x81R`\x04` R`@\x90 \x80T`\xFF\x19\x16\x90U[P\x96\x95PPPPPPV[\x7Fb\x7F\x0C\x11s(7\xB3$\n-\xE8\x9C\x0BcCQ(\x86\xDDP\x97\x8B\x99\xC7jh\xC6AjM\x92`\0R`\x02` R\x7FT\x81\xD7!\x19B\x86\x87\xFE=\xCB?\xA9\xE7\xCD0\xAB8\x06\xD1H\xEE\xEBW\xED\xEC\x06\xEB\xE9\x14\x0C\x8BT`\x01`\x01`\xA0\x1B\x03\x163\x14\x80\x15\x90a\x17*WP\x7F\xAB\xEAo\xD3\xDBV\xA6\xE6\xD0$!\x11\xB4>\xBB\x13\xD1\xC4'\te\x1C\x03,x\x94\x96 #\xA1\xF9\t`\0R`\x02` R\x7FP\xEB\xFB\xBFQM\xA2\t\xFD\x91\x83\x95/\xC6\x1A\x81\x93\xCD\xFD7\xE0Z)H\xDB$\x99\x0E&JEaT`\x01`\x01`\xA0\x1B\x03\x163\x14\x15[\x15a\x17HW`@Qc\":\xA8=`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x82\x81\x81\x14a\x17\x82W`@Q\x7F\x14\xA2'_\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0[\x81\x81\x10\x15a\x07\xF8W`\0\x86\x86\x83\x81\x81\x10a\x17\xA1Wa\x17\xA1a;\xA1V[\x90P` \x02\x81\x01\x90a\x17\xB3\x91\x90a?%V[\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x82\x90RP\x93\x94P\x88\x92P\x87\x91P\x85\x90P\x81\x81\x10a\x17\xFEWa\x17\xFEa;\xA1V[\x90P` \x02\x015\x90P`\0`\x01`\x01`\xA0\x1B\x03\x16a\x18\x1B\x83a\x1B\x12V[`\x01`\x01`\xA0\x1B\x03\x16\x14\x15a\x18EW\x81`@Qc9]\t\xBF`\xE1\x1B\x81R`\x04\x01a\x11/\x91\x90a5\xB0V[a\x18O\x82\x82a*\xE6V[PP\x80a\x18[\x90a<\x0BV[\x90Pa\x17\x85V[30\x14a\x18\x82W`@Qc\x14\xE1\xDB\xF7`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80\x80\x80\x80\x80a\x18\x95\x88\x8A\x01\x8Aa?lV[\x95P\x95P\x95P\x95P\x95P\x95P`\0`\x01`\x01`\xA0\x1B\x03\x16a\x18\xB5\x86a\x1B\x12V[`\x01`\x01`\xA0\x1B\x03\x16\x14a\x18\xF7W\x84`@Q\x7F\xAA~\x8B2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x11/\x91\x90a5\xB0V[a\x19\x01\x85\x82a*\xE6V[`\x01`\x01`\xA0\x1B\x03\x82\x16a\x1AmW`\0\x85`@Q` \x01a\x19\"\x91\x90a;\xB7V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90Pa\x19E\x86`\x01a+>V[`\0\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16co\xC9[4`\xE0\x1B\x8A\x8A\x8A\x8A\x88`@Q`$\x01a\x19\x93\x95\x94\x93\x92\x91\x90a@\x0FV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16`\x01`\x01`\xE0\x1B\x03\x19\x90\x94\x16\x93\x90\x93\x17\x90\x92R\x90Qa\x19\xD1\x91\x90a;\xB7V[`\0`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80`\0\x81\x14a\x1A\x0CW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x1A\x11V[``\x91P[P\x91P\x91P\x81a\x1AOW\x87`@Q\x7F\x86\xD5'C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x11/\x91\x90a5\xB0V[\x80\x80` \x01\x90Q\x81\x01\x90a\x1Ac\x91\x90a@UV[\x94PPPPa\x1A\xC4V[`\x01`\x01`\xA0\x1B\x03\x82\x16;a\x1A\xB9W`@Q\x7F\xC5\xCC\xDD\xDE\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x83\x16`\x04\x82\x01R`$\x01a\x11/V[a\x1A\xC4\x85`\x02a+>V[\x7F\xBF\x90\xB5\xA1\xEC\x97c\xE8\xBFK\x92E\xCE\xF0\xC2\x8D\xB9+\xAB0\x9F\xC2\xC5\x17\x7F\x17\x81O8$i8\x85\x83`@Qa\x1A\xF5\x92\x91\x90a@rV[`@Q\x80\x91\x03\x90\xA1a\x1B\x07\x85\x83a+nV[PPPPPPPPPV[`\0a\x0F\x08a\x03\x8D\x83a+\xB0V[30\x14a\x1B@W`@Qc\x14\xE1\xDB\xF7`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x7F\x19.u\x9EU\xF3Y\xCD\x982\xB5\xC0\xC6\xE3\x8EKm\xF5\xC5\xCA3\xF3\xBD\\\x90s\x8E\x86ZR\x18r\x83\x83`@Qa\x1Bq\x92\x91\x90a8mV[`@Q\x80\x91\x03\x90\xA1`@Qc\xD2\x89\xD1\xCB`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xD2\x89\xD1\xCB\x90a\x1B\xC7\x90\x86\x90\x86\x90`\x04\x01a8mV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1B\xE1W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x1B\xF5W=`\0\x80>=`\0\xFD[PPPPPPPV[`\0\x81\x81R`\x01` R`@\x90 \x80T``\x91\x90a\x1C\x1B\x90a@\x9DV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x1CG\x90a@\x9DV[\x80\x15a\x1C\x94W\x80`\x1F\x10a\x1CiWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x1C\x94V[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x1CwW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x90P\x91\x90PV[0`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x15a\x1D\x03W`@Q\x7F\xBF\x10\xDD:\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80\x80a\x1D\x13\x84\x86\x01\x86a@\xD8V[\x91\x94P\x92P\x90P`\x01`\x01`\xA0\x1B\x03\x83\x16\x15a\x1D2Wa\x1D2\x83a+\xE5V[`\x01`\x01`\xA0\x1B\x03\x82\x16\x15a\x1DJWa\x1DJ\x82a)\x04V[\x80Q\x15a\x1E\x02W\x7F\x19.u\x9EU\xF3Y\xCD\x982\xB5\xC0\xC6\xE3\x8EKm\xF5\xC5\xCA3\xF3\xBD\\\x90s\x8E\x86ZR\x18r\x81`@Qa\x1D\x80\x91\x90a5\xB0V[`@Q\x80\x91\x03\x90\xA1`@Qc\xD2\x89\xD1\xCB`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xD2\x89\xD1\xCB\x90a\x1D\xD4\x90\x84\x90`\x04\x01a5\xB0V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x1D\xEEW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x1B\x07W=`\0\x80>=`\0\xFD[PPPPPV[\x7F\xAB\xEAo\xD3\xDBV\xA6\xE6\xD0$!\x11\xB4>\xBB\x13\xD1\xC4'\te\x1C\x03,x\x94\x96 #\xA1\xF9\t`\0R`\x02` R\x7FP\xEB\xFB\xBFQM\xA2\t\xFD\x91\x83\x95/\xC6\x1A\x81\x93\xCD\xFD7\xE0Z)H\xDB$\x99\x0E&JEaT`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1E|W`@Qc-[\xE4\xCB`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x83`\x01`\x01`\xA0\x1B\x03\x16?\x83\x14a\x1E\xBFW`@Q\x7F\x8F\x84\xFB$\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x83`\x01`\x01`\xA0\x1B\x03\x16c\x82\x91(l`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a\x1E\xF8W`\0\x80\xFD[PZ\xFA\x15\x80\x15a\x1F\x0CW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1F0\x91\x90a=\x90V[\x7F*ep\x9D\xA5\xD4\xD3\x08\xD6\x7F\xA5Zm\xC3M\xD7\x996\xF8\xF1\x83\xE6\xFB\x9D$\x81\x14@M\xD8\x1A\xAB\x14a\x1F\x88W`@Q\x7Fh\x15_\x9A\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`@Q`\x01`\x01`\xA0\x1B\x03\x85\x16\x90\x7F\xBC|\xD7Z \xEE'\xFD\x9A\xDE\xBA\xB3 A\xF7U!M\xBCk\xFF\xA9\x0C\xC0\"[9\xDA.\\-;\x90`\0\x90\xA2a 3\x84\x7F6\x08\x94\xA1;\xA1\xA3!\x06g\xC8(I-\xB9\x8D\xCA> v\xCC75\xA9 \xA3\xCAP]8+\xBC`\0R`\x02` R\x7F\x11\x14\x1FFli\xFD@\x9E\x19\x90\xE0c\xB4\x9C\xD6\xD6\x1E\xD2\xEC\xFF'\xA2\xE4\x02\xE2Y\xCAk\x9A\x01\xA3\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x17\x90UPV[\x80\x15a!\x19W`\0\x84`\x01`\x01`\xA0\x1B\x03\x16c\x9D\xED\x06\xDF`\xE0\x1B\x84\x84`@Q`$\x01a `\x92\x91\x90a8mV[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x81R` \x82\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16`\x01`\x01`\xE0\x1B\x03\x19\x90\x94\x16\x93\x90\x93\x17\x90\x92R\x90Qa \x9E\x91\x90a;\xB7V[`\0`@Q\x80\x83\x03\x81\x85Z\xF4\x91PP=\x80`\0\x81\x14a \xD9W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a \xDEV[``\x91P[PP\x90P\x80a\x1E\x02W`@Q\x7F\x97\x90]\xFB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[PPPPV[a!a3\x84\x84\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x86\x92Pa&\xF9\x91PPV[\x84\x84`@Qa!q\x92\x91\x90a<\x8BV[`@Q\x80\x91\x03\x90 3`\x01`\x01`\xA0\x1B\x03\x16\x7F~PV\x9D&\xBEd;\xDAwWr\"\x91\xECf\xB1\xBEf\xD8(4t\xAE?\xABZ\x98\xF8x\xA7\xA2\x8B\x8B\x8B\x8B\x8B\x8B\x8B\x8B\x8B`@Qa!\xC2\x99\x98\x97\x96\x95\x94\x93\x92\x91\x90aA:V[`@Q\x80\x91\x03\x90\xA3PPPPPPPPPV[`\0a\"\x99a\x05h\x8C\x8C\x8C\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x81\x84\x01R`\x1F\x19`\x1F\x82\x01\x16\x90P\x80\x83\x01\x92PPPPPPP\x8B\x8B\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPP`@\x80Q` `\x1F\x8D\x01\x81\x90\x04\x81\x02\x82\x01\x81\x01\x90\x92R\x8B\x81R\x8E\x93P\x8D\x92P\x90\x8C\x90\x8C\x90\x81\x90\x84\x01\x83\x82\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x8C\x92Pa&`\x91PPV[\x9B\x9APPPPPPPPPPPV[`\0\x81\x81R`\x03` R`@\x90 \x80T``\x91\x90a\x1C\x1B\x90a@\x9DV[`\0a\x0F\x08a\x06\x7F\x83a\"\xDAaT`BaA\xA2V[a,\xDAV[`\0a\x0F\x08a\x05h\x83a-/V[\x7F\xAB\xEAo\xD3\xDBV\xA6\xE6\xD0$!\x11\xB4>\xBB\x13\xD1\xC4'\te\x1C\x03,x\x94\x96 #\xA1\xF9\t`\0R`\x02` R\x7FP\xEB\xFB\xBFQM\xA2\t\xFD\x91\x83\x95/\xC6\x1A\x81\x93\xCD\xFD7\xE0Z)H\xDB$\x99\x0E&JEaT`\x01`\x01`\xA0\x1B\x03\x163\x14a#`W`@Qc-[\xE4\xCB`\xE2\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x01`\x01`\xA0\x1B\x03\x81\x16a#\x9FW`@Q~c\x18l\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a\x10\xC3\x81a+\xE5V[`\0a$%a\x05h\x89\x89\x89\x80\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x93\x92\x91\x90\x81\x81R` \x01\x83\x83\x80\x82\x847`\0\x92\x01\x91\x90\x91RPP`@\x80Q` `\x1F\x8D\x01\x81\x90\x04\x81\x02\x82\x01\x81\x01\x90\x92R\x8B\x81R\x92P\x8B\x91P\x8A\x90\x81\x90\x84\x01\x83\x82\x80\x82\x847`\0\x92\x01\x91\x90\x91RP\x8A\x92P\x89\x91Pa*\x88\x90PV[\x98\x97PPPPPPPPV[30\x14a$QW`@Qc\x14\xE1\xDB\xF7`\xE1\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80\x80\x80\x80\x80a$d\x88\x8A\x01\x8AaA\xC4V[\x95P\x95P\x95P\x95P\x95P\x95Pa$}\x87\x87\x87\x87\x87a-jV[\x82\x84`\x01`\x01`\xA0\x1B\x03\x16\x88\x7FD\xE4\xF8\xF6\xBDh,Z:\xEB\xA96\x01\xAB\x07\xCBM\x1F!\xB2\xAA\xB1\xAEH\x80\xD9Wy\x190\x9A\xA4\x89\x89\x87\x87`@Qa$\xBE\x94\x93\x92\x91\x90aBVV[`@Q\x80\x91\x03\x90\xA4PPPPPPPPPV[`@Q\x7F\x19Ethereum Signed Message:\n32\0\0\0\0` \x82\x01R`<\x81\x01\x82\x90R`\0\x90`\\\x01[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x91\x90PV[a%Ma%1\x83a-/V[`\0\x90\x81R`\x04` R`@\x90 \x80T`\xFF\x19\x16\x83\x15\x15\x17\x90UV[PPV[`\0a%\\\x84a\x1B\x12V[\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16a%\x87W\x83`@Qc9]\t\xBF`\xE1\x1B\x81R`\x04\x01a\x11/\x91\x90a5\xB0V[a%\xA4\x84\x83a%\x95\x87a\"\xC5V[a%\x9F\x91\x90aB\x8FV[a-zV[`\x02a%\xAF\x85a)\xF9V[`\x02\x81\x11\x15a%\xC0Wa%\xC0a=zV[\x14\x15a%\xDFWa%\xDA`\x01`\x01`\xA0\x1B\x03\x82\x16\x84\x84a-\xF5V[a!\x19V[`@Q\x7F@\xC1\x0F\x19\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x01`\x01`\xA0\x1B\x03\x84\x81\x16`\x04\x83\x01R`$\x82\x01\x84\x90R\x82\x16\x90c@\xC1\x0F\x19\x90`D\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a&BW`\0\x80\xFD[PZ\xF1\x15\x80\x15a&VW=`\0\x80>=`\0\xFD[PPPPPPPPV[`\0\x7F\xB7\xAD\x97+qGX`a=\xB3\xBA\x1F\xE6\x99\xB8\x86\xC8x\xF9\0*\t%\r\xC2^v\x9E\xB1\x9A\x10\x88\x88\x88\x88\x88\x88\x88`@Q` \x01a&\xA1\x98\x97\x96\x95\x94\x93\x92\x91\x90aB\xA7V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x97\x96PPPPPPPV[`\0\x7F\xEE\xE94\x8BJ\xAB\xA3d{\x16\x12\xB2rO\x18\xE9;\x92\x99\xDA&\xFB2\x1C{?\xDA\x13]}\xEA\x87\x82`@Q` \x01a%\x08\x92\x91\x90aC\x1AV[`\0a'\x04\x83a\x1B\x12V[\x90P`\x01`\x01`\xA0\x1B\x03\x81\x16a'/W\x82`@Qc9]\t\xBF`\xE1\x1B\x81R`\x04\x01a\x11/\x91\x90a5\xB0V[\x81a'fW`@Q\x7F,R\x11\xC6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0a'q\x84a)\xF9V[\x90P`\x02\x81`\x02\x81\x11\x15a'\x87Wa'\x87a=zV[\x14\x15a'\xA7Wa'\xA2`\x01`\x01`\xA0\x1B\x03\x83\x16\x860\x86a.XV[a\x1E\x02V[`\x01\x81`\x02\x81\x11\x15a'\xBBWa'\xBBa=zV[\x14\x15a(.W`@\x80Q`\x01`\x01`\xA0\x1B\x03\x87\x81\x16`$\x83\x01R`D\x80\x83\x01\x87\x90R\x83Q\x80\x84\x03\x90\x91\x01\x81R`d\x90\x92\x01\x90\x92R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16\x7Fy\xCCg\x90\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90Ra'\xA2\x91\x84\x16\x90a.\xA9V[`@Q\x7F1\xEE\xCA\xF4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\0`\x04\x82\x01Ra(\xD7\x90\x86\x90`\x01`\x01`\xA0\x1B\x03\x85\x16\x90c1\xEE\xCA\xF4\x90`$\x01` `@Q\x80\x83\x03\x81\x86\x80;\x15\x80\x15a(\x8DW`\0\x80\xFD[PZ\xFA\x15\x80\x15a(\xA1W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a(\xC5\x91\x90a@UV[`\x01`\x01`\xA0\x1B\x03\x85\x16\x91\x90\x86a.XV[`@Qc\x08\xA1\xEE\xE1`\xE0\x1B\x81R`\0`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x83\x16\x90c\x08\xA1\xEE\xE1\x90`$\x01a\x1D\xD4V[\x7Fb\x7F\x0C\x11s(7\xB3$\n-\xE8\x9C\x0BcCQ(\x86\xDDP\x97\x8B\x99\xC7jh\xC6AjM\x92`\0\x90\x81R`\x02` R\x7FT\x81\xD7!\x19B\x86\x87\xFE=\xCB?\xA9\xE7\xCD0\xAB8\x06\xD1H\xEE\xEBW\xED\xEC\x06\xEB\xE9\x14\x0C\x8BT`@Q`\x01`\x01`\xA0\x1B\x03\x84\x81\x16\x93\x92\x16\x91\x7F\xA90<\x86\x0C]\xE3\xC0\xC8f\xC3T\xD2\x81x\\\x89w\x8A\xC5\xCA-\xFF\xDF\x12\x84\x1CE\xCDN\x1En\x91\xA3\x7Fb\x7F\x0C\x11s(7\xB3$\n-\xE8\x9C\x0BcCQ(\x86\xDDP\x97\x8B\x99\xC7jh\xC6AjM\x92`\0R`\x02` R\x7FT\x81\xD7!\x19B\x86\x87\xFE=\xCB?\xA9\xE7\xCD0\xAB8\x06\xD1H\xEE\xEBW\xED\xEC\x06\xEB\xE9\x14\x0C\x8B\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x17\x90UPV[`\0a*\x07a\x06\x7F\x83a/\x80V[`\x02\x81\x11\x15a\x0F\x08Wa\x0F\x08a=zV[`\0`\x01`\x01`\xA0\x1B\x03\x82\x16?\x80\x15\x80\x15\x90a*TWP\x7F\xC5\xD2F\x01\x86\xF7#<\x92~}\xB2\xDC\xC7\x03\xC0\xE5\0\xB6S\xCA\x82';{\xFA\xD8\x04]\x85\xA4p\x81\x14\x15[\x93\x92PPPV[a\x1B\xF5a*m\x88\x88\x88\x88\x88\x88\x88a&`V[`\0\x90\x81R`\x04` R`@\x90 \x80T`\xFF\x19\x16`\x01\x17\x90UV[`\0\x7F\x07\xB0\xD40O\x82\x01+\xD3\xB7\x0B\x1DS\x1C\x16\x0E2`g\xC9\x08)\xE2\xA3\xD3\x86r*\xD1\x0B\x89\xC3\x86\x86\x86\x86\x86`@Q` \x01a*\xC5\x96\x95\x94\x93\x92\x91\x90aC@V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x95\x94PPPPPV[\x7F\xD9\x94F\xC1\xD7c\x85\xBBU\x19\xCC\xFBRt\xAB\xCF\xD5\x89m\xFC\"@^@\x01\x0F\xDE!\x7F\x01\x8A\x18\x82\x82`@Qa+\x17\x92\x91\x90a;\xD3V[`@Q\x80\x91\x03\x90\xA1a%Ma++\x83a&\xC4V[\x82`\0\x91\x82R` \x82\x90R`@\x90\x91 UV[a%Ma+J\x83a/\x80V[\x82`\x02\x81\x11\x15a+\\Wa+\\a=zV[`\0\x91\x82R` \x82\x90R`@\x90\x91 UV[a%Ma+z\x83a+\xB0V[`\0\x90\x81R`\x02` R`@\x90 \x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`\xA0\x1B\x03\x84\x16\x17\x90UV[`\0\x7F\xC4\xE62w\x9Ajx8sm\xD7\xE5\xE6\xA0\xEA\xDF\x17\x1D\xD3}\xFBb0r\x0E&Uv\xDF\xCFB\xBB\x82`@Q` \x01a%\x08\x92\x91\x90aC\x1AV[\x7F\xAB\xEAo\xD3\xDBV\xA6\xE6\xD0$!\x11\xB4>\xBB\x13\xD1\xC4'\te\x1C\x03,x\x94\x96 #\xA1\xF9\t`\0\x90\x81R`\x02` R\x7FP\xEB\xFB\xBFQM\xA2\t\xFD\x91\x83\x95/\xC6\x1A\x81\x93\xCD\xFD7\xE0Z)H\xDB$\x99\x0E&JEaT`@Q`\x01`\x01`\xA0\x1B\x03\x84\x81\x16\x93\x92\x16\x91\x7F_V\xBE\xE8\xCF\xFB\xE9\xA7\x86R\xA7J`p^\xDE\xDE\x02\xAF\x10\xB0\xBB\xB8\x88\xCAD\xB7\x9A\rB\xCE\x80\x91\xA3\x7F\xAB\xEAo\xD3\xDBV\xA6\xE6\xD0$!\x11\xB4>\xBB\x13\xD1\xC4'\te\x1C\x03,x\x94\x96 #\xA1\xF9\t`\0R`\x02` R\x7FP\xEB\xFB\xBFQM\xA2\t\xFD\x91\x83\x95/\xC6\x1A\x81\x93\xCD\xFD7\xE0Z)H\xDB$\x99\x0E&JEa\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x17\x90UPV[`\0\x7F/\x88Q\xFE\rmS~U*O%\xB7\xA3\x16}H\xEB\x12\x92b,q\xD8F0\xA2\xA4GW\xBC\xED\x83\x83`@Q` \x01a-\x11\x93\x92\x91\x90aC\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x90P\x92\x91PPV[`@\x80Q\x7F\x95w\x05\xA3t2k0\xF4\xA1\x06\x9C\x93msl\xC9\x99>\xD6\xC8 \xB4\xE0\xE2\xFD\x94\xA8\xBE\xCA\r\x1D` \x82\x01R\x90\x81\x01\x82\x90R`\0\x90``\x01a%\x08V[a\x1E\x02a*m\x86\x86\x86\x86\x86a*\x88V[`\0a-\x85\x83a\x0E\xFAV[\x90P`\0\x81\x11\x80\x15a-\x96WP\x80\x82\x11[\x15a-\xCFW\x82`@Q\x7F\x03\x7F`\xE5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x11/\x91\x90a5\xB0V[a\x15\x08a-\xE2\x84a\"\xDAaT`BaA\xA2V[\x83`\0\x91\x82R` \x82\x90R`@\x90\x91 UV[`@Q`\x01`\x01`\xA0\x1B\x03\x83\x16`$\x82\x01R`D\x81\x01\x82\x90Ra\x15\x08\x90\x84\x90c\xA9\x05\x9C\xBB`\xE0\x1B\x90`d\x01[`@\x80Q`\x1F\x19\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q`\x01`\x01`\xE0\x1B\x03\x16`\x01`\x01`\xE0\x1B\x03\x19\x90\x93\x16\x92\x90\x92\x17\x90\x91Ra.\xA9V[`@Q`\x01`\x01`\xA0\x1B\x03\x80\x85\x16`$\x83\x01R\x83\x16`D\x82\x01R`d\x81\x01\x82\x90Ra!\x19\x90\x85\x90\x7F#\xB8r\xDD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90`\x84\x01a.!V[`\0\x80\x83`\x01`\x01`\xA0\x1B\x03\x16\x83`@Qa.\xC4\x91\x90a;\xB7V[`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a/\x01W`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a/\x06V[``\x91P[P\x91P\x91P`\0\x82\x80\x15a/2WP\x81Q\x15\x80a/2WP\x81\x80` \x01\x90Q\x81\x01\x90a/2\x91\x90a9\x13V[\x90P\x80\x15\x80a/IWP`\x01`\x01`\xA0\x1B\x03\x85\x16;\x15[\x15a\x1E\x02W`@Q\x7F\x04\\K\x02\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x7F\xA8\r\"Y\xAFU\x89\x06\x18\xEC.\xEB:\xC7-\xE4\xBD\xBA\"R\x9B\xB1HE\xD8\xA3\xD7\x12\xD1\xC3\xF6!\x82`@Q` \x01a%\x08\x92\x91\x90aC\x1AV[a\x03I\x80aC\xBA\x839\x01\x90V[`\0\x80\x83`\x1F\x84\x01\x12a/\xD4W`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a/\xECW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a0\x04W`\0\x80\xFD[\x92P\x92\x90PV[`\0\x80` \x83\x85\x03\x12\x15a0\x1EW`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a05W`\0\x80\xFD[a0A\x85\x82\x86\x01a/\xC2V[\x90\x96\x90\x95P\x93PPPPV[`\0\x80`\0`@\x84\x86\x03\x12\x15a0bW`\0\x80\xFD[\x835g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a0yW`\0\x80\xFD[a0\x85\x86\x82\x87\x01a/\xC2V[\x90\x97\x90\x96P` \x95\x90\x95\x015\x94\x93PPPPV[`\0` \x82\x84\x03\x12\x15a0\xABW`\0\x80\xFD[P5\x91\x90PV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a0\xF3W\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a0\xCEV[P\x90\x96\x95PPPPPPV[`\0\x80`\0\x80`\0\x80`\0\x80`\0`\xC0\x8A\x8C\x03\x12\x15a1\x1DW`\0\x80\xFD[\x895\x98P` \x8A\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a1<W`\0\x80\xFD[a1H\x8D\x83\x8E\x01a/\xC2V[\x90\x9AP\x98P`@\x8C\x015\x91P\x80\x82\x11\x15a1aW`\0\x80\xFD[a1m\x8D\x83\x8E\x01a/\xC2V[\x90\x98P\x96P``\x8C\x015\x95P`\x80\x8C\x015\x91P\x80\x82\x11\x15a1\x8DW`\0\x80\xFD[Pa1\x9A\x8C\x82\x8D\x01a/\xC2V[\x9A\x9D\x99\x9CP\x97\x9A\x96\x99\x95\x98\x94\x97\x96`\xA0\x015\x94\x93PPPPV[`\0\x80`\0\x80`\0\x80``\x87\x89\x03\x12\x15a1\xCDW`\0\x80\xFD[\x865g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a1\xE5W`\0\x80\xFD[a1\xF1\x8A\x83\x8B\x01a/\xC2V[\x90\x98P\x96P` \x89\x015\x91P\x80\x82\x11\x15a2\nW`\0\x80\xFD[a2\x16\x8A\x83\x8B\x01a/\xC2V[\x90\x96P\x94P`@\x89\x015\x91P\x80\x82\x11\x15a2/W`\0\x80\xFD[Pa2<\x89\x82\x8A\x01a/\xC2V[\x97\x9A\x96\x99P\x94\x97P\x92\x95\x93\x94\x92PPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a2\x8DWa2\x8Da2NV[`@R\x91\x90PV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a2\xAFWa2\xAFa2NV[P`\x1F\x01`\x1F\x19\x16` \x01\x90V[`\0\x82`\x1F\x83\x01\x12a2\xCEW`\0\x80\xFD[\x815a2\xE1a2\xDC\x82a2\x95V[a2dV[\x81\x81R\x84` \x83\x86\x01\x01\x11\x15a2\xF6W`\0\x80\xFD[\x81` \x85\x01` \x83\x017`\0\x91\x81\x01` \x01\x91\x90\x91R\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15a3%W`\0\x80\xFD[\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a3<W`\0\x80\xFD[a3H\x84\x82\x85\x01a2\xBDV[\x94\x93PPPPV[`\0\x80`\0\x80`\0\x80`\0`\x80\x88\x8A\x03\x12\x15a3kW`\0\x80\xFD[\x875g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a3\x83W`\0\x80\xFD[a3\x8F\x8B\x83\x8C\x01a/\xC2V[\x90\x99P\x97P` \x8A\x015\x91P\x80\x82\x11\x15a3\xA8W`\0\x80\xFD[a3\xB4\x8B\x83\x8C\x01a/\xC2V[\x90\x97P\x95P`@\x8A\x015\x91P\x80\x82\x11\x15a3\xCDW`\0\x80\xFD[Pa3\xDA\x8A\x82\x8B\x01a/\xC2V[\x98\x9B\x97\x9AP\x95\x98\x94\x97\x95\x96``\x90\x95\x015\x94\x93PPPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x10\xC3W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a4\x1AW`\0\x80\xFD[\x815a*T\x81a3\xF3V[`\0\x80`\0\x80`\0\x80`\x80\x87\x89\x03\x12\x15a4>W`\0\x80\xFD[\x865\x95P` \x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a4]W`\0\x80\xFD[a4i\x8A\x83\x8B\x01a/\xC2V[\x90\x97P\x95P`@\x89\x015\x91P\x80\x82\x11\x15a4\x82W`\0\x80\xFD[Pa4\x8F\x89\x82\x8A\x01a/\xC2V[\x97\x9A\x96\x99P\x94\x97\x94\x96\x95``\x90\x95\x015\x94\x93PPPPV[`\0\x80\x83`\x1F\x84\x01\x12a4\xB9W`\0\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a4\xD1W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a0\x04W`\0\x80\xFD[`\0\x80`\0\x80`@\x85\x87\x03\x12\x15a5\x02W`\0\x80\xFD[\x845g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a5\x1AW`\0\x80\xFD[a5&\x88\x83\x89\x01a4\xA7V[\x90\x96P\x94P` \x87\x015\x91P\x80\x82\x11\x15a5?W`\0\x80\xFD[Pa5L\x87\x82\x88\x01a4\xA7V[\x95\x98\x94\x97P\x95PPPPV[`\0[\x83\x81\x10\x15a5sW\x81\x81\x01Q\x83\x82\x01R` \x01a5[V[\x83\x81\x11\x15a!\x19WPP`\0\x91\x01RV[`\0\x81Q\x80\x84Ra5\x9C\x81` \x86\x01` \x86\x01a5XV[`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01` \x01\x92\x91PPV[` \x81R`\0a*T` \x83\x01\x84a5\x84V[`\0\x80`\0\x80``\x85\x87\x03\x12\x15a5\xD9W`\0\x80\xFD[\x845a5\xE4\x81a3\xF3V[\x93P` \x85\x015\x92P`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a6\x07W`\0\x80\xFD[a5L\x87\x82\x88\x01a/\xC2V[`\0\x80`\0\x80`\0\x80`\0\x80`\0`\xA0\x8A\x8C\x03\x12\x15a61W`\0\x80\xFD[\x895g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a6IW`\0\x80\xFD[a6U\x8D\x83\x8E\x01a/\xC2V[\x90\x9BP\x99P` \x8C\x015\x91P\x80\x82\x11\x15a6nW`\0\x80\xFD[a6z\x8D\x83\x8E\x01a/\xC2V[\x90\x99P\x97P`@\x8C\x015\x91P\x80\x82\x11\x15a6\x93W`\0\x80\xFD[a6\x9F\x8D\x83\x8E\x01a/\xC2V[\x90\x97P\x95P``\x8C\x015\x91P\x80\x82\x11\x15a6\xB8W`\0\x80\xFD[Pa6\xC5\x8C\x82\x8D\x01a/\xC2V[\x9A\x9D\x99\x9CP\x97\x9A\x96\x99\x95\x98\x94\x97\x96`\x80\x015\x94\x93PPPPV[`\0\x80`\0\x80`\0\x80`\0\x80`\0\x80`\xE0\x8B\x8D\x03\x12\x15a6\xFEW`\0\x80\xFD[\x8A5\x99P` \x8B\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a7\x1DW`\0\x80\xFD[a7)\x8E\x83\x8F\x01a/\xC2V[\x90\x9BP\x99P`@\x8D\x015\x91P\x80\x82\x11\x15a7BW`\0\x80\xFD[a7N\x8E\x83\x8F\x01a/\xC2V[\x90\x99P\x97P``\x8D\x015\x91Pa7c\x82a3\xF3V[\x90\x95P`\x80\x8C\x015\x94P`\xA0\x8C\x015\x90\x80\x82\x11\x15a7\x80W`\0\x80\xFD[Pa7\x8D\x8D\x82\x8E\x01a/\xC2V[\x91P\x80\x94PP\x80\x92PP`\xC0\x8B\x015\x90P\x92\x95\x98\x9B\x91\x94\x97\x9AP\x92\x95\x98PV[`\0\x80`\0\x80`\0\x80`\0`\xA0\x88\x8A\x03\x12\x15a7\xC8W`\0\x80\xFD[\x875\x96P` \x88\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a7\xE7W`\0\x80\xFD[a7\xF3\x8B\x83\x8C\x01a/\xC2V[\x90\x98P\x96P`@\x8A\x015\x91P\x80\x82\x11\x15a8\x0CW`\0\x80\xFD[Pa8\x19\x8A\x82\x8B\x01a/\xC2V[\x90\x95P\x93PP``\x88\x015a8-\x81a3\xF3V[\x80\x92PP`\x80\x88\x015\x90P\x92\x95\x98\x91\x94\x97P\x92\x95PV[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[` \x81R`\0a3H` \x83\x01\x84\x86a8DV[`\0\x80`@\x83\x85\x03\x12\x15a8\x94W`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a8\xACW`\0\x80\xFD[a8\xB8\x86\x83\x87\x01a2\xBDV[\x93P` \x85\x015\x91P\x80\x82\x11\x15a8\xCEW`\0\x80\xFD[Pa8\xDB\x85\x82\x86\x01a2\xBDV[\x91PP\x92P\x92\x90PV[\x82\x81R`@` \x82\x01R`\0a3H`@\x83\x01\x84a5\x84V[\x80Q\x80\x15\x15\x81\x14a9\x0EW`\0\x80\xFD[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a9%W`\0\x80\xFD[a*T\x82a8\xFEV[`\0g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a9HWa9Ha2NV[P`\x05\x1B` \x01\x90V[`\0a9`a2\xDC\x84a2\x95V[\x90P\x82\x81R\x83\x83\x83\x01\x11\x15a9tW`\0\x80\xFD[a*T\x83` \x83\x01\x84a5XV[`\0\x82`\x1F\x83\x01\x12a9\x93W`\0\x80\xFD[\x81Q` a9\xA3a2\xDC\x83a9.V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a9\xC2W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a\x16iW\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a9\xE6W`\0\x80\x81\xFD[\x87\x01`?\x81\x01\x89\x13a9\xF8W`\0\x80\x81\xFD[a:\t\x89\x86\x83\x01Q`@\x84\x01a9RV[\x84RP\x91\x83\x01\x91\x83\x01a9\xC6V[`\0\x82`\x1F\x83\x01\x12a:(W`\0\x80\xFD[a*T\x83\x83Q` \x85\x01a9RV[`\0\x82`\x1F\x83\x01\x12a:HW`\0\x80\xFD[\x81Q` a:Xa2\xDC\x83a9.V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a:wW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a\x16iW\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a:\x9BW`\0\x80\x81\xFD[a:\xA9\x89\x86\x83\x8B\x01\x01a:\x17V[\x84RP\x91\x83\x01\x91\x83\x01a:{V[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a:\xCDW`\0\x80\xFD[\x84Q\x93P` \x80\x86\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a:\xEDW`\0\x80\xFD[\x81\x88\x01\x91P\x88`\x1F\x83\x01\x12a;\x01W`\0\x80\xFD[\x81Qa;\x0Fa2\xDC\x82a9.V[\x81\x81R`\x05\x91\x90\x91\x1B\x83\x01\x84\x01\x90\x84\x81\x01\x90\x8B\x83\x11\x15a;.W`\0\x80\xFD[\x93\x85\x01\x93[\x82\x85\x10\x15a;LW\x84Q\x82R\x93\x85\x01\x93\x90\x85\x01\x90a;3V[`@\x8B\x01Q\x90\x98P\x94PPP\x80\x83\x11\x15a;eW`\0\x80\xFD[a;q\x89\x84\x8A\x01a9\x82V[\x94P``\x88\x01Q\x92P\x80\x83\x11\x15a;\x87W`\0\x80\xFD[PPa;\x95\x87\x82\x88\x01a:7V[\x91PP\x92\x95\x91\x94P\x92PV[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0\x82Qa;\xC9\x81\x84` \x87\x01a5XV[\x91\x90\x91\x01\x92\x91PPV[`@\x81R`\0a;\xE6`@\x83\x01\x85a5\x84V[\x90P\x82` \x83\x01R\x93\x92PPPV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\0\x19\x82\x14\x15a<\x1FWa<\x1Fa;\xF5V[P`\x01\x01\x90V[\x805a9\x0E\x81a3\xF3V[`\0\x80`\0``\x84\x86\x03\x12\x15a<FW`\0\x80\xFD[\x835g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a<]W`\0\x80\xFD[a<i\x86\x82\x87\x01a2\xBDV[\x93PP` \x84\x015a<z\x81a3\xF3V[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[\x81\x83\x827`\0\x91\x01\x90\x81R\x91\x90PV[``\x81R`\0a<\xAF``\x83\x01\x88\x8Aa8DV[\x82\x81\x03` \x84\x01Ra<\xC2\x81\x87\x89a8DV[\x90P\x82\x81\x03`@\x84\x01Ra<\xD7\x81\x85\x87a8DV[\x99\x98PPPPPPPPPV[`\x80\x81R`\0a<\xF8`\x80\x83\x01\x89\x8Ba8DV[\x82\x81\x03` \x84\x01Ra=\x0B\x81\x88\x8Aa8DV[\x90P\x82\x81\x03`@\x84\x01Ra= \x81\x86\x88a8DV[\x91PP\x82``\x83\x01R\x98\x97PPPPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a=HW`\0\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a=_W`\0\x80\xFD[a=k\x85\x82\x86\x01a2\xBDV[\x95` \x94\x90\x94\x015\x94PPPPV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a=\xA2W`\0\x80\xFD[PQ\x91\x90PV[`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01R`\0a3H`@\x83\x01\x84a5\x84V[`\0\x80`@\x83\x85\x03\x12\x15a=\xDEW`\0\x80\xFD[a=\xE7\x83a8\xFEV[\x91P` \x83\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a>\x03W`\0\x80\xFD[a8\xDB\x85\x82\x86\x01a:\x17V[`\0\x80`\0\x80`\0\x80`\0\x80a\x01\0\x89\x8B\x03\x12\x15a>,W`\0\x80\xFD[\x885g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a>DW`\0\x80\xFD[a>P\x8C\x83\x8D\x01a2\xBDV[\x99P` \x8B\x015\x91P\x80\x82\x11\x15a>fW`\0\x80\xFD[a>r\x8C\x83\x8D\x01a2\xBDV[\x98Pa>\x80`@\x8C\x01a<&V[\x97P``\x8B\x015\x96P`\x80\x8B\x015\x91P\x80\x82\x11\x15a>\x9DW`\0\x80\xFD[Pa>\xAA\x8B\x82\x8C\x01a2\xBDV[\x98\x9B\x97\x9AP\x95\x98\x94\x97\x96`\xA0\x86\x015\x96P`\xC0\x86\x015\x95`\xE0\x015\x94P\x92PPPV[`\xC0\x81R`\0a>\xE0`\xC0\x83\x01\x89a5\x84V[\x82\x81\x03` \x84\x01Ra>\xF2\x81\x89a5\x84V[\x90P\x82\x81\x03`@\x84\x01Ra?\x06\x81\x88a5\x84V[``\x84\x01\x96\x90\x96RPP`\x80\x81\x01\x92\x90\x92R`\xA0\x90\x91\x01R\x93\x92PPPV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a?<W`\0\x80\xFD[\x83\x01\x805\x91Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a?WW`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a0\x04W`\0\x80\xFD[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15a?\x85W`\0\x80\xFD[\x865g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a?\x9DW`\0\x80\xFD[a?\xA9\x8A\x83\x8B\x01a2\xBDV[\x97P` \x89\x015\x91P\x80\x82\x11\x15a?\xBFW`\0\x80\xFD[Pa?\xCC\x89\x82\x8A\x01a2\xBDV[\x95PP`@\x87\x015`\xFF\x81\x16\x81\x14a?\xE3W`\0\x80\xFD[\x93P``\x87\x015\x92P`\x80\x87\x015a?\xFA\x81a3\xF3V[\x80\x92PP`\xA0\x87\x015\x90P\x92\x95P\x92\x95P\x92\x95V[`\xA0\x81R`\0a@\"`\xA0\x83\x01\x88a5\x84V[\x82\x81\x03` \x84\x01Ra@4\x81\x88a5\x84V[`\xFF\x96\x90\x96\x16`@\x84\x01RPP``\x81\x01\x92\x90\x92R`\x80\x90\x91\x01R\x92\x91PPV[`\0` \x82\x84\x03\x12\x15a@gW`\0\x80\xFD[\x81Qa*T\x81a3\xF3V[`@\x81R`\0a@\x85`@\x83\x01\x85a5\x84V[\x90P`\x01`\x01`\xA0\x1B\x03\x83\x16` \x83\x01R\x93\x92PPPV[`\x01\x81\x81\x1C\x90\x82\x16\x80a@\xB1W`\x7F\x82\x16\x91P[` \x82\x10\x81\x14\x15a@\xD2WcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[P\x91\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a@\xEDW`\0\x80\xFD[\x835a@\xF8\x81a3\xF3V[\x92P` \x84\x015aA\x08\x81a3\xF3V[\x91P`@\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aA$W`\0\x80\xFD[aA0\x86\x82\x87\x01a2\xBDV[\x91PP\x92P\x92P\x92V[`\xA0\x81R`\0aAN`\xA0\x83\x01\x8B\x8Da8DV[\x82\x81\x03` \x84\x01RaAa\x81\x8A\x8Ca8DV[\x90P\x82\x81\x03`@\x84\x01RaAv\x81\x88\x8Aa8DV[\x90P\x82\x81\x03``\x84\x01RaA\x8B\x81\x86\x88a8DV[\x91PP\x82`\x80\x83\x01R\x9A\x99PPPPPPPPPPV[`\0\x82aA\xBFWcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15aA\xDDW`\0\x80\xFD[\x865g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15aA\xF5W`\0\x80\xFD[aB\x01\x8A\x83\x8B\x01a2\xBDV[\x97P` \x89\x015\x91P\x80\x82\x11\x15aB\x17W`\0\x80\xFD[PaB$\x89\x82\x8A\x01a2\xBDV[\x95PP`@\x87\x015aB5\x81a3\xF3V[\x95\x98\x94\x97P\x94\x95``\x81\x015\x95P`\x80\x81\x015\x94`\xA0\x90\x91\x015\x93P\x91PPV[`\x80\x81R`\0aBi`\x80\x83\x01\x87a5\x84V[\x82\x81\x03` \x84\x01RaB{\x81\x87a5\x84V[`@\x84\x01\x95\x90\x95RPP``\x01R\x92\x91PPV[`\0\x82\x19\x82\x11\x15aB\xA2WaB\xA2a;\xF5V[P\x01\x90V[`\0a\x01\0\x8A\x83R\x89` \x84\x01R\x80`@\x84\x01RaB\xC7\x81\x84\x01\x8Aa5\x84V[\x90P\x82\x81\x03``\x84\x01RaB\xDB\x81\x89a5\x84V[\x90P`\x01`\x01`\xA0\x1B\x03\x87\x16`\x80\x84\x01R\x85`\xA0\x84\x01R\x82\x81\x03`\xC0\x84\x01RaC\x04\x81\x86a5\x84V[\x91PP\x82`\xE0\x83\x01R\x99\x98PPPPPPPPPV[\x82\x81R`\0\x82QaC2\x81` \x85\x01` \x87\x01a5XV[\x91\x90\x91\x01` \x01\x93\x92PPPV[\x86\x81R\x85` \x82\x01R`\xC0`@\x82\x01R`\0aC_`\xC0\x83\x01\x87a5\x84V[\x82\x81\x03``\x84\x01RaCq\x81\x87a5\x84V[`\x01`\x01`\xA0\x1B\x03\x95\x90\x95\x16`\x80\x84\x01RPP`\xA0\x01R\x94\x93PPPPV[\x83\x81R``` \x82\x01R`\0aC\xA9``\x83\x01\x85a5\x84V[\x90P\x82`@\x83\x01R\x94\x93PPPPV\xFE`\x80`@R`\x01`\0U4\x80\x15a\0\x15W`\0\x80\xFD[Pa\x03$\x80a\0%`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x005W`\x005`\xE0\x1C\x80b\xF5]\x9D\x14a\0:W\x80c\x1C\xFFy\xCD\x14a\0OW[`\0\x80\xFD[a\0Ma\0H6`\x04a\x01\xDAV[a\0yV[\0[a\0ba\0]6`\x04a\x01\xFCV[a\0\xBBV[`@Qa\0p\x92\x91\x90a\x02\x7FV[`@Q\x80\x91\x03\x90\xF3[`\x02`\0T\x14\x15a\0\x9DW`@Qc\xCA\xA3\x0FU`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02`\0Us\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\xFF[`\0```\x02`\0T\x14\x15a\0\xE3W`@Qc\xCA\xA3\x0FU`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\x02`\0Us\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16;a\x016W`@Q\x7Fo|C\xF1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84\x84`@Qa\x01]\x92\x91\x90a\x02\xDEV[`\0`@Q\x80\x83\x03\x81`\0\x86Z\xF1\x91PP=\x80`\0\x81\x14a\x01\x9AW`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=`\0` \x84\x01>a\x01\x9FV[``\x91P[P`\x01`\0U\x90\x96\x90\x95P\x93PPPPV[\x805s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x01\xD5W`\0\x80\xFD[\x91\x90PV[`\0` \x82\x84\x03\x12\x15a\x01\xECW`\0\x80\xFD[a\x01\xF5\x82a\x01\xB1V[\x93\x92PPPV[`\0\x80`\0`@\x84\x86\x03\x12\x15a\x02\x11W`\0\x80\xFD[a\x02\x1A\x84a\x01\xB1V[\x92P` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x11\x15a\x027W`\0\x80\xFD[\x81\x86\x01\x91P\x86`\x1F\x83\x01\x12a\x02KW`\0\x80\xFD[\x815\x81\x81\x11\x15a\x02ZW`\0\x80\xFD[\x87` \x82\x85\x01\x01\x11\x15a\x02lW`\0\x80\xFD[` \x83\x01\x94P\x80\x93PPPP\x92P\x92P\x92V[\x82\x15\x15\x81R`\0` `@\x81\x84\x01R\x83Q\x80`@\x85\x01R`\0[\x81\x81\x10\x15a\x02\xB5W\x85\x81\x01\x83\x01Q\x85\x82\x01``\x01R\x82\x01a\x02\x99V[\x81\x81\x11\x15a\x02\xC7W`\0``\x83\x87\x01\x01R[P`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01``\x01\x94\x93PPPPV[\x81\x83\x827`\0\x91\x01\x90\x81R\x91\x90PV\xFE\xA2dipfsX\"\x12 2\xCB^th\x16\xB7\xFA\xC9R\x05\xC0h\xB3\r\xA3{\xD4\x01\x19\xA5re\xBE3\x1C\x16,\xAEtq$dsolcC\0\x08\t\x003\xA2dipfsX\"\x12 \x82`\x16\xCA\x85\x85\xB5V\x07\x9C\x9B]I\xCA\xA9\xC1,w.\xAD:\x9D\xE6CD\xC0\xCA\xD8\xAB\x01*wdsolcC\0\x08\t\x003";
    /// The deployed bytecode of the contract.
    pub static SCALARGATEWAY_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct ScalarGateway<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for ScalarGateway<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for ScalarGateway<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for ScalarGateway<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for ScalarGateway<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(ScalarGateway))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> ScalarGateway<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    SCALARGATEWAY_ABI.clone(),
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
                SCALARGATEWAY_ABI.clone(),
                SCALARGATEWAY_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `adminEpoch` (0x364940d8) function
        pub fn admin_epoch(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([54, 73, 64, 216], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `adminThreshold` (0x88b30587) function
        pub fn admin_threshold(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([136, 179, 5, 135], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `admins` (0x14bfd6d0) function
        pub fn admins(
            &self,
            p0: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::Address>,
        > {
            self.0
                .method_hash([20, 191, 214, 208], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `allTokensFrozen` (0xaa1e1f0a) function
        pub fn all_tokens_frozen(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([170, 30, 31, 10], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `approveContractCall` (0xfbe0a31b) function
        pub fn approve_contract_call(
            &self,
            params: ::ethers::core::types::Bytes,
            command_id: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([251, 224, 163, 27], (params, command_id))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `approveContractCallWithMint` (0x585a9fd4) function
        pub fn approve_contract_call_with_mint(
            &self,
            params: ::ethers::core::types::Bytes,
            command_id: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([88, 90, 159, 212], (params, command_id))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `authModule` (0x64940c56) function
        pub fn auth_module(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([100, 148, 12, 86], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `burnToken` (0x4656ae2e) function
        pub fn burn_token(
            &self,
            params: ::ethers::core::types::Bytes,
            p1: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([70, 86, 174, 46], (params, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `callContract` (0x1c92115f) function
        pub fn call_contract(
            &self,
            destination_chain: ::std::string::String,
            destination_contract_address: ::std::string::String,
            payload: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [28, 146, 17, 95],
                    (destination_chain, destination_contract_address, payload),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `callContractWithToken` (0xb5417084) function
        pub fn call_contract_with_token(
            &self,
            destination_chain: ::std::string::String,
            destination_contract_address: ::std::string::String,
            payload: ::ethers::core::types::Bytes,
            symbol: ::std::string::String,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [181, 65, 112, 132],
                    (
                        destination_chain,
                        destination_contract_address,
                        payload,
                        symbol,
                        amount,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `contractId` (0x8291286c) function
        pub fn contract_id(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([130, 145, 40, 108], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deployToken` (0x886a625d) function
        pub fn deploy_token(
            &self,
            params: ::ethers::core::types::Bytes,
            p1: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([136, 106, 98, 93], (params, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `execute` (0x09c5eabe) function
        pub fn execute(
            &self,
            input: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([9, 197, 234, 190], input)
                .expect("method not found (this should never happen)")
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
        ///Calls the contract's `governance` (0x5aa6e675) function
        pub fn governance(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([90, 166, 230, 117], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `implementation` (0x5c60da1b) function
        pub fn implementation(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([92, 96, 218, 27], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isCommandExecuted` (0xd26ff210) function
        pub fn is_command_executed(
            &self,
            command_id: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([210, 111, 242, 16], command_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isContractCallAndMintApproved` (0xbc00c216) function
        pub fn is_contract_call_and_mint_approved(
            &self,
            command_id: [u8; 32],
            source_chain: ::std::string::String,
            source_address: ::std::string::String,
            contract_address: ::ethers::core::types::Address,
            payload_hash: [u8; 32],
            symbol: ::std::string::String,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash(
                    [188, 0, 194, 22],
                    (
                        command_id,
                        source_chain,
                        source_address,
                        contract_address,
                        payload_hash,
                        symbol,
                        amount,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isContractCallApproved` (0xf6a5f9f5) function
        pub fn is_contract_call_approved(
            &self,
            command_id: [u8; 32],
            source_chain: ::std::string::String,
            source_address: ::std::string::String,
            contract_address: ::ethers::core::types::Address,
            payload_hash: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash(
                    [246, 165, 249, 245],
                    (
                        command_id,
                        source_chain,
                        source_address,
                        contract_address,
                        payload_hash,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `mintLimiter` (0xc82fe87a) function
        pub fn mint_limiter(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([200, 47, 232, 122], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `mintToken` (0x146e2d78) function
        pub fn mint_token(
            &self,
            params: ::ethers::core::types::Bytes,
            p1: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([20, 110, 45, 120], (params, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `resetOperatorship` (0x00f7d49f) function
        pub fn reset_operatorship(
            &self,
            params: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([0, 247, 212, 159], params)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `sendToken` (0x26ef699d) function
        pub fn send_token(
            &self,
            destination_chain: ::std::string::String,
            destination_address: ::std::string::String,
            symbol: ::std::string::String,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [38, 239, 105, 157],
                    (destination_chain, destination_address, symbol, amount),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setTokenMintLimits` (0x67ace8eb) function
        pub fn set_token_mint_limits(
            &self,
            symbols: ::std::vec::Vec<::std::string::String>,
            limits: ::std::vec::Vec<::ethers::core::types::U256>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([103, 172, 232, 235], (symbols, limits))
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
        ///Calls the contract's `tokenAddresses` (0x935b13f6) function
        pub fn token_addresses(
            &self,
            symbol: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([147, 91, 19, 246], symbol)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `tokenDeployer` (0x2a2dae0a) function
        pub fn token_deployer(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([42, 45, 174, 10], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `tokenFrozen` (0x7b1b769e) function
        pub fn token_frozen(
            &self,
            p0: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([123, 27, 118, 158], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `tokenMintAmount` (0xcec7b359) function
        pub fn token_mint_amount(
            &self,
            symbol: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([206, 199, 179, 89], symbol)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `tokenMintLimit` (0x269eb65e) function
        pub fn token_mint_limit(
            &self,
            symbol: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([38, 158, 182, 94], symbol)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferGovernance` (0xd38bfff4) function
        pub fn transfer_governance(
            &self,
            new_governance: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([211, 139, 255, 244], new_governance)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferMintLimiter` (0x41d8f26b) function
        pub fn transfer_mint_limiter(
            &self,
            new_mint_limiter: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([65, 216, 242, 107], new_mint_limiter)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferOperatorship` (0x97b87ba6) function
        pub fn transfer_operatorship(
            &self,
            new_operators_data: ::ethers::core::types::Bytes,
            p1: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([151, 184, 123, 166], (new_operators_data, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `upgrade` (0xa3499c73) function
        pub fn upgrade(
            &self,
            new_implementation: ::ethers::core::types::Address,
            new_implementation_code_hash: [u8; 32],
            setup_params: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [163, 73, 156, 115],
                    (new_implementation, new_implementation_code_hash, setup_params),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `validateContractCall` (0x5f6970c3) function
        pub fn validate_contract_call(
            &self,
            command_id: [u8; 32],
            source_chain: ::std::string::String,
            source_address: ::std::string::String,
            payload_hash: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash(
                    [95, 105, 112, 195],
                    (command_id, source_chain, source_address, payload_hash),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `validateContractCallAndMint` (0x1876eed9) function
        pub fn validate_contract_call_and_mint(
            &self,
            command_id: [u8; 32],
            source_chain: ::std::string::String,
            source_address: ::std::string::String,
            payload_hash: [u8; 32],
            symbol: ::std::string::String,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash(
                    [24, 118, 238, 217],
                    (
                        command_id,
                        source_chain,
                        source_address,
                        payload_hash,
                        symbol,
                        amount,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `ContractCall` event
        pub fn contract_call_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ContractCallFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ContractCallApproved` event
        pub fn contract_call_approved_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ContractCallApprovedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ContractCallApprovedWithMint` event
        pub fn contract_call_approved_with_mint_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ContractCallApprovedWithMintFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `ContractCallWithToken` event
        pub fn contract_call_with_token_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ContractCallWithTokenFilter,
        > {
            self.0.event()
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
        ///Gets the contract's `GovernanceTransferred` event
        pub fn governance_transferred_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            GovernanceTransferredFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `MintLimiterTransferred` event
        pub fn mint_limiter_transferred_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            MintLimiterTransferredFilter,
        > {
            self.0.event()
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
        ///Gets the contract's `TokenDeployed` event
        pub fn token_deployed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            TokenDeployedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `TokenMintLimitUpdated` event
        pub fn token_mint_limit_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            TokenMintLimitUpdatedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `TokenSent` event
        pub fn token_sent_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            TokenSentFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Upgraded` event
        pub fn upgraded_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            UpgradedFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            ScalarGatewayEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for ScalarGateway<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `BurnFailed` with signature `BurnFailed(string)` and selector `0xe217b0ad`
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
    #[etherror(name = "BurnFailed", abi = "BurnFailed(string)")]
    pub struct BurnFailed {
        pub symbol: ::std::string::String,
    }
    ///Custom Error type `ExceedMintLimit` with signature `ExceedMintLimit(string)` and selector `0x037f60e5`
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
    #[etherror(name = "ExceedMintLimit", abi = "ExceedMintLimit(string)")]
    pub struct ExceedMintLimit {
        pub symbol: ::std::string::String,
    }
    ///Custom Error type `InvalidAmount` with signature `InvalidAmount()` and selector `0x2c5211c6`
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
    #[etherror(name = "InvalidAmount", abi = "InvalidAmount()")]
    pub struct InvalidAmount;
    ///Custom Error type `InvalidAuthModule` with signature `InvalidAuthModule()` and selector `0x735326ab`
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
    #[etherror(name = "InvalidAuthModule", abi = "InvalidAuthModule()")]
    pub struct InvalidAuthModule;
    ///Custom Error type `InvalidChainId` with signature `InvalidChainId()` and selector `0x7a47c9a2`
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
    #[etherror(name = "InvalidChainId", abi = "InvalidChainId()")]
    pub struct InvalidChainId;
    ///Custom Error type `InvalidCodeHash` with signature `InvalidCodeHash()` and selector `0x8f84fb24`
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
    #[etherror(name = "InvalidCodeHash", abi = "InvalidCodeHash()")]
    pub struct InvalidCodeHash;
    ///Custom Error type `InvalidCommands` with signature `InvalidCommands()` and selector `0xca9a28f5`
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
    #[etherror(name = "InvalidCommands", abi = "InvalidCommands()")]
    pub struct InvalidCommands;
    ///Custom Error type `InvalidGovernance` with signature `InvalidGovernance()` and selector `0x0063186c`
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
    #[etherror(name = "InvalidGovernance", abi = "InvalidGovernance()")]
    pub struct InvalidGovernance;
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
    ///Custom Error type `InvalidMintLimiter` with signature `InvalidMintLimiter()` and selector `0xd79d772c`
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
    #[etherror(name = "InvalidMintLimiter", abi = "InvalidMintLimiter()")]
    pub struct InvalidMintLimiter;
    ///Custom Error type `InvalidSetMintLimitsParams` with signature `InvalidSetMintLimitsParams()` and selector `0x14a2275f`
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
    #[etherror(
        name = "InvalidSetMintLimitsParams",
        abi = "InvalidSetMintLimitsParams()"
    )]
    pub struct InvalidSetMintLimitsParams;
    ///Custom Error type `InvalidTokenDeployer` with signature `InvalidTokenDeployer()` and selector `0x6426d5f8`
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
    #[etherror(name = "InvalidTokenDeployer", abi = "InvalidTokenDeployer()")]
    pub struct InvalidTokenDeployer;
    ///Custom Error type `MintFailed` with signature `MintFailed(string)` and selector `0x90c52ed7`
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
    #[etherror(name = "MintFailed", abi = "MintFailed(string)")]
    pub struct MintFailed {
        pub symbol: ::std::string::String,
    }
    ///Custom Error type `NotGovernance` with signature `NotGovernance()` and selector `0xb56f932c`
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
    #[etherror(name = "NotGovernance", abi = "NotGovernance()")]
    pub struct NotGovernance;
    ///Custom Error type `NotMintLimiter` with signature `NotMintLimiter()` and selector `0x4475507a`
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
    #[etherror(name = "NotMintLimiter", abi = "NotMintLimiter()")]
    pub struct NotMintLimiter;
    ///Custom Error type `NotProxy` with signature `NotProxy()` and selector `0xbf10dd3a`
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
    #[etherror(name = "NotProxy", abi = "NotProxy()")]
    pub struct NotProxy;
    ///Custom Error type `NotSelf` with signature `NotSelf()` and selector `0x29c3b7ee`
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
    #[etherror(name = "NotSelf", abi = "NotSelf()")]
    pub struct NotSelf;
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
    ///Custom Error type `TokenAlreadyExists` with signature `TokenAlreadyExists(string)` and selector `0xaa7e8b32`
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
    #[etherror(name = "TokenAlreadyExists", abi = "TokenAlreadyExists(string)")]
    pub struct TokenAlreadyExists {
        pub symbol: ::std::string::String,
    }
    ///Custom Error type `TokenContractDoesNotExist` with signature `TokenContractDoesNotExist(address)` and selector `0xc5ccddde`
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
    #[etherror(
        name = "TokenContractDoesNotExist",
        abi = "TokenContractDoesNotExist(address)"
    )]
    pub struct TokenContractDoesNotExist {
        pub token: ::ethers::core::types::Address,
    }
    ///Custom Error type `TokenDeployFailed` with signature `TokenDeployFailed(string)` and selector `0x86d52743`
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
    #[etherror(name = "TokenDeployFailed", abi = "TokenDeployFailed(string)")]
    pub struct TokenDeployFailed {
        pub symbol: ::std::string::String,
    }
    ///Custom Error type `TokenDoesNotExist` with signature `TokenDoesNotExist(string)` and selector `0x72ba137e`
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
    #[etherror(name = "TokenDoesNotExist", abi = "TokenDoesNotExist(string)")]
    pub struct TokenDoesNotExist {
        pub symbol: ::std::string::String,
    }
    ///Custom Error type `TokenTransferFailed` with signature `TokenTransferFailed()` and selector `0x045c4b02`
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
    #[etherror(name = "TokenTransferFailed", abi = "TokenTransferFailed()")]
    pub struct TokenTransferFailed;
    ///Container type for all of the contract's custom errors
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum ScalarGatewayErrors {
        BurnFailed(BurnFailed),
        ExceedMintLimit(ExceedMintLimit),
        InvalidAmount(InvalidAmount),
        InvalidAuthModule(InvalidAuthModule),
        InvalidChainId(InvalidChainId),
        InvalidCodeHash(InvalidCodeHash),
        InvalidCommands(InvalidCommands),
        InvalidGovernance(InvalidGovernance),
        InvalidImplementation(InvalidImplementation),
        InvalidMintLimiter(InvalidMintLimiter),
        InvalidSetMintLimitsParams(InvalidSetMintLimitsParams),
        InvalidTokenDeployer(InvalidTokenDeployer),
        MintFailed(MintFailed),
        NotGovernance(NotGovernance),
        NotMintLimiter(NotMintLimiter),
        NotProxy(NotProxy),
        NotSelf(NotSelf),
        SetupFailed(SetupFailed),
        TokenAlreadyExists(TokenAlreadyExists),
        TokenContractDoesNotExist(TokenContractDoesNotExist),
        TokenDeployFailed(TokenDeployFailed),
        TokenDoesNotExist(TokenDoesNotExist),
        TokenTransferFailed(TokenTransferFailed),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for ScalarGatewayErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <BurnFailed as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BurnFailed(decoded));
            }
            if let Ok(decoded) = <ExceedMintLimit as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExceedMintLimit(decoded));
            }
            if let Ok(decoded) = <InvalidAmount as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidAmount(decoded));
            }
            if let Ok(decoded) = <InvalidAuthModule as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidAuthModule(decoded));
            }
            if let Ok(decoded) = <InvalidChainId as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidChainId(decoded));
            }
            if let Ok(decoded) = <InvalidCodeHash as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidCodeHash(decoded));
            }
            if let Ok(decoded) = <InvalidCommands as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidCommands(decoded));
            }
            if let Ok(decoded) = <InvalidGovernance as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidGovernance(decoded));
            }
            if let Ok(decoded) = <InvalidImplementation as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidImplementation(decoded));
            }
            if let Ok(decoded) = <InvalidMintLimiter as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidMintLimiter(decoded));
            }
            if let Ok(decoded) = <InvalidSetMintLimitsParams as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidSetMintLimitsParams(decoded));
            }
            if let Ok(decoded) = <InvalidTokenDeployer as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::InvalidTokenDeployer(decoded));
            }
            if let Ok(decoded) = <MintFailed as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MintFailed(decoded));
            }
            if let Ok(decoded) = <NotGovernance as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::NotGovernance(decoded));
            }
            if let Ok(decoded) = <NotMintLimiter as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::NotMintLimiter(decoded));
            }
            if let Ok(decoded) = <NotProxy as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::NotProxy(decoded));
            }
            if let Ok(decoded) = <NotSelf as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::NotSelf(decoded));
            }
            if let Ok(decoded) = <SetupFailed as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetupFailed(decoded));
            }
            if let Ok(decoded) = <TokenAlreadyExists as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TokenAlreadyExists(decoded));
            }
            if let Ok(decoded) = <TokenContractDoesNotExist as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TokenContractDoesNotExist(decoded));
            }
            if let Ok(decoded) = <TokenDeployFailed as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TokenDeployFailed(decoded));
            }
            if let Ok(decoded) = <TokenDoesNotExist as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TokenDoesNotExist(decoded));
            }
            if let Ok(decoded) = <TokenTransferFailed as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TokenTransferFailed(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ScalarGatewayErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::BurnFailed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ExceedMintLimit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidAmount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidAuthModule(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidChainId(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidCodeHash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidCommands(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidGovernance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidImplementation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidMintLimiter(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidSetMintLimitsParams(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidTokenDeployer(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MintFailed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NotGovernance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NotMintLimiter(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NotProxy(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NotSelf(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetupFailed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TokenAlreadyExists(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TokenContractDoesNotExist(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TokenDeployFailed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TokenDoesNotExist(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TokenTransferFailed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for ScalarGatewayErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <BurnFailed as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <ExceedMintLimit as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidAmount as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidAuthModule as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidChainId as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidCodeHash as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidCommands as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidGovernance as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidImplementation as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidMintLimiter as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidSetMintLimitsParams as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidTokenDeployer as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <MintFailed as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <NotGovernance as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NotMintLimiter as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NotProxy as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <NotSelf as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <SetupFailed as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <TokenAlreadyExists as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <TokenContractDoesNotExist as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <TokenDeployFailed as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <TokenDoesNotExist as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <TokenTransferFailed as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for ScalarGatewayErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::BurnFailed(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExceedMintLimit(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidAmount(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidAuthModule(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidChainId(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidCodeHash(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidCommands(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidGovernance(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidImplementation(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidMintLimiter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidSetMintLimitsParams(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidTokenDeployer(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MintFailed(element) => ::core::fmt::Display::fmt(element, f),
                Self::NotGovernance(element) => ::core::fmt::Display::fmt(element, f),
                Self::NotMintLimiter(element) => ::core::fmt::Display::fmt(element, f),
                Self::NotProxy(element) => ::core::fmt::Display::fmt(element, f),
                Self::NotSelf(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetupFailed(element) => ::core::fmt::Display::fmt(element, f),
                Self::TokenAlreadyExists(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TokenContractDoesNotExist(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TokenDeployFailed(element) => ::core::fmt::Display::fmt(element, f),
                Self::TokenDoesNotExist(element) => ::core::fmt::Display::fmt(element, f),
                Self::TokenTransferFailed(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for ScalarGatewayErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<BurnFailed> for ScalarGatewayErrors {
        fn from(value: BurnFailed) -> Self {
            Self::BurnFailed(value)
        }
    }
    impl ::core::convert::From<ExceedMintLimit> for ScalarGatewayErrors {
        fn from(value: ExceedMintLimit) -> Self {
            Self::ExceedMintLimit(value)
        }
    }
    impl ::core::convert::From<InvalidAmount> for ScalarGatewayErrors {
        fn from(value: InvalidAmount) -> Self {
            Self::InvalidAmount(value)
        }
    }
    impl ::core::convert::From<InvalidAuthModule> for ScalarGatewayErrors {
        fn from(value: InvalidAuthModule) -> Self {
            Self::InvalidAuthModule(value)
        }
    }
    impl ::core::convert::From<InvalidChainId> for ScalarGatewayErrors {
        fn from(value: InvalidChainId) -> Self {
            Self::InvalidChainId(value)
        }
    }
    impl ::core::convert::From<InvalidCodeHash> for ScalarGatewayErrors {
        fn from(value: InvalidCodeHash) -> Self {
            Self::InvalidCodeHash(value)
        }
    }
    impl ::core::convert::From<InvalidCommands> for ScalarGatewayErrors {
        fn from(value: InvalidCommands) -> Self {
            Self::InvalidCommands(value)
        }
    }
    impl ::core::convert::From<InvalidGovernance> for ScalarGatewayErrors {
        fn from(value: InvalidGovernance) -> Self {
            Self::InvalidGovernance(value)
        }
    }
    impl ::core::convert::From<InvalidImplementation> for ScalarGatewayErrors {
        fn from(value: InvalidImplementation) -> Self {
            Self::InvalidImplementation(value)
        }
    }
    impl ::core::convert::From<InvalidMintLimiter> for ScalarGatewayErrors {
        fn from(value: InvalidMintLimiter) -> Self {
            Self::InvalidMintLimiter(value)
        }
    }
    impl ::core::convert::From<InvalidSetMintLimitsParams> for ScalarGatewayErrors {
        fn from(value: InvalidSetMintLimitsParams) -> Self {
            Self::InvalidSetMintLimitsParams(value)
        }
    }
    impl ::core::convert::From<InvalidTokenDeployer> for ScalarGatewayErrors {
        fn from(value: InvalidTokenDeployer) -> Self {
            Self::InvalidTokenDeployer(value)
        }
    }
    impl ::core::convert::From<MintFailed> for ScalarGatewayErrors {
        fn from(value: MintFailed) -> Self {
            Self::MintFailed(value)
        }
    }
    impl ::core::convert::From<NotGovernance> for ScalarGatewayErrors {
        fn from(value: NotGovernance) -> Self {
            Self::NotGovernance(value)
        }
    }
    impl ::core::convert::From<NotMintLimiter> for ScalarGatewayErrors {
        fn from(value: NotMintLimiter) -> Self {
            Self::NotMintLimiter(value)
        }
    }
    impl ::core::convert::From<NotProxy> for ScalarGatewayErrors {
        fn from(value: NotProxy) -> Self {
            Self::NotProxy(value)
        }
    }
    impl ::core::convert::From<NotSelf> for ScalarGatewayErrors {
        fn from(value: NotSelf) -> Self {
            Self::NotSelf(value)
        }
    }
    impl ::core::convert::From<SetupFailed> for ScalarGatewayErrors {
        fn from(value: SetupFailed) -> Self {
            Self::SetupFailed(value)
        }
    }
    impl ::core::convert::From<TokenAlreadyExists> for ScalarGatewayErrors {
        fn from(value: TokenAlreadyExists) -> Self {
            Self::TokenAlreadyExists(value)
        }
    }
    impl ::core::convert::From<TokenContractDoesNotExist> for ScalarGatewayErrors {
        fn from(value: TokenContractDoesNotExist) -> Self {
            Self::TokenContractDoesNotExist(value)
        }
    }
    impl ::core::convert::From<TokenDeployFailed> for ScalarGatewayErrors {
        fn from(value: TokenDeployFailed) -> Self {
            Self::TokenDeployFailed(value)
        }
    }
    impl ::core::convert::From<TokenDoesNotExist> for ScalarGatewayErrors {
        fn from(value: TokenDoesNotExist) -> Self {
            Self::TokenDoesNotExist(value)
        }
    }
    impl ::core::convert::From<TokenTransferFailed> for ScalarGatewayErrors {
        fn from(value: TokenTransferFailed) -> Self {
            Self::TokenTransferFailed(value)
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
        name = "ContractCall",
        abi = "ContractCall(address,string,string,bytes32,bytes)"
    )]
    pub struct ContractCallFilter {
        #[ethevent(indexed)]
        pub sender: ::ethers::core::types::Address,
        pub destination_chain: ::std::string::String,
        pub destination_contract_address: ::std::string::String,
        #[ethevent(indexed)]
        pub payload_hash: [u8; 32],
        pub payload: ::ethers::core::types::Bytes,
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
        name = "ContractCallApproved",
        abi = "ContractCallApproved(bytes32,string,string,address,bytes32,bytes32,uint256)"
    )]
    pub struct ContractCallApprovedFilter {
        #[ethevent(indexed)]
        pub command_id: [u8; 32],
        pub source_chain: ::std::string::String,
        pub source_address: ::std::string::String,
        #[ethevent(indexed)]
        pub contract_address: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub payload_hash: [u8; 32],
        pub source_tx_hash: [u8; 32],
        pub source_event_index: ::ethers::core::types::U256,
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
        name = "ContractCallApprovedWithMint",
        abi = "ContractCallApprovedWithMint(bytes32,string,string,address,bytes32,string,uint256,bytes32,uint256)"
    )]
    pub struct ContractCallApprovedWithMintFilter {
        #[ethevent(indexed)]
        pub command_id: [u8; 32],
        pub source_chain: ::std::string::String,
        pub source_address: ::std::string::String,
        #[ethevent(indexed)]
        pub contract_address: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub payload_hash: [u8; 32],
        pub symbol: ::std::string::String,
        pub amount: ::ethers::core::types::U256,
        pub source_tx_hash: [u8; 32],
        pub source_event_index: ::ethers::core::types::U256,
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
        name = "ContractCallWithToken",
        abi = "ContractCallWithToken(address,string,string,bytes32,bytes,string,uint256)"
    )]
    pub struct ContractCallWithTokenFilter {
        #[ethevent(indexed)]
        pub sender: ::ethers::core::types::Address,
        pub destination_chain: ::std::string::String,
        pub destination_contract_address: ::std::string::String,
        #[ethevent(indexed)]
        pub payload_hash: [u8; 32],
        pub payload: ::ethers::core::types::Bytes,
        pub symbol: ::std::string::String,
        pub amount: ::ethers::core::types::U256,
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
    #[ethevent(name = "Executed", abi = "Executed(bytes32)")]
    pub struct ExecutedFilter {
        #[ethevent(indexed)]
        pub command_id: [u8; 32],
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
        name = "GovernanceTransferred",
        abi = "GovernanceTransferred(address,address)"
    )]
    pub struct GovernanceTransferredFilter {
        #[ethevent(indexed)]
        pub previous_governance: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_governance: ::ethers::core::types::Address,
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
        name = "MintLimiterTransferred",
        abi = "MintLimiterTransferred(address,address)"
    )]
    pub struct MintLimiterTransferredFilter {
        #[ethevent(indexed)]
        pub previous_governance: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_governance: ::ethers::core::types::Address,
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
    #[ethevent(name = "OperatorshipTransferred", abi = "OperatorshipTransferred(bytes)")]
    pub struct OperatorshipTransferredFilter {
        pub new_operators_data: ::ethers::core::types::Bytes,
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
    #[ethevent(name = "TokenDeployed", abi = "TokenDeployed(string,address)")]
    pub struct TokenDeployedFilter {
        pub symbol: ::std::string::String,
        pub token_addresses: ::ethers::core::types::Address,
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
        name = "TokenMintLimitUpdated",
        abi = "TokenMintLimitUpdated(string,uint256)"
    )]
    pub struct TokenMintLimitUpdatedFilter {
        pub symbol: ::std::string::String,
        pub limit: ::ethers::core::types::U256,
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
        name = "TokenSent",
        abi = "TokenSent(address,string,string,string,uint256)"
    )]
    pub struct TokenSentFilter {
        #[ethevent(indexed)]
        pub sender: ::ethers::core::types::Address,
        pub destination_chain: ::std::string::String,
        pub destination_address: ::std::string::String,
        pub symbol: ::std::string::String,
        pub amount: ::ethers::core::types::U256,
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
    #[ethevent(name = "Upgraded", abi = "Upgraded(address)")]
    pub struct UpgradedFilter {
        #[ethevent(indexed)]
        pub implementation: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum ScalarGatewayEvents {
        ContractCallFilter(ContractCallFilter),
        ContractCallApprovedFilter(ContractCallApprovedFilter),
        ContractCallApprovedWithMintFilter(ContractCallApprovedWithMintFilter),
        ContractCallWithTokenFilter(ContractCallWithTokenFilter),
        ExecutedFilter(ExecutedFilter),
        GovernanceTransferredFilter(GovernanceTransferredFilter),
        MintLimiterTransferredFilter(MintLimiterTransferredFilter),
        OperatorshipTransferredFilter(OperatorshipTransferredFilter),
        TokenDeployedFilter(TokenDeployedFilter),
        TokenMintLimitUpdatedFilter(TokenMintLimitUpdatedFilter),
        TokenSentFilter(TokenSentFilter),
        UpgradedFilter(UpgradedFilter),
    }
    impl ::ethers::contract::EthLogDecode for ScalarGatewayEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = ContractCallFilter::decode_log(log) {
                return Ok(ScalarGatewayEvents::ContractCallFilter(decoded));
            }
            if let Ok(decoded) = ContractCallApprovedFilter::decode_log(log) {
                return Ok(ScalarGatewayEvents::ContractCallApprovedFilter(decoded));
            }
            if let Ok(decoded) = ContractCallApprovedWithMintFilter::decode_log(log) {
                return Ok(
                    ScalarGatewayEvents::ContractCallApprovedWithMintFilter(decoded),
                );
            }
            if let Ok(decoded) = ContractCallWithTokenFilter::decode_log(log) {
                return Ok(ScalarGatewayEvents::ContractCallWithTokenFilter(decoded));
            }
            if let Ok(decoded) = ExecutedFilter::decode_log(log) {
                return Ok(ScalarGatewayEvents::ExecutedFilter(decoded));
            }
            if let Ok(decoded) = GovernanceTransferredFilter::decode_log(log) {
                return Ok(ScalarGatewayEvents::GovernanceTransferredFilter(decoded));
            }
            if let Ok(decoded) = MintLimiterTransferredFilter::decode_log(log) {
                return Ok(ScalarGatewayEvents::MintLimiterTransferredFilter(decoded));
            }
            if let Ok(decoded) = OperatorshipTransferredFilter::decode_log(log) {
                return Ok(ScalarGatewayEvents::OperatorshipTransferredFilter(decoded));
            }
            if let Ok(decoded) = TokenDeployedFilter::decode_log(log) {
                return Ok(ScalarGatewayEvents::TokenDeployedFilter(decoded));
            }
            if let Ok(decoded) = TokenMintLimitUpdatedFilter::decode_log(log) {
                return Ok(ScalarGatewayEvents::TokenMintLimitUpdatedFilter(decoded));
            }
            if let Ok(decoded) = TokenSentFilter::decode_log(log) {
                return Ok(ScalarGatewayEvents::TokenSentFilter(decoded));
            }
            if let Ok(decoded) = UpgradedFilter::decode_log(log) {
                return Ok(ScalarGatewayEvents::UpgradedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for ScalarGatewayEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ContractCallFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ContractCallApprovedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ContractCallApprovedWithMintFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ContractCallWithTokenFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ExecutedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::GovernanceTransferredFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MintLimiterTransferredFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OperatorshipTransferredFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TokenDeployedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TokenMintLimitUpdatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TokenSentFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::UpgradedFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<ContractCallFilter> for ScalarGatewayEvents {
        fn from(value: ContractCallFilter) -> Self {
            Self::ContractCallFilter(value)
        }
    }
    impl ::core::convert::From<ContractCallApprovedFilter> for ScalarGatewayEvents {
        fn from(value: ContractCallApprovedFilter) -> Self {
            Self::ContractCallApprovedFilter(value)
        }
    }
    impl ::core::convert::From<ContractCallApprovedWithMintFilter>
    for ScalarGatewayEvents {
        fn from(value: ContractCallApprovedWithMintFilter) -> Self {
            Self::ContractCallApprovedWithMintFilter(value)
        }
    }
    impl ::core::convert::From<ContractCallWithTokenFilter> for ScalarGatewayEvents {
        fn from(value: ContractCallWithTokenFilter) -> Self {
            Self::ContractCallWithTokenFilter(value)
        }
    }
    impl ::core::convert::From<ExecutedFilter> for ScalarGatewayEvents {
        fn from(value: ExecutedFilter) -> Self {
            Self::ExecutedFilter(value)
        }
    }
    impl ::core::convert::From<GovernanceTransferredFilter> for ScalarGatewayEvents {
        fn from(value: GovernanceTransferredFilter) -> Self {
            Self::GovernanceTransferredFilter(value)
        }
    }
    impl ::core::convert::From<MintLimiterTransferredFilter> for ScalarGatewayEvents {
        fn from(value: MintLimiterTransferredFilter) -> Self {
            Self::MintLimiterTransferredFilter(value)
        }
    }
    impl ::core::convert::From<OperatorshipTransferredFilter> for ScalarGatewayEvents {
        fn from(value: OperatorshipTransferredFilter) -> Self {
            Self::OperatorshipTransferredFilter(value)
        }
    }
    impl ::core::convert::From<TokenDeployedFilter> for ScalarGatewayEvents {
        fn from(value: TokenDeployedFilter) -> Self {
            Self::TokenDeployedFilter(value)
        }
    }
    impl ::core::convert::From<TokenMintLimitUpdatedFilter> for ScalarGatewayEvents {
        fn from(value: TokenMintLimitUpdatedFilter) -> Self {
            Self::TokenMintLimitUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<TokenSentFilter> for ScalarGatewayEvents {
        fn from(value: TokenSentFilter) -> Self {
            Self::TokenSentFilter(value)
        }
    }
    impl ::core::convert::From<UpgradedFilter> for ScalarGatewayEvents {
        fn from(value: UpgradedFilter) -> Self {
            Self::UpgradedFilter(value)
        }
    }
    ///Container type for all input parameters for the `adminEpoch` function with signature `adminEpoch()` and selector `0x364940d8`
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
    #[ethcall(name = "adminEpoch", abi = "adminEpoch()")]
    pub struct AdminEpochCall;
    ///Container type for all input parameters for the `adminThreshold` function with signature `adminThreshold(uint256)` and selector `0x88b30587`
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
    #[ethcall(name = "adminThreshold", abi = "adminThreshold(uint256)")]
    pub struct AdminThresholdCall(pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `admins` function with signature `admins(uint256)` and selector `0x14bfd6d0`
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
    #[ethcall(name = "admins", abi = "admins(uint256)")]
    pub struct AdminsCall(pub ::ethers::core::types::U256);
    ///Container type for all input parameters for the `allTokensFrozen` function with signature `allTokensFrozen()` and selector `0xaa1e1f0a`
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
    #[ethcall(name = "allTokensFrozen", abi = "allTokensFrozen()")]
    pub struct AllTokensFrozenCall;
    ///Container type for all input parameters for the `approveContractCall` function with signature `approveContractCall(bytes,bytes32)` and selector `0xfbe0a31b`
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
    #[ethcall(name = "approveContractCall", abi = "approveContractCall(bytes,bytes32)")]
    pub struct ApproveContractCallCall {
        pub params: ::ethers::core::types::Bytes,
        pub command_id: [u8; 32],
    }
    ///Container type for all input parameters for the `approveContractCallWithMint` function with signature `approveContractCallWithMint(bytes,bytes32)` and selector `0x585a9fd4`
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
        name = "approveContractCallWithMint",
        abi = "approveContractCallWithMint(bytes,bytes32)"
    )]
    pub struct ApproveContractCallWithMintCall {
        pub params: ::ethers::core::types::Bytes,
        pub command_id: [u8; 32],
    }
    ///Container type for all input parameters for the `authModule` function with signature `authModule()` and selector `0x64940c56`
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
    #[ethcall(name = "authModule", abi = "authModule()")]
    pub struct AuthModuleCall;
    ///Container type for all input parameters for the `burnToken` function with signature `burnToken(bytes,bytes32)` and selector `0x4656ae2e`
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
    #[ethcall(name = "burnToken", abi = "burnToken(bytes,bytes32)")]
    pub struct BurnTokenCall {
        pub params: ::ethers::core::types::Bytes,
        pub p1: [u8; 32],
    }
    ///Container type for all input parameters for the `callContract` function with signature `callContract(string,string,bytes)` and selector `0x1c92115f`
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
    #[ethcall(name = "callContract", abi = "callContract(string,string,bytes)")]
    pub struct CallContractCall {
        pub destination_chain: ::std::string::String,
        pub destination_contract_address: ::std::string::String,
        pub payload: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `callContractWithToken` function with signature `callContractWithToken(string,string,bytes,string,uint256)` and selector `0xb5417084`
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
        name = "callContractWithToken",
        abi = "callContractWithToken(string,string,bytes,string,uint256)"
    )]
    pub struct CallContractWithTokenCall {
        pub destination_chain: ::std::string::String,
        pub destination_contract_address: ::std::string::String,
        pub payload: ::ethers::core::types::Bytes,
        pub symbol: ::std::string::String,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `contractId` function with signature `contractId()` and selector `0x8291286c`
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
    #[ethcall(name = "contractId", abi = "contractId()")]
    pub struct ContractIdCall;
    ///Container type for all input parameters for the `deployToken` function with signature `deployToken(bytes,bytes32)` and selector `0x886a625d`
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
    #[ethcall(name = "deployToken", abi = "deployToken(bytes,bytes32)")]
    pub struct DeployTokenCall {
        pub params: ::ethers::core::types::Bytes,
        pub p1: [u8; 32],
    }
    ///Container type for all input parameters for the `execute` function with signature `execute(bytes)` and selector `0x09c5eabe`
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
    #[ethcall(name = "execute", abi = "execute(bytes)")]
    pub struct ExecuteCall {
        pub input: ::ethers::core::types::Bytes,
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
    ///Container type for all input parameters for the `governance` function with signature `governance()` and selector `0x5aa6e675`
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
    #[ethcall(name = "governance", abi = "governance()")]
    pub struct GovernanceCall;
    ///Container type for all input parameters for the `implementation` function with signature `implementation()` and selector `0x5c60da1b`
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
    #[ethcall(name = "implementation", abi = "implementation()")]
    pub struct ImplementationCall;
    ///Container type for all input parameters for the `isCommandExecuted` function with signature `isCommandExecuted(bytes32)` and selector `0xd26ff210`
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
    #[ethcall(name = "isCommandExecuted", abi = "isCommandExecuted(bytes32)")]
    pub struct IsCommandExecutedCall {
        pub command_id: [u8; 32],
    }
    ///Container type for all input parameters for the `isContractCallAndMintApproved` function with signature `isContractCallAndMintApproved(bytes32,string,string,address,bytes32,string,uint256)` and selector `0xbc00c216`
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
        name = "isContractCallAndMintApproved",
        abi = "isContractCallAndMintApproved(bytes32,string,string,address,bytes32,string,uint256)"
    )]
    pub struct IsContractCallAndMintApprovedCall {
        pub command_id: [u8; 32],
        pub source_chain: ::std::string::String,
        pub source_address: ::std::string::String,
        pub contract_address: ::ethers::core::types::Address,
        pub payload_hash: [u8; 32],
        pub symbol: ::std::string::String,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `isContractCallApproved` function with signature `isContractCallApproved(bytes32,string,string,address,bytes32)` and selector `0xf6a5f9f5`
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
        name = "isContractCallApproved",
        abi = "isContractCallApproved(bytes32,string,string,address,bytes32)"
    )]
    pub struct IsContractCallApprovedCall {
        pub command_id: [u8; 32],
        pub source_chain: ::std::string::String,
        pub source_address: ::std::string::String,
        pub contract_address: ::ethers::core::types::Address,
        pub payload_hash: [u8; 32],
    }
    ///Container type for all input parameters for the `mintLimiter` function with signature `mintLimiter()` and selector `0xc82fe87a`
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
    #[ethcall(name = "mintLimiter", abi = "mintLimiter()")]
    pub struct MintLimiterCall;
    ///Container type for all input parameters for the `mintToken` function with signature `mintToken(bytes,bytes32)` and selector `0x146e2d78`
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
    #[ethcall(name = "mintToken", abi = "mintToken(bytes,bytes32)")]
    pub struct MintTokenCall {
        pub params: ::ethers::core::types::Bytes,
        pub p1: [u8; 32],
    }
    ///Container type for all input parameters for the `resetOperatorship` function with signature `resetOperatorship(bytes)` and selector `0x00f7d49f`
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
    #[ethcall(name = "resetOperatorship", abi = "resetOperatorship(bytes)")]
    pub struct ResetOperatorshipCall {
        pub params: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `sendToken` function with signature `sendToken(string,string,string,uint256)` and selector `0x26ef699d`
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
    #[ethcall(name = "sendToken", abi = "sendToken(string,string,string,uint256)")]
    pub struct SendTokenCall {
        pub destination_chain: ::std::string::String,
        pub destination_address: ::std::string::String,
        pub symbol: ::std::string::String,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `setTokenMintLimits` function with signature `setTokenMintLimits(string[],uint256[])` and selector `0x67ace8eb`
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
        name = "setTokenMintLimits",
        abi = "setTokenMintLimits(string[],uint256[])"
    )]
    pub struct SetTokenMintLimitsCall {
        pub symbols: ::std::vec::Vec<::std::string::String>,
        pub limits: ::std::vec::Vec<::ethers::core::types::U256>,
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
    ///Container type for all input parameters for the `tokenAddresses` function with signature `tokenAddresses(string)` and selector `0x935b13f6`
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
    #[ethcall(name = "tokenAddresses", abi = "tokenAddresses(string)")]
    pub struct TokenAddressesCall {
        pub symbol: ::std::string::String,
    }
    ///Container type for all input parameters for the `tokenDeployer` function with signature `tokenDeployer()` and selector `0x2a2dae0a`
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
    #[ethcall(name = "tokenDeployer", abi = "tokenDeployer()")]
    pub struct TokenDeployerCall;
    ///Container type for all input parameters for the `tokenFrozen` function with signature `tokenFrozen(string)` and selector `0x7b1b769e`
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
    #[ethcall(name = "tokenFrozen", abi = "tokenFrozen(string)")]
    pub struct TokenFrozenCall(pub ::std::string::String);
    ///Container type for all input parameters for the `tokenMintAmount` function with signature `tokenMintAmount(string)` and selector `0xcec7b359`
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
    #[ethcall(name = "tokenMintAmount", abi = "tokenMintAmount(string)")]
    pub struct TokenMintAmountCall {
        pub symbol: ::std::string::String,
    }
    ///Container type for all input parameters for the `tokenMintLimit` function with signature `tokenMintLimit(string)` and selector `0x269eb65e`
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
    #[ethcall(name = "tokenMintLimit", abi = "tokenMintLimit(string)")]
    pub struct TokenMintLimitCall {
        pub symbol: ::std::string::String,
    }
    ///Container type for all input parameters for the `transferGovernance` function with signature `transferGovernance(address)` and selector `0xd38bfff4`
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
    #[ethcall(name = "transferGovernance", abi = "transferGovernance(address)")]
    pub struct TransferGovernanceCall {
        pub new_governance: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `transferMintLimiter` function with signature `transferMintLimiter(address)` and selector `0x41d8f26b`
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
    #[ethcall(name = "transferMintLimiter", abi = "transferMintLimiter(address)")]
    pub struct TransferMintLimiterCall {
        pub new_mint_limiter: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `transferOperatorship` function with signature `transferOperatorship(bytes,bytes32)` and selector `0x97b87ba6`
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
        name = "transferOperatorship",
        abi = "transferOperatorship(bytes,bytes32)"
    )]
    pub struct TransferOperatorshipCall {
        pub new_operators_data: ::ethers::core::types::Bytes,
        pub p1: [u8; 32],
    }
    ///Container type for all input parameters for the `upgrade` function with signature `upgrade(address,bytes32,bytes)` and selector `0xa3499c73`
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
    #[ethcall(name = "upgrade", abi = "upgrade(address,bytes32,bytes)")]
    pub struct UpgradeCall {
        pub new_implementation: ::ethers::core::types::Address,
        pub new_implementation_code_hash: [u8; 32],
        pub setup_params: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `validateContractCall` function with signature `validateContractCall(bytes32,string,string,bytes32)` and selector `0x5f6970c3`
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
        name = "validateContractCall",
        abi = "validateContractCall(bytes32,string,string,bytes32)"
    )]
    pub struct ValidateContractCallCall {
        pub command_id: [u8; 32],
        pub source_chain: ::std::string::String,
        pub source_address: ::std::string::String,
        pub payload_hash: [u8; 32],
    }
    ///Container type for all input parameters for the `validateContractCallAndMint` function with signature `validateContractCallAndMint(bytes32,string,string,bytes32,string,uint256)` and selector `0x1876eed9`
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
        name = "validateContractCallAndMint",
        abi = "validateContractCallAndMint(bytes32,string,string,bytes32,string,uint256)"
    )]
    pub struct ValidateContractCallAndMintCall {
        pub command_id: [u8; 32],
        pub source_chain: ::std::string::String,
        pub source_address: ::std::string::String,
        pub payload_hash: [u8; 32],
        pub symbol: ::std::string::String,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum ScalarGatewayCalls {
        AdminEpoch(AdminEpochCall),
        AdminThreshold(AdminThresholdCall),
        Admins(AdminsCall),
        AllTokensFrozen(AllTokensFrozenCall),
        ApproveContractCall(ApproveContractCallCall),
        ApproveContractCallWithMint(ApproveContractCallWithMintCall),
        AuthModule(AuthModuleCall),
        BurnToken(BurnTokenCall),
        CallContract(CallContractCall),
        CallContractWithToken(CallContractWithTokenCall),
        ContractId(ContractIdCall),
        DeployToken(DeployTokenCall),
        Execute(ExecuteCall),
        GetAddress(GetAddressCall),
        GetBool(GetBoolCall),
        GetBytes(GetBytesCall),
        GetInt(GetIntCall),
        GetString(GetStringCall),
        GetUint(GetUintCall),
        Governance(GovernanceCall),
        Implementation(ImplementationCall),
        IsCommandExecuted(IsCommandExecutedCall),
        IsContractCallAndMintApproved(IsContractCallAndMintApprovedCall),
        IsContractCallApproved(IsContractCallApprovedCall),
        MintLimiter(MintLimiterCall),
        MintToken(MintTokenCall),
        ResetOperatorship(ResetOperatorshipCall),
        SendToken(SendTokenCall),
        SetTokenMintLimits(SetTokenMintLimitsCall),
        Setup(SetupCall),
        TokenAddresses(TokenAddressesCall),
        TokenDeployer(TokenDeployerCall),
        TokenFrozen(TokenFrozenCall),
        TokenMintAmount(TokenMintAmountCall),
        TokenMintLimit(TokenMintLimitCall),
        TransferGovernance(TransferGovernanceCall),
        TransferMintLimiter(TransferMintLimiterCall),
        TransferOperatorship(TransferOperatorshipCall),
        Upgrade(UpgradeCall),
        ValidateContractCall(ValidateContractCallCall),
        ValidateContractCallAndMint(ValidateContractCallAndMintCall),
    }
    impl ::ethers::core::abi::AbiDecode for ScalarGatewayCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <AdminEpochCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AdminEpoch(decoded));
            }
            if let Ok(decoded) = <AdminThresholdCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AdminThreshold(decoded));
            }
            if let Ok(decoded) = <AdminsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Admins(decoded));
            }
            if let Ok(decoded) = <AllTokensFrozenCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AllTokensFrozen(decoded));
            }
            if let Ok(decoded) = <ApproveContractCallCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ApproveContractCall(decoded));
            }
            if let Ok(decoded) = <ApproveContractCallWithMintCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ApproveContractCallWithMint(decoded));
            }
            if let Ok(decoded) = <AuthModuleCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AuthModule(decoded));
            }
            if let Ok(decoded) = <BurnTokenCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::BurnToken(decoded));
            }
            if let Ok(decoded) = <CallContractCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CallContract(decoded));
            }
            if let Ok(decoded) = <CallContractWithTokenCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CallContractWithToken(decoded));
            }
            if let Ok(decoded) = <ContractIdCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ContractId(decoded));
            }
            if let Ok(decoded) = <DeployTokenCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DeployToken(decoded));
            }
            if let Ok(decoded) = <ExecuteCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Execute(decoded));
            }
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
            if let Ok(decoded) = <GovernanceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Governance(decoded));
            }
            if let Ok(decoded) = <ImplementationCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Implementation(decoded));
            }
            if let Ok(decoded) = <IsCommandExecutedCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IsCommandExecuted(decoded));
            }
            if let Ok(decoded) = <IsContractCallAndMintApprovedCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IsContractCallAndMintApproved(decoded));
            }
            if let Ok(decoded) = <IsContractCallApprovedCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IsContractCallApproved(decoded));
            }
            if let Ok(decoded) = <MintLimiterCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MintLimiter(decoded));
            }
            if let Ok(decoded) = <MintTokenCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MintToken(decoded));
            }
            if let Ok(decoded) = <ResetOperatorshipCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ResetOperatorship(decoded));
            }
            if let Ok(decoded) = <SendTokenCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SendToken(decoded));
            }
            if let Ok(decoded) = <SetTokenMintLimitsCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetTokenMintLimits(decoded));
            }
            if let Ok(decoded) = <SetupCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Setup(decoded));
            }
            if let Ok(decoded) = <TokenAddressesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TokenAddresses(decoded));
            }
            if let Ok(decoded) = <TokenDeployerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TokenDeployer(decoded));
            }
            if let Ok(decoded) = <TokenFrozenCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TokenFrozen(decoded));
            }
            if let Ok(decoded) = <TokenMintAmountCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TokenMintAmount(decoded));
            }
            if let Ok(decoded) = <TokenMintLimitCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TokenMintLimit(decoded));
            }
            if let Ok(decoded) = <TransferGovernanceCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TransferGovernance(decoded));
            }
            if let Ok(decoded) = <TransferMintLimiterCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TransferMintLimiter(decoded));
            }
            if let Ok(decoded) = <TransferOperatorshipCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TransferOperatorship(decoded));
            }
            if let Ok(decoded) = <UpgradeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Upgrade(decoded));
            }
            if let Ok(decoded) = <ValidateContractCallCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ValidateContractCall(decoded));
            }
            if let Ok(decoded) = <ValidateContractCallAndMintCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ValidateContractCallAndMint(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ScalarGatewayCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AdminEpoch(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AdminThreshold(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Admins(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::AllTokensFrozen(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ApproveContractCall(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ApproveContractCallWithMint(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::AuthModule(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BurnToken(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CallContract(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CallContractWithToken(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ContractId(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::DeployToken(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Execute(element) => ::ethers::core::abi::AbiEncode::encode(element),
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
                Self::Governance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Implementation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsCommandExecuted(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsContractCallAndMintApproved(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsContractCallApproved(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MintLimiter(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MintToken(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ResetOperatorship(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SendToken(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetTokenMintLimits(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Setup(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TokenAddresses(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TokenDeployer(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TokenFrozen(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TokenMintAmount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TokenMintLimit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferGovernance(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferMintLimiter(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferOperatorship(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Upgrade(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ValidateContractCall(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ValidateContractCallAndMint(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for ScalarGatewayCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AdminEpoch(element) => ::core::fmt::Display::fmt(element, f),
                Self::AdminThreshold(element) => ::core::fmt::Display::fmt(element, f),
                Self::Admins(element) => ::core::fmt::Display::fmt(element, f),
                Self::AllTokensFrozen(element) => ::core::fmt::Display::fmt(element, f),
                Self::ApproveContractCall(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ApproveContractCallWithMint(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::AuthModule(element) => ::core::fmt::Display::fmt(element, f),
                Self::BurnToken(element) => ::core::fmt::Display::fmt(element, f),
                Self::CallContract(element) => ::core::fmt::Display::fmt(element, f),
                Self::CallContractWithToken(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ContractId(element) => ::core::fmt::Display::fmt(element, f),
                Self::DeployToken(element) => ::core::fmt::Display::fmt(element, f),
                Self::Execute(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetBool(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetBytes(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetInt(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetString(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetUint(element) => ::core::fmt::Display::fmt(element, f),
                Self::Governance(element) => ::core::fmt::Display::fmt(element, f),
                Self::Implementation(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsCommandExecuted(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsContractCallAndMintApproved(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::IsContractCallApproved(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MintLimiter(element) => ::core::fmt::Display::fmt(element, f),
                Self::MintToken(element) => ::core::fmt::Display::fmt(element, f),
                Self::ResetOperatorship(element) => ::core::fmt::Display::fmt(element, f),
                Self::SendToken(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetTokenMintLimits(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Setup(element) => ::core::fmt::Display::fmt(element, f),
                Self::TokenAddresses(element) => ::core::fmt::Display::fmt(element, f),
                Self::TokenDeployer(element) => ::core::fmt::Display::fmt(element, f),
                Self::TokenFrozen(element) => ::core::fmt::Display::fmt(element, f),
                Self::TokenMintAmount(element) => ::core::fmt::Display::fmt(element, f),
                Self::TokenMintLimit(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferGovernance(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TransferMintLimiter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TransferOperatorship(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Upgrade(element) => ::core::fmt::Display::fmt(element, f),
                Self::ValidateContractCall(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ValidateContractCallAndMint(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<AdminEpochCall> for ScalarGatewayCalls {
        fn from(value: AdminEpochCall) -> Self {
            Self::AdminEpoch(value)
        }
    }
    impl ::core::convert::From<AdminThresholdCall> for ScalarGatewayCalls {
        fn from(value: AdminThresholdCall) -> Self {
            Self::AdminThreshold(value)
        }
    }
    impl ::core::convert::From<AdminsCall> for ScalarGatewayCalls {
        fn from(value: AdminsCall) -> Self {
            Self::Admins(value)
        }
    }
    impl ::core::convert::From<AllTokensFrozenCall> for ScalarGatewayCalls {
        fn from(value: AllTokensFrozenCall) -> Self {
            Self::AllTokensFrozen(value)
        }
    }
    impl ::core::convert::From<ApproveContractCallCall> for ScalarGatewayCalls {
        fn from(value: ApproveContractCallCall) -> Self {
            Self::ApproveContractCall(value)
        }
    }
    impl ::core::convert::From<ApproveContractCallWithMintCall> for ScalarGatewayCalls {
        fn from(value: ApproveContractCallWithMintCall) -> Self {
            Self::ApproveContractCallWithMint(value)
        }
    }
    impl ::core::convert::From<AuthModuleCall> for ScalarGatewayCalls {
        fn from(value: AuthModuleCall) -> Self {
            Self::AuthModule(value)
        }
    }
    impl ::core::convert::From<BurnTokenCall> for ScalarGatewayCalls {
        fn from(value: BurnTokenCall) -> Self {
            Self::BurnToken(value)
        }
    }
    impl ::core::convert::From<CallContractCall> for ScalarGatewayCalls {
        fn from(value: CallContractCall) -> Self {
            Self::CallContract(value)
        }
    }
    impl ::core::convert::From<CallContractWithTokenCall> for ScalarGatewayCalls {
        fn from(value: CallContractWithTokenCall) -> Self {
            Self::CallContractWithToken(value)
        }
    }
    impl ::core::convert::From<ContractIdCall> for ScalarGatewayCalls {
        fn from(value: ContractIdCall) -> Self {
            Self::ContractId(value)
        }
    }
    impl ::core::convert::From<DeployTokenCall> for ScalarGatewayCalls {
        fn from(value: DeployTokenCall) -> Self {
            Self::DeployToken(value)
        }
    }
    impl ::core::convert::From<ExecuteCall> for ScalarGatewayCalls {
        fn from(value: ExecuteCall) -> Self {
            Self::Execute(value)
        }
    }
    impl ::core::convert::From<GetAddressCall> for ScalarGatewayCalls {
        fn from(value: GetAddressCall) -> Self {
            Self::GetAddress(value)
        }
    }
    impl ::core::convert::From<GetBoolCall> for ScalarGatewayCalls {
        fn from(value: GetBoolCall) -> Self {
            Self::GetBool(value)
        }
    }
    impl ::core::convert::From<GetBytesCall> for ScalarGatewayCalls {
        fn from(value: GetBytesCall) -> Self {
            Self::GetBytes(value)
        }
    }
    impl ::core::convert::From<GetIntCall> for ScalarGatewayCalls {
        fn from(value: GetIntCall) -> Self {
            Self::GetInt(value)
        }
    }
    impl ::core::convert::From<GetStringCall> for ScalarGatewayCalls {
        fn from(value: GetStringCall) -> Self {
            Self::GetString(value)
        }
    }
    impl ::core::convert::From<GetUintCall> for ScalarGatewayCalls {
        fn from(value: GetUintCall) -> Self {
            Self::GetUint(value)
        }
    }
    impl ::core::convert::From<GovernanceCall> for ScalarGatewayCalls {
        fn from(value: GovernanceCall) -> Self {
            Self::Governance(value)
        }
    }
    impl ::core::convert::From<ImplementationCall> for ScalarGatewayCalls {
        fn from(value: ImplementationCall) -> Self {
            Self::Implementation(value)
        }
    }
    impl ::core::convert::From<IsCommandExecutedCall> for ScalarGatewayCalls {
        fn from(value: IsCommandExecutedCall) -> Self {
            Self::IsCommandExecuted(value)
        }
    }
    impl ::core::convert::From<IsContractCallAndMintApprovedCall>
    for ScalarGatewayCalls {
        fn from(value: IsContractCallAndMintApprovedCall) -> Self {
            Self::IsContractCallAndMintApproved(value)
        }
    }
    impl ::core::convert::From<IsContractCallApprovedCall> for ScalarGatewayCalls {
        fn from(value: IsContractCallApprovedCall) -> Self {
            Self::IsContractCallApproved(value)
        }
    }
    impl ::core::convert::From<MintLimiterCall> for ScalarGatewayCalls {
        fn from(value: MintLimiterCall) -> Self {
            Self::MintLimiter(value)
        }
    }
    impl ::core::convert::From<MintTokenCall> for ScalarGatewayCalls {
        fn from(value: MintTokenCall) -> Self {
            Self::MintToken(value)
        }
    }
    impl ::core::convert::From<ResetOperatorshipCall> for ScalarGatewayCalls {
        fn from(value: ResetOperatorshipCall) -> Self {
            Self::ResetOperatorship(value)
        }
    }
    impl ::core::convert::From<SendTokenCall> for ScalarGatewayCalls {
        fn from(value: SendTokenCall) -> Self {
            Self::SendToken(value)
        }
    }
    impl ::core::convert::From<SetTokenMintLimitsCall> for ScalarGatewayCalls {
        fn from(value: SetTokenMintLimitsCall) -> Self {
            Self::SetTokenMintLimits(value)
        }
    }
    impl ::core::convert::From<SetupCall> for ScalarGatewayCalls {
        fn from(value: SetupCall) -> Self {
            Self::Setup(value)
        }
    }
    impl ::core::convert::From<TokenAddressesCall> for ScalarGatewayCalls {
        fn from(value: TokenAddressesCall) -> Self {
            Self::TokenAddresses(value)
        }
    }
    impl ::core::convert::From<TokenDeployerCall> for ScalarGatewayCalls {
        fn from(value: TokenDeployerCall) -> Self {
            Self::TokenDeployer(value)
        }
    }
    impl ::core::convert::From<TokenFrozenCall> for ScalarGatewayCalls {
        fn from(value: TokenFrozenCall) -> Self {
            Self::TokenFrozen(value)
        }
    }
    impl ::core::convert::From<TokenMintAmountCall> for ScalarGatewayCalls {
        fn from(value: TokenMintAmountCall) -> Self {
            Self::TokenMintAmount(value)
        }
    }
    impl ::core::convert::From<TokenMintLimitCall> for ScalarGatewayCalls {
        fn from(value: TokenMintLimitCall) -> Self {
            Self::TokenMintLimit(value)
        }
    }
    impl ::core::convert::From<TransferGovernanceCall> for ScalarGatewayCalls {
        fn from(value: TransferGovernanceCall) -> Self {
            Self::TransferGovernance(value)
        }
    }
    impl ::core::convert::From<TransferMintLimiterCall> for ScalarGatewayCalls {
        fn from(value: TransferMintLimiterCall) -> Self {
            Self::TransferMintLimiter(value)
        }
    }
    impl ::core::convert::From<TransferOperatorshipCall> for ScalarGatewayCalls {
        fn from(value: TransferOperatorshipCall) -> Self {
            Self::TransferOperatorship(value)
        }
    }
    impl ::core::convert::From<UpgradeCall> for ScalarGatewayCalls {
        fn from(value: UpgradeCall) -> Self {
            Self::Upgrade(value)
        }
    }
    impl ::core::convert::From<ValidateContractCallCall> for ScalarGatewayCalls {
        fn from(value: ValidateContractCallCall) -> Self {
            Self::ValidateContractCall(value)
        }
    }
    impl ::core::convert::From<ValidateContractCallAndMintCall> for ScalarGatewayCalls {
        fn from(value: ValidateContractCallAndMintCall) -> Self {
            Self::ValidateContractCallAndMint(value)
        }
    }
    ///Container type for all return fields from the `adminEpoch` function with signature `adminEpoch()` and selector `0x364940d8`
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
    pub struct AdminEpochReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `adminThreshold` function with signature `adminThreshold(uint256)` and selector `0x88b30587`
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
    pub struct AdminThresholdReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `admins` function with signature `admins(uint256)` and selector `0x14bfd6d0`
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
    pub struct AdminsReturn(pub ::std::vec::Vec<::ethers::core::types::Address>);
    ///Container type for all return fields from the `allTokensFrozen` function with signature `allTokensFrozen()` and selector `0xaa1e1f0a`
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
    pub struct AllTokensFrozenReturn(pub bool);
    ///Container type for all return fields from the `authModule` function with signature `authModule()` and selector `0x64940c56`
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
    pub struct AuthModuleReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `contractId` function with signature `contractId()` and selector `0x8291286c`
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
    pub struct ContractIdReturn(pub [u8; 32]);
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
    ///Container type for all return fields from the `governance` function with signature `governance()` and selector `0x5aa6e675`
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
    pub struct GovernanceReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `implementation` function with signature `implementation()` and selector `0x5c60da1b`
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
    pub struct ImplementationReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `isCommandExecuted` function with signature `isCommandExecuted(bytes32)` and selector `0xd26ff210`
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
    pub struct IsCommandExecutedReturn(pub bool);
    ///Container type for all return fields from the `isContractCallAndMintApproved` function with signature `isContractCallAndMintApproved(bytes32,string,string,address,bytes32,string,uint256)` and selector `0xbc00c216`
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
    pub struct IsContractCallAndMintApprovedReturn(pub bool);
    ///Container type for all return fields from the `isContractCallApproved` function with signature `isContractCallApproved(bytes32,string,string,address,bytes32)` and selector `0xf6a5f9f5`
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
    pub struct IsContractCallApprovedReturn(pub bool);
    ///Container type for all return fields from the `mintLimiter` function with signature `mintLimiter()` and selector `0xc82fe87a`
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
    pub struct MintLimiterReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `tokenAddresses` function with signature `tokenAddresses(string)` and selector `0x935b13f6`
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
    pub struct TokenAddressesReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `tokenDeployer` function with signature `tokenDeployer()` and selector `0x2a2dae0a`
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
    pub struct TokenDeployerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `tokenFrozen` function with signature `tokenFrozen(string)` and selector `0x7b1b769e`
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
    pub struct TokenFrozenReturn(pub bool);
    ///Container type for all return fields from the `tokenMintAmount` function with signature `tokenMintAmount(string)` and selector `0xcec7b359`
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
    pub struct TokenMintAmountReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `tokenMintLimit` function with signature `tokenMintLimit(string)` and selector `0x269eb65e`
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
    pub struct TokenMintLimitReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `validateContractCall` function with signature `validateContractCall(bytes32,string,string,bytes32)` and selector `0x5f6970c3`
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
    pub struct ValidateContractCallReturn {
        pub valid: bool,
    }
    ///Container type for all return fields from the `validateContractCallAndMint` function with signature `validateContractCallAndMint(bytes32,string,string,bytes32,string,uint256)` and selector `0x1876eed9`
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
    pub struct ValidateContractCallAndMintReturn {
        pub valid: bool,
    }
}

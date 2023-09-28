pub use message_sender::*;
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
pub mod message_sender {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("_gateway"),
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
                    ::std::borrow::ToOwned::to_owned("sendMessage"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("sendMessage"),
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
                                    name: ::std::borrow::ToOwned::to_owned("value_"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::String,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("string"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::Payable,
                        },
                    ],
                ),
            ]),
            events: ::std::collections::BTreeMap::new(),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static MESSAGESENDER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(
        __abi,
    );
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\xC0`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`@Qa\x07\x078\x03\x80a\x07\x07\x839\x81\x81\x01`@R\x81\x01\x90a\x002\x91\x90a\x01\x04V[\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x80\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\xA0\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPPPPa\x01DV[`\0\x80\xFD[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[`\0a\0\xD1\x82a\0\xA6V[\x90P\x91\x90PV[a\0\xE1\x81a\0\xC6V[\x81\x14a\0\xECW`\0\x80\xFD[PV[`\0\x81Q\x90Pa\0\xFE\x81a\0\xD8V[\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a\x01\x1BWa\x01\x1Aa\0\xA1V[[`\0a\x01)\x85\x82\x86\x01a\0\xEFV[\x92PP` a\x01:\x85\x82\x86\x01a\0\xEFV[\x91PP\x92P\x92\x90PV[`\x80Q`\xA0Qa\x05\x9Fa\x01h`\09`\0`\xAA\x01R`\0a\x01B\x01Ra\x05\x9F`\0\xF3\xFE`\x80`@R`\x046\x10a\0\x1EW`\x005`\xE0\x1C\x80c\x0E\xAB\xEF\xFE\x14a\0#W[`\0\x80\xFD[a\0=`\x04\x806\x03\x81\x01\x90a\08\x91\x90a\x02KV[a\0?V[\0[`\x004\x11a\0\x82W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\0y\x90a\x03\\V[`@Q\x80\x91\x03\x90\xFD[`\0\x82\x82`@Q` \x01a\0\x97\x92\x91\x90a\x03\xC9V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x0C\x93\xE3\xBB40\x8A\x8A\x8A\x8A\x883`@Q\x89c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x01\x0E\x97\x96\x95\x94\x93\x92\x91\x90a\x04\xADV[`\0`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a\x01'W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x01;W=`\0\x80>=`\0\xFD[PPPPP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x1C\x92\x11_\x88\x88\x88\x88\x86`@Q\x86c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x01\xA1\x95\x94\x93\x92\x91\x90a\x05\x19V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x01\xBBW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x01\xCFW=`\0\x80>=`\0\xFD[PPPPPPPPPPPV[`\0\x80\xFD[`\0\x80\xFD[`\0\x80\xFD[`\0\x80\xFD[`\0\x80\xFD[`\0\x80\x83`\x1F\x84\x01\x12a\x02\x0BWa\x02\na\x01\xE6V[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x02(Wa\x02'a\x01\xEBV[[` \x83\x01\x91P\x83`\x01\x82\x02\x83\x01\x11\x15a\x02DWa\x02Ca\x01\xF0V[[\x92P\x92\x90PV[`\0\x80`\0\x80`\0\x80``\x87\x89\x03\x12\x15a\x02hWa\x02ga\x01\xDCV[[`\0\x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x02\x86Wa\x02\x85a\x01\xE1V[[a\x02\x92\x89\x82\x8A\x01a\x01\xF5V[\x96P\x96PP` \x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x02\xB5Wa\x02\xB4a\x01\xE1V[[a\x02\xC1\x89\x82\x8A\x01a\x01\xF5V[\x94P\x94PP`@\x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x02\xE4Wa\x02\xE3a\x01\xE1V[[a\x02\xF0\x89\x82\x8A\x01a\x01\xF5V[\x92P\x92PP\x92\x95P\x92\x95P\x92\x95V[`\0\x82\x82R` \x82\x01\x90P\x92\x91PPV[\x7FGas payment is required\0\0\0\0\0\0\0\0\0`\0\x82\x01RPV[`\0a\x03F`\x17\x83a\x02\xFFV[\x91Pa\x03Q\x82a\x03\x10V[` \x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x03u\x81a\x039V[\x90P\x91\x90PV[\x82\x81\x837`\0\x83\x83\x01RPPPV[`\0`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[`\0a\x03\xA8\x83\x85a\x02\xFFV[\x93Pa\x03\xB5\x83\x85\x84a\x03|V[a\x03\xBE\x83a\x03\x8BV[\x84\x01\x90P\x93\x92PPPV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x03\xE4\x81\x84\x86a\x03\x9CV[\x90P\x93\x92PPPV[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[`\0a\x04\x18\x82a\x03\xEDV[\x90P\x91\x90PV[a\x04(\x81a\x04\rV[\x82RPPV[`\0\x81Q\x90P\x91\x90PV[`\0\x82\x82R` \x82\x01\x90P\x92\x91PPV[`\0[\x83\x81\x10\x15a\x04hW\x80\x82\x01Q\x81\x84\x01R` \x81\x01\x90Pa\x04MV[`\0\x84\x84\x01RPPPPV[`\0a\x04\x7F\x82a\x04.V[a\x04\x89\x81\x85a\x049V[\x93Pa\x04\x99\x81\x85` \x86\x01a\x04JV[a\x04\xA2\x81a\x03\x8BV[\x84\x01\x91PP\x92\x91PPV[`\0`\xA0\x82\x01\x90Pa\x04\xC2`\0\x83\x01\x8Aa\x04\x1FV[\x81\x81\x03` \x83\x01Ra\x04\xD5\x81\x88\x8Aa\x03\x9CV[\x90P\x81\x81\x03`@\x83\x01Ra\x04\xEA\x81\x86\x88a\x03\x9CV[\x90P\x81\x81\x03``\x83\x01Ra\x04\xFE\x81\x85a\x04tV[\x90Pa\x05\r`\x80\x83\x01\x84a\x04\x1FV[\x98\x97PPPPPPPPV[`\0``\x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x054\x81\x87\x89a\x03\x9CV[\x90P\x81\x81\x03` \x83\x01Ra\x05I\x81\x85\x87a\x03\x9CV[\x90P\x81\x81\x03`@\x83\x01Ra\x05]\x81\x84a\x04tV[\x90P\x96\x95PPPPPPV\xFE\xA2dipfsX\"\x12 \xBA\x93\xD5\xEA-JQP\xBBz:\x91]1>Yb\x86\xA5*\xC9j\xA3\xA3\x8FMO\xDC\xDD\xD5\r\xD1dsolcC\0\x08\x13\x003";
    /// The bytecode of the contract.
    pub static MESSAGESENDER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R`\x046\x10a\0\x1EW`\x005`\xE0\x1C\x80c\x0E\xAB\xEF\xFE\x14a\0#W[`\0\x80\xFD[a\0=`\x04\x806\x03\x81\x01\x90a\08\x91\x90a\x02KV[a\0?V[\0[`\x004\x11a\0\x82W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\0y\x90a\x03\\V[`@Q\x80\x91\x03\x90\xFD[`\0\x82\x82`@Q` \x01a\0\x97\x92\x91\x90a\x03\xC9V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x90P\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x0C\x93\xE3\xBB40\x8A\x8A\x8A\x8A\x883`@Q\x89c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x01\x0E\x97\x96\x95\x94\x93\x92\x91\x90a\x04\xADV[`\0`@Q\x80\x83\x03\x81\x85\x88\x80;\x15\x80\x15a\x01'W`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x01;W=`\0\x80>=`\0\xFD[PPPPP\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x1C\x92\x11_\x88\x88\x88\x88\x86`@Q\x86c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x01\xA1\x95\x94\x93\x92\x91\x90a\x05\x19V[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x01\xBBW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x01\xCFW=`\0\x80>=`\0\xFD[PPPPPPPPPPPV[`\0\x80\xFD[`\0\x80\xFD[`\0\x80\xFD[`\0\x80\xFD[`\0\x80\xFD[`\0\x80\x83`\x1F\x84\x01\x12a\x02\x0BWa\x02\na\x01\xE6V[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x02(Wa\x02'a\x01\xEBV[[` \x83\x01\x91P\x83`\x01\x82\x02\x83\x01\x11\x15a\x02DWa\x02Ca\x01\xF0V[[\x92P\x92\x90PV[`\0\x80`\0\x80`\0\x80``\x87\x89\x03\x12\x15a\x02hWa\x02ga\x01\xDCV[[`\0\x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x02\x86Wa\x02\x85a\x01\xE1V[[a\x02\x92\x89\x82\x8A\x01a\x01\xF5V[\x96P\x96PP` \x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x02\xB5Wa\x02\xB4a\x01\xE1V[[a\x02\xC1\x89\x82\x8A\x01a\x01\xF5V[\x94P\x94PP`@\x87\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x02\xE4Wa\x02\xE3a\x01\xE1V[[a\x02\xF0\x89\x82\x8A\x01a\x01\xF5V[\x92P\x92PP\x92\x95P\x92\x95P\x92\x95V[`\0\x82\x82R` \x82\x01\x90P\x92\x91PPV[\x7FGas payment is required\0\0\0\0\0\0\0\0\0`\0\x82\x01RPV[`\0a\x03F`\x17\x83a\x02\xFFV[\x91Pa\x03Q\x82a\x03\x10V[` \x82\x01\x90P\x91\x90PV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x03u\x81a\x039V[\x90P\x91\x90PV[\x82\x81\x837`\0\x83\x83\x01RPPPV[`\0`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[`\0a\x03\xA8\x83\x85a\x02\xFFV[\x93Pa\x03\xB5\x83\x85\x84a\x03|V[a\x03\xBE\x83a\x03\x8BV[\x84\x01\x90P\x93\x92PPPV[`\0` \x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x03\xE4\x81\x84\x86a\x03\x9CV[\x90P\x93\x92PPPV[`\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[`\0a\x04\x18\x82a\x03\xEDV[\x90P\x91\x90PV[a\x04(\x81a\x04\rV[\x82RPPV[`\0\x81Q\x90P\x91\x90PV[`\0\x82\x82R` \x82\x01\x90P\x92\x91PPV[`\0[\x83\x81\x10\x15a\x04hW\x80\x82\x01Q\x81\x84\x01R` \x81\x01\x90Pa\x04MV[`\0\x84\x84\x01RPPPPV[`\0a\x04\x7F\x82a\x04.V[a\x04\x89\x81\x85a\x049V[\x93Pa\x04\x99\x81\x85` \x86\x01a\x04JV[a\x04\xA2\x81a\x03\x8BV[\x84\x01\x91PP\x92\x91PPV[`\0`\xA0\x82\x01\x90Pa\x04\xC2`\0\x83\x01\x8Aa\x04\x1FV[\x81\x81\x03` \x83\x01Ra\x04\xD5\x81\x88\x8Aa\x03\x9CV[\x90P\x81\x81\x03`@\x83\x01Ra\x04\xEA\x81\x86\x88a\x03\x9CV[\x90P\x81\x81\x03``\x83\x01Ra\x04\xFE\x81\x85a\x04tV[\x90Pa\x05\r`\x80\x83\x01\x84a\x04\x1FV[\x98\x97PPPPPPPPV[`\0``\x82\x01\x90P\x81\x81\x03`\0\x83\x01Ra\x054\x81\x87\x89a\x03\x9CV[\x90P\x81\x81\x03` \x83\x01Ra\x05I\x81\x85\x87a\x03\x9CV[\x90P\x81\x81\x03`@\x83\x01Ra\x05]\x81\x84a\x04tV[\x90P\x96\x95PPPPPPV\xFE\xA2dipfsX\"\x12 \xBA\x93\xD5\xEA-JQP\xBBz:\x91]1>Yb\x86\xA5*\xC9j\xA3\xA3\x8FMO\xDC\xDD\xD5\r\xD1dsolcC\0\x08\x13\x003";
    /// The deployed bytecode of the contract.
    pub static MESSAGESENDER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct MessageSender<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for MessageSender<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for MessageSender<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for MessageSender<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for MessageSender<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(MessageSender))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> MessageSender<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    MESSAGESENDER_ABI.clone(),
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
                MESSAGESENDER_ABI.clone(),
                MESSAGESENDER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `sendMessage` (0x0eabeffe) function
        pub fn send_message(
            &self,
            destination_chain: ::std::string::String,
            destination_address: ::std::string::String,
            value: ::std::string::String,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [14, 171, 239, 254],
                    (destination_chain, destination_address, value),
                )
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for MessageSender<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `sendMessage` function with signature `sendMessage(string,string,string)` and selector `0x0eabeffe`
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
    #[ethcall(name = "sendMessage", abi = "sendMessage(string,string,string)")]
    pub struct SendMessageCall {
        pub destination_chain: ::std::string::String,
        pub destination_address: ::std::string::String,
        pub value: ::std::string::String,
    }
}

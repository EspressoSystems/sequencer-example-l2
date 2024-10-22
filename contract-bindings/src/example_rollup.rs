pub use example_rollup::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types
)]
pub mod example_rollup {
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::Some(::ethers::core::abi::ethabi::Constructor {
                inputs: ::std::vec![
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("lightClientAddress"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Address,
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("address"),
                        ),
                    },
                    ::ethers::core::abi::ethabi::Param {
                        name: ::std::borrow::ToOwned::to_owned("initialState"),
                        kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                        internal_type: ::core::option::Option::Some(
                            ::std::borrow::ToOwned::to_owned("uint256"),
                        ),
                    },
                ],
            }),
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("lightClient"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("lightClient"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Address,
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("contract LightClient"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("numVerifiedBlocks"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("numVerifiedBlocks"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("stateCommitment"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("stateCommitment"),
                        inputs: ::std::vec![],
                        outputs: ::std::vec![::ethers::core::abi::ethabi::Param {
                            name: ::std::string::String::new(),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            internal_type: ::core::option::Option::Some(
                                ::std::borrow::ToOwned::to_owned("uint256"),
                            ),
                        },],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("verifyBlocks"),
                    ::std::vec![::ethers::core::abi::ethabi::Function {
                        name: ::std::borrow::ToOwned::to_owned("verifyBlocks"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("count"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint64"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("nextStateCommitment",),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("proof"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Tuple(::std::vec![
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                ],),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned(
                                        "struct ExampleRollup.BatchProof",
                                    ),
                                ),
                            },
                        ],
                        outputs: ::std::vec![],
                        constant: ::core::option::Option::None,
                        state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                    },],
                ),
            ]),
            events: ::core::convert::From::from([(
                ::std::borrow::ToOwned::to_owned("StateUpdate"),
                ::std::vec![::ethers::core::abi::ethabi::Event {
                    name: ::std::borrow::ToOwned::to_owned("StateUpdate"),
                    inputs: ::std::vec![
                        ::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("blockHeight"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            indexed: false,
                        },
                        ::ethers::core::abi::ethabi::EventParam {
                            name: ::std::borrow::ToOwned::to_owned("stateCommitment"),
                            kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                            indexed: false,
                        },
                    ],
                    anonymous: false,
                },],
            )]),
            errors: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("InvalidProof"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("InvalidProof"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NoBlocks"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("NoBlocks"),
                        inputs: ::std::vec![],
                    },],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("NotYetSequenced"),
                    ::std::vec![::ethers::core::abi::ethabi::AbiError {
                        name: ::std::borrow::ToOwned::to_owned("NotYetSequenced"),
                        inputs: ::std::vec![
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("numVerifiedBlocks"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("count"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(64usize),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint64"),
                                ),
                            },
                            ::ethers::core::abi::ethabi::Param {
                                name: ::std::borrow::ToOwned::to_owned("blockHeight"),
                                kind: ::ethers::core::abi::ethabi::ParamType::Uint(256usize,),
                                internal_type: ::core::option::Option::Some(
                                    ::std::borrow::ToOwned::to_owned("uint256"),
                                ),
                            },
                        ],
                    },],
                ),
            ]),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static EXAMPLEROLLUP_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15`\x0FW`\0\x80\xFD[P`@Qa\x04G8\x03\x80a\x04G\x839\x81\x01`@\x81\x90R`,\x91`VV[`\0\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16\x92\x90\x92\x17\x82U`\x01U`\x02U`\x8EV[`\0\x80`@\x83\x85\x03\x12\x15`hW`\0\x80\xFD[\x82Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14`~W`\0\x80\xFD[` \x93\x90\x93\x01Q\x92\x94\x92\x93PPPV[a\x03\xAA\x80a\0\x9D`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0LW`\x005`\xE0\x1C\x80c\x03%q\xA9\x14a\0QW\x80cA,\xC8\xFE\x14a\0fW\x80c\xB5p\x0Eh\x14a\0\x82W\x80c\xD8\0t\x1E\x14a\0\xADW[`\0\x80\xFD[a\0da\0_6`\x04a\x02gV[a\0\xB6V[\0[a\0o`\x02T\x81V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[`\0Ta\0\x95\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0yV[a\0o`\x01T\x81V[\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0\x03a\0\xE1W`@Qc\x0F\xD4\xB67`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81\x81``\x01Q\x14a\x01\x05W`@Qc\t\xBD\xE39`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x9F\xDBT\xA7`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01```@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x01YW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01}\x91\x90a\x03\nV[P\x91PP\x80g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x02Ta\x01\xA4\x91\x90a\x03MV[\x11\x15a\x01\xE6W`\x02T`@Qc\xF08Hg`\xE0\x1B\x81R`\x04\x81\x01\x91\x90\x91Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x86\x16`$\x83\x01R\x82\x16`D\x82\x01R`d\x01`@Q\x80\x91\x03\x90\xFD[\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x02`\0\x82\x82Ta\x02\x02\x91\x90a\x03MV[\x90\x91UPP`\x01\x83\x90U`\x02T`@\x80Q\x91\x82R` \x82\x01\x85\x90R\x7F\x9C:SN\xC4A\xC7c:pv_T\xC0\x02L\xA8\xF9\xA7nz,\xDA\xAC*j\x8C5\x19\xC0\xCA\xF3\x91\x01`@Q\x80\x91\x03\x90\xA1PPPPV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x02dW`\0\x80\xFD[PV[`\0\x80`\0\x83\x85\x03`\xC0\x81\x12\x15a\x02}W`\0\x80\xFD[\x845a\x02\x88\x81a\x02NV[\x93P` \x85\x015\x92P`\x80`?\x19\x82\x01\x12\x15a\x02\xA3W`\0\x80\xFD[P`@Q`\x80\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x02\xD5WcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[\x80`@RP`@\x85\x015\x81R``\x85\x015` \x82\x01R`\x80\x85\x015`@\x82\x01R`\xA0\x85\x015``\x82\x01R\x80\x91PP\x92P\x92P\x92V[`\0\x80`\0``\x84\x86\x03\x12\x15a\x03\x1FW`\0\x80\xFD[\x83Qa\x03*\x81a\x02NV[` \x85\x01Q\x90\x93Pa\x03;\x81a\x02NV[\x80\x92PP`@\x84\x01Q\x90P\x92P\x92P\x92V[\x80\x82\x01\x80\x82\x11\x15a\x03nWcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x92\x91PPV\xFE\xA2dipfsX\"\x12 \xE2\r\xDD\xA5\xE9A\x1D\xD3\x8F,\x11\xAA\xCC\x81\n\xA7\x0E\x9E\xE1\x0F\x8B\xBC9\x98\xD3\nS:\xE8I\xE2\x83dsolcC\0\x08\x19\x003";
    /// The bytecode of the contract.
    pub static EXAMPLEROLLUP_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\0LW`\x005`\xE0\x1C\x80c\x03%q\xA9\x14a\0QW\x80cA,\xC8\xFE\x14a\0fW\x80c\xB5p\x0Eh\x14a\0\x82W\x80c\xD8\0t\x1E\x14a\0\xADW[`\0\x80\xFD[a\0da\0_6`\x04a\x02gV[a\0\xB6V[\0[a\0o`\x02T\x81V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[`\0Ta\0\x95\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\0yV[a\0o`\x01T\x81V[\x82g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\0\x03a\0\xE1W`@Qc\x0F\xD4\xB67`\xE3\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[\x81\x81``\x01Q\x14a\x01\x05W`@Qc\t\xBD\xE39`\xE0\x1B\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[`\0\x80`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\x9F\xDBT\xA7`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01```@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x01YW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x01}\x91\x90a\x03\nV[P\x91PP\x80g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x84g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x02Ta\x01\xA4\x91\x90a\x03MV[\x11\x15a\x01\xE6W`\x02T`@Qc\xF08Hg`\xE0\x1B\x81R`\x04\x81\x01\x91\x90\x91Rg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x86\x16`$\x83\x01R\x82\x16`D\x82\x01R`d\x01`@Q\x80\x91\x03\x90\xFD[\x83g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x02`\0\x82\x82Ta\x02\x02\x91\x90a\x03MV[\x90\x91UPP`\x01\x83\x90U`\x02T`@\x80Q\x91\x82R` \x82\x01\x85\x90R\x7F\x9C:SN\xC4A\xC7c:pv_T\xC0\x02L\xA8\xF9\xA7nz,\xDA\xAC*j\x8C5\x19\xC0\xCA\xF3\x91\x01`@Q\x80\x91\x03\x90\xA1PPPPV[g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x02dW`\0\x80\xFD[PV[`\0\x80`\0\x83\x85\x03`\xC0\x81\x12\x15a\x02}W`\0\x80\xFD[\x845a\x02\x88\x81a\x02NV[\x93P` \x85\x015\x92P`\x80`?\x19\x82\x01\x12\x15a\x02\xA3W`\0\x80\xFD[P`@Q`\x80\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a\x02\xD5WcNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[\x80`@RP`@\x85\x015\x81R``\x85\x015` \x82\x01R`\x80\x85\x015`@\x82\x01R`\xA0\x85\x015``\x82\x01R\x80\x91PP\x92P\x92P\x92V[`\0\x80`\0``\x84\x86\x03\x12\x15a\x03\x1FW`\0\x80\xFD[\x83Qa\x03*\x81a\x02NV[` \x85\x01Q\x90\x93Pa\x03;\x81a\x02NV[\x80\x92PP`@\x84\x01Q\x90P\x92P\x92P\x92V[\x80\x82\x01\x80\x82\x11\x15a\x03nWcNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[\x92\x91PPV\xFE\xA2dipfsX\"\x12 \xE2\r\xDD\xA5\xE9A\x1D\xD3\x8F,\x11\xAA\xCC\x81\n\xA7\x0E\x9E\xE1\x0F\x8B\xBC9\x98\xD3\nS:\xE8I\xE2\x83dsolcC\0\x08\x19\x003";
    /// The deployed bytecode of the contract.
    pub static EXAMPLEROLLUP_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
    pub struct ExampleRollup<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for ExampleRollup<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for ExampleRollup<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for ExampleRollup<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for ExampleRollup<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(ExampleRollup))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> ExampleRollup<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(::ethers::contract::Contract::new(
                address.into(),
                EXAMPLEROLLUP_ABI.clone(),
                client,
            ))
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
                EXAMPLEROLLUP_ABI.clone(),
                EXAMPLEROLLUP_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `lightClient` (0xb5700e68) function
        pub fn light_client(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([181, 112, 14, 104], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `numVerifiedBlocks` (0x412cc8fe) function
        pub fn num_verified_blocks(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([65, 44, 200, 254], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `stateCommitment` (0xd800741e) function
        pub fn state_commitment(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([216, 0, 116, 30], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `verifyBlocks` (0x032571a9) function
        pub fn verify_blocks(
            &self,
            count: u64,
            next_state_commitment: ::ethers::core::types::U256,
            proof: BatchProof,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([3, 37, 113, 169], (count, next_state_commitment, proof))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `StateUpdate` event
        pub fn state_update_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, StateUpdateFilter>
        {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, StateUpdateFilter>
        {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for ExampleRollup<M>
    {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `InvalidProof` with signature `InvalidProof()` and selector `0x09bde339`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "InvalidProof", abi = "InvalidProof()")]
    pub struct InvalidProof;
    ///Custom Error type `NoBlocks` with signature `NoBlocks()` and selector `0x7ea5b1b8`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(name = "NoBlocks", abi = "NoBlocks()")]
    pub struct NoBlocks;
    ///Custom Error type `NotYetSequenced` with signature `NotYetSequenced(uint256,uint64,uint256)` and selector `0xf0384867`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[etherror(
        name = "NotYetSequenced",
        abi = "NotYetSequenced(uint256,uint64,uint256)"
    )]
    pub struct NotYetSequenced {
        pub num_verified_blocks: ::ethers::core::types::U256,
        pub count: u64,
        pub block_height: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's custom errors
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        serde::Serialize,
        serde::Deserialize,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub enum ExampleRollupErrors {
        InvalidProof(InvalidProof),
        NoBlocks(NoBlocks),
        NotYetSequenced(NotYetSequenced),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for ExampleRollupErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) =
                <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded) = <InvalidProof as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InvalidProof(decoded));
            }
            if let Ok(decoded) = <NoBlocks as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::NoBlocks(decoded));
            }
            if let Ok(decoded) = <NotYetSequenced as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::NotYetSequenced(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ExampleRollupErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::InvalidProof(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::NoBlocks(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::NotYetSequenced(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for ExampleRollupErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector == <InvalidProof as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <NoBlocks as ::ethers::contract::EthError>::selector() => true,
                _ if selector == <NotYetSequenced as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for ExampleRollupErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::InvalidProof(element) => ::core::fmt::Display::fmt(element, f),
                Self::NoBlocks(element) => ::core::fmt::Display::fmt(element, f),
                Self::NotYetSequenced(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for ExampleRollupErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<InvalidProof> for ExampleRollupErrors {
        fn from(value: InvalidProof) -> Self {
            Self::InvalidProof(value)
        }
    }
    impl ::core::convert::From<NoBlocks> for ExampleRollupErrors {
        fn from(value: NoBlocks) -> Self {
            Self::NoBlocks(value)
        }
    }
    impl ::core::convert::From<NotYetSequenced> for ExampleRollupErrors {
        fn from(value: NotYetSequenced) -> Self {
            Self::NotYetSequenced(value)
        }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethevent(name = "StateUpdate", abi = "StateUpdate(uint256,uint256)")]
    pub struct StateUpdateFilter {
        pub block_height: ::ethers::core::types::U256,
        pub state_commitment: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `lightClient` function with signature `lightClient()` and selector `0xb5700e68`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "lightClient", abi = "lightClient()")]
    pub struct LightClientCall;
    ///Container type for all input parameters for the `numVerifiedBlocks` function with signature `numVerifiedBlocks()` and selector `0x412cc8fe`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "numVerifiedBlocks", abi = "numVerifiedBlocks()")]
    pub struct NumVerifiedBlocksCall;
    ///Container type for all input parameters for the `stateCommitment` function with signature `stateCommitment()` and selector `0xd800741e`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(name = "stateCommitment", abi = "stateCommitment()")]
    pub struct StateCommitmentCall;
    ///Container type for all input parameters for the `verifyBlocks` function with signature `verifyBlocks(uint64,uint256,(uint256,uint256,uint256,uint256))` and selector `0x032571a9`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    #[ethcall(
        name = "verifyBlocks",
        abi = "verifyBlocks(uint64,uint256,(uint256,uint256,uint256,uint256))"
    )]
    pub struct VerifyBlocksCall {
        pub count: u64,
        pub next_state_commitment: ::ethers::core::types::U256,
        pub proof: BatchProof,
    }
    ///Container type for all of the contract's call
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        serde::Serialize,
        serde::Deserialize,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub enum ExampleRollupCalls {
        LightClient(LightClientCall),
        NumVerifiedBlocks(NumVerifiedBlocksCall),
        StateCommitment(StateCommitmentCall),
        VerifyBlocks(VerifyBlocksCall),
    }
    impl ::ethers::core::abi::AbiDecode for ExampleRollupCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <LightClientCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::LightClient(decoded));
            }
            if let Ok(decoded) =
                <NumVerifiedBlocksCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::NumVerifiedBlocks(decoded));
            }
            if let Ok(decoded) =
                <StateCommitmentCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::StateCommitment(decoded));
            }
            if let Ok(decoded) = <VerifyBlocksCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::VerifyBlocks(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ExampleRollupCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::LightClient(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::NumVerifiedBlocks(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::StateCommitment(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::VerifyBlocks(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for ExampleRollupCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::LightClient(element) => ::core::fmt::Display::fmt(element, f),
                Self::NumVerifiedBlocks(element) => ::core::fmt::Display::fmt(element, f),
                Self::StateCommitment(element) => ::core::fmt::Display::fmt(element, f),
                Self::VerifyBlocks(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<LightClientCall> for ExampleRollupCalls {
        fn from(value: LightClientCall) -> Self {
            Self::LightClient(value)
        }
    }
    impl ::core::convert::From<NumVerifiedBlocksCall> for ExampleRollupCalls {
        fn from(value: NumVerifiedBlocksCall) -> Self {
            Self::NumVerifiedBlocks(value)
        }
    }
    impl ::core::convert::From<StateCommitmentCall> for ExampleRollupCalls {
        fn from(value: StateCommitmentCall) -> Self {
            Self::StateCommitment(value)
        }
    }
    impl ::core::convert::From<VerifyBlocksCall> for ExampleRollupCalls {
        fn from(value: VerifyBlocksCall) -> Self {
            Self::VerifyBlocks(value)
        }
    }
    ///Container type for all return fields from the `lightClient` function with signature `lightClient()` and selector `0xb5700e68`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct LightClientReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `numVerifiedBlocks` function with signature `numVerifiedBlocks()` and selector `0x412cc8fe`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct NumVerifiedBlocksReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `stateCommitment` function with signature `stateCommitment()` and selector `0xd800741e`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct StateCommitmentReturn(pub ::ethers::core::types::U256);
    ///`BatchProof(uint256,uint256,uint256,uint256)`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash,
    )]
    pub struct BatchProof {
        pub first_block: ::ethers::core::types::U256,
        pub last_block: ::ethers::core::types::U256,
        pub old_state: ::ethers::core::types::U256,
        pub new_state: ::ethers::core::types::U256,
    }
}

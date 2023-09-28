use anyhow::anyhow;
pub struct EvmRelayerInner {}

impl EvmRelayerInner {
    async fn start(
        &mut self, // The private-public key pair of this authority.
                   // keypair: KeyPair,
                   // // The private-public network key pair of this authority.
                   // network_keypair: NetworkKeyPair,
                   // // The committee information.
                   // committee: Committee,
                   // // The worker information cache.
                   // worker_cache: WorkerCache,
                   // // Client for communications.
                   // client: NetworkClient,
                   // // The state used by the client to execute transactions.
                   // execution_state: Arc<State>,
                   // tx_external_message: UnboundedSender<ExternalMessage>,
                   // rx_scalar_transaction: Arc<Mutex<UnboundedReceiver<Vec<ScalarEventTransaction>>>>,
    ) -> Result<(), anyhow::Error> {
        if self.is_running().await {
            return Err(anyhow!("Relayer already running"));
        }
        self.own_peer_id = Some(PeerId(network_keypair.public().0.to_bytes()));

        // create a new registry
        let registry = Registry::new_custom(None, None).ok();

        // create the channel to send the shutdown signal
        let mut tx_shutdown = PreSubscribedBroadcastSender::new(NUM_SHUTDOWN_RECEIVERS);

        // spawn relayer if not already running
        let handles = Self::spawn(
            keypair,
            network_keypair,
            committee,
            worker_cache,
            client,
            self.parameters.clone(),
            execution_state,
            &registry.as_ref().unwrap(),
            &mut tx_shutdown,
            tx_external_message,
            rx_scalar_transaction,
        )
        .await?;
        // store the registry
        self.swap_registry(registry);
        // now keep the handlers
        self.handles.clear();
        self.handles.extend(handles);
        self.tx_shutdown = Some(tx_shutdown);

        Ok(())
    }
    // Will shutdown the primary node and wait until the node has shutdown by waiting on the
    // underlying components handles. If the node was not already running then the
    // method will return immediately.
    #[instrument(level = "info", skip_all)]
    async fn shutdown(&mut self) {
        if !self.is_running().await {
            return;
        }

        // send the shutdown signal to the node
        let now = Instant::now();
        info!("Sending shutdown message to relayer node");

        // if let Some(c) = self.client.take() {
        //     c.shutdown();
        // }

        if let Some(tx_shutdown) = self.tx_shutdown.as_ref() {
            tx_shutdown
                .send()
                .expect("Couldn't send the shutdown signal to downstream components");
            self.tx_shutdown = None
        }

        // Now wait until handles have been completed
        try_join_all(&mut self.handles).await.unwrap();

        self.swap_registry(None);

        info!(
            "Narwhal EVM Relayer Node shutdown is complete - took {} seconds",
            now.elapsed().as_secs_f64()
        );
    }
    // Helper method useful to wait on the execution of the primary node
    async fn wait(&mut self) {
        try_join_all(&mut self.handles).await.unwrap();
    }
    // If any of the underlying handles haven't still finished, then this method will return
    // true, otherwise false will returned instead.
    async fn is_running(&self) -> bool {
        self.handles.iter().any(|h| !h.is_finished())
    }
    // Accepts an Option registry. If it's Some, then the new registry will be added in the
    // registry service and the registry_id will be updated. Also, any previous registry will
    // be removed. If None is passed, then the registry_id is updated to None and any old
    // registry is removed from the RegistryService.
    fn swap_registry(&mut self, registry: Option<Registry>) {
        if let Some((registry_id, _registry)) = self.registry.as_ref() {
            self.registry_service.remove(*registry_id);
        }

        if let Some(registry) = registry {
            self.registry = Some((self.registry_service.add(registry.clone()), registry));
        } else {
            self.registry = None
        }
    }
    pub async fn spawn<State>(
        // The private-public key pair of this authority.
        keypair: KeyPair,
        // The private-public network key pair of this authority.
        network_keypair: NetworkKeyPair,
        // The committee information.
        committee: Committee,
        // The worker information cache.
        worker_cache: WorkerCache,
        // Client for communications.
        client: NetworkClient,
        // The configuration parameterspubkey.
        parameters: Parameters,
        // The state used by the client to execute transactions.
        execution_state: Arc<State>,
        // A prometheus exporter Registry to use for the metrics
        registry: &Registry,
        // The channel to send the shutdown signal
        tx_shutdown: &mut PreSubscribedBroadcastSender,
        tx_external_message: UnboundedSender<ExternalMessage>,
        rx_scalar_transaction: Arc<Mutex<UnboundedReceiver<Vec<ScalarEventTransaction>>>>,
    ) -> Result<Vec<JoinHandle<()>>, anyhow::Error>
    where
        State: ExecutionState + Send + Sync + 'static,
    {
        let mut handles = Vec::new();
        // Compute the public key of this authority.
        let name = keypair.public().clone();
        // Figure out the id for this authority
        let authority = committee
            .authority_by_key(&name)
            .unwrap_or_else(|| panic!("Our node with key {:?} should be in committee", &name));
        // let mut rx_shutdown = tx_shutdown.subscribe();

        let relayer_config_dir =
            std::env::var("CONFIG_DIR").unwrap_or_else(|_| String::from("/opt/sui/config"));
        let config_path = format!(
            "{}/evm_relayer{}.toml",
            relayer_config_dir,
            authority.id().0
        );
        let config_str = fs::read_to_string(config_path.as_str())
            .map_err(|e| {
                let msg = format!("{:?}", e);
                println!("{}", msg.as_str());
                anyhow!(msg)
            })
            .expect(format!("Failed to read relayer config file {}", config_path).as_str());
        let relayer_configs: RelayerConfigs = toml::from_str(config_str.as_str()).unwrap();
        let narwhal_client = Arc::new(Mutex::new(AnemoClient::new(
            keypair,
            network_keypair,
            committee,
            worker_cache,
        )));
        let mut rx_shutdown = tx_shutdown.subscribe();
        let out_transaction_handle = tokio::spawn(async move {
            let mut rx_transaction = rx_scalar_transaction.lock().await;
            //Todo: add router for routing output transaction to destination chain
            // MVP level: Static evm client

            loop {
                tokio::select! {
                    _ = rx_shutdown.receiver.recv() => {
                        warn!("EVM Relayer Node is shuting down");
                        break;
                    },
                    Some(trans) = rx_transaction.recv() => {
                        info!("EVM Relayer received transaction {:?}", &trans);
                        //Call to evm client only
                    }
                }
            }
        });
        handles.push(out_transaction_handle);
        //info!("Evm relayer config {}", config_str.as_str());
        let mut ws_handles = relayer_configs
            .scalar_relayer_evm
            .into_iter()
            .map(|config| {
                let mut rx_shutdown = tx_shutdown.subscribe();
                // Anemo client
                //let anemo_client = narwhal_client.clone();
                let anemo_client = client.clone();
                let tx = tx_external_message.clone();
                tokio::spawn(async move {
                    // A Ws provider can be created from a ws(s) URI.
                    // In case of wss you must add the "rustls" or "openssl" feature
                    // to the ethers library dependency in `Cargo.toml`
                    // Axelar simulation chains do not support websocket connection
                    // if let (Some(ws_url), Some(start)) = (config.ws_addr, config.start_with_bridge)
                    // {
                    //     if start {
                    //         let provider = Provider::<Ws>::connect(ws_url.as_str()).await.expect(
                    //             format!("Cannot connect to websocket url {:?}", ws_url.as_str())
                    //                 .as_str(),
                    //         );
                    //         info!("Connected to websocket {:?} successfully", ws_url.as_str());
                    //         let mut stream =
                    //             provider.subscribe_blocks().await.expect("Cannot subscribe");
                    //         let stream_id = stream.id;
                    //         loop {
                    //             tokio::select! {
                    //                 _ = rx_shutdown.receiver.recv() => {
                    //                     warn!("EVM Relayer Node is shuting down");
                    //                     break;
                    //                 },
                    //                 block = stream.next() => {
                    //                     //Received event from source chain
                    //                     //Broadcast a poll
                    //                     //Send it to the worker for create poll
                    //                     match block {
                    //                         Some(block) => {
                    //                             info!("Received evm block {:?}", &block.hash);
                    //                             // let hash = block.hash.clone();
                    //                             let external_message = ExternalMessage::new(block);
                    //                             tx.send(external_message);
                    //                             //anemo_client.lock().await.broadcast_block(block).await;
                    //                         },
                    //                         None => {
                    //                             info!("Data from stream is unavailable");
                    //                         },
                    //                     }
                    //                 }
                    //             }
                    //         }
                    //         let _ = provider.unsubscribe(stream_id);
                    //     }
                    // }
                    //https://github.com/gakonst/ethers-rs/blob/master/examples/contracts/examples/abigen.rs
                    if let (Some(url), Some(contract_addr), Some(start)) = (
                        config.rpc_addr,
                        config.contract_addr,
                        config.start_with_bridge,
                    ) {
                        if start {
                            let provider = Provider::<Http>::try_from(url.as_str()).expect(
                                format!("Cannot connect to rpc url {:?}", url.as_str()).as_str(),
                            );
                            info!("Connected to rpc {:?} successfully", url.as_str());
                            let client = Arc::new(provider);
                            let address: Address = contract_addr.parse()?;
                            let contract = ExecutableSample::new(address, client);
                            let mut stream =
                                provider.subscribe_blocks().await.expect("Cannot subscribe");
                            let stream_id = stream.id;
                            loop {
                                tokio::select! {
                                    _ = rx_shutdown.receiver.recv() => {
                                        warn!("EVM Relayer Node is shuting down");
                                        break;
                                    },
                                    block = stream.next() => {
                                        //Received event from source chain
                                        //Broadcast a poll
                                        //Send it to the worker for create poll
                                        match block {
                                            Some(block) => {
                                                info!("Received evm block {:?}", &block.hash);
                                                // let hash = block.hash.clone();
                                                let external_message = ExternalMessage::new(block);
                                                tx.send(external_message);
                                                //anemo_client.lock().await.broadcast_block(block).await;
                                            },
                                            None => {
                                                info!("Data from stream is unavailable");
                                            },
                                        }
                                    }
                                }
                            }
                            info!("Stop listen event from external chain");
                        }
                    }
                })
            });
        handles.extend(&mut ws_handles);
        Ok(handles)
    }
}
pub struct EvmRelayer {
    internal: Arc<RwLock<EvmRelayerInner>>,
}

impl EvmRelayer {
    pub fn new(parameters: Parameters, registry_service: RegistryService) -> Self {
        let inner = EvmRelayerInner {
            parameters,
            registry_service,
            registry: None,
            handles: FuturesUnordered::new(),
            tx_shutdown: None,
            own_peer_id: None,
        };

        Self {
            internal: Arc::new(RwLock::new(inner)),
        }
    }
    #[instrument(level = "info", skip_all)]
    pub async fn start<State>(
        &self, // The private-public key pair of this authority.
        keypair: KeyPair,
        // The private-public network key pair of this authority.
        network_keypair: NetworkKeyPair,
        // The committee information.
        committee: Committee,
        // The worker information cache.
        worker_cache: WorkerCache,
        // Client for communications.
        client: NetworkClient,
        // The state used by the client to execute transactions.
        execution_state: Arc<State>,
        tx_external_message: Option<UnboundedSender<ExternalMessage>>,
        // ConsensusOutTransaction channel
        rx_scalar_transaction: Arc<Mutex<UnboundedReceiver<Vec<ScalarEventTransaction>>>>,
    ) -> Result<(), anyhow::Error>
    where
        State: ExecutionState + Send + Sync + 'static,
    {
        let mut guard = self.internal.write().await;
        guard
            .start(
                keypair,
                network_keypair,
                committee,
                worker_cache,
                client,
                execution_state,
                tx_external_message.unwrap(),
                rx_scalar_transaction,
            )
            .await
    }
    pub async fn shutdown(&self) {
        let mut guard = self.internal.write().await;
        guard.shutdown().await
    }

    pub async fn is_running(&self) -> bool {
        let guard = self.internal.read().await;
        guard.is_running().await
    }

    pub async fn wait(&self) {
        let mut guard = self.internal.write().await;
        guard.wait().await
    }

    pub async fn registry(&self) -> Option<(RegistryID, Registry)> {
        let guard = self.internal.read().await;
        guard.registry.clone()
    }
}

impl Relayer for EvmRelayer {}

fn match_for_io_error(err_status: &Status) -> Option<&std::io::Error> {
    let mut err: &(dyn Error + 'static) = err_status;

    loop {
        if let Some(io_err) = err.downcast_ref::<std::io::Error>() {
            return Some(io_err);
        }

        // h2::Error do not expose std::io::Error with `source()`
        // https://github.com/hyperium/h2/pull/462
        if let Some(h2_err) = err.downcast_ref::<h2::Error>() {
            if let Some(io_err) = h2_err.get_io() {
                return Some(io_err);
            }
        }

        err = match err.source() {
            Some(err) => err,
            None => return None,
        };
    }
}

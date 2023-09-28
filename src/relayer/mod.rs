//mod evm;
//pub use evm::*;
use crate::abis::{executable_sample, message_receiver, message_sender};
use crate::proto::{scalar_abci_client::ScalarAbciClient, ScalarAbciRequest};
use ethers::prelude::*;
use serde::Deserialize;
use std::sync::Arc;
use std::{thread, time};
use tokio::sync::mpsc::{self, UnboundedSender};
use tonic::transport::Channel;
use tracing::info;
pub const NAMESPACE: &str = "scalar";

#[derive(Clone, Debug, Deserialize)]
pub struct RelayerConfigs {
    pub relayer_evm: Vec<RelayerConfig>,
}
/// This is what we're going to decode into. Each field is optional, meaning
/// that it doesn't have to be present in TOML.
#[derive(Clone, Debug, Deserialize)]
pub struct RelayerConfig {
    pub name: String,
    pub call_contract: Option<String>,
    pub rpc_addr: Option<String>,
    pub ws_addr: Option<String>,
    pub start_with_bridge: Option<bool>,
}

pub trait Relayer {}
pub async fn start(
    config: RelayerConfig,
    mut grpc_client: ScalarAbciClient<Channel>,
    tx: UnboundedSender<String>,
) -> anyhow::Result<()> {
    //let handle = tokio::spawn(async move {});
    if let (Some(url), Some(call_contract)) =
        (config.rpc_addr.as_ref(), config.call_contract.as_ref())
    {
        let provider = Provider::<Http>::try_from(url.as_str())
            .expect(format!("Cannot connect to rpc url {:?}", url.as_str()).as_str());
        info!("Connected to chain rpc {:?} successfully", url.as_str());
        let client = Arc::new(provider);
        let address: Address = call_contract.parse()?;
        info!("Call contract {:?}", &address);
        let (tx_external_event, mut rx_external_event) = mpsc::unbounded_channel::<String>();
        let message_reciver = message_receiver::MessageReceiver::new(address, client.clone());
        // let timer = tokio::time::sleep(Duration::from_millis(2_000));
        // tokio::pin!(timer);
        let sleep_duration = time::Duration::from_millis(2_000);
        let message_call = message_reciver.message();
        let _grpc_handle = tokio::spawn(async move {
            let in_stream = async_stream::stream! {
                while let Some(item) = rx_external_event.recv().await {
                    //let value = serde_json::to_string(&item).unwrap();
                    //println!("Push block into stream {:?}", &value);
                    info!("Receive message {:?}", item);
                    yield ScalarAbciRequest{
                        namespace: String::from(NAMESPACE),
                        message: item.as_bytes().to_vec(),
                    };
                }
            };
            let response = grpc_client
                .bidirectional_streaming_scalar_abci(in_stream)
                .await
                .unwrap();

            let mut resp_stream = response.into_inner();

            while let Some(received) = resp_stream.next().await {
                let received = received.unwrap();
                let out_message = String::from_utf8(received.message).unwrap();
                info!("\treceived message: `{}`", &out_message);
                let _ = tx.send(out_message);
            }
        });
        loop {
            match message_call.call().await {
                Ok(msg) => {
                    info!("Message call result from chain {}, {:?}", &config.name, msg);
                    let _ = tx_external_event.send(msg);
                }
                Err(_) => {}
            }
            thread::sleep(sleep_duration);
        }
        //info!("Stop listen event from external chain");
    }
    Ok(())
}

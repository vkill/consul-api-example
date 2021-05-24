use std::error;

use consul_api::{
    api::catalog::{DatacentersEndpoint, NodesEndpoint},
    Client as _, IsahcClient,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    let mut client = IsahcClient::new().unwrap();
    client.set_base_url("http://127.0.0.1:8500/");

    let endpoint = DatacentersEndpoint::new();
    let res = client.respond_endpoint(&endpoint).await?;
    println!("datacenters {:?}", res);

    let endpoint = NodesEndpoint::new();
    let res = client.respond_endpoint(&endpoint).await?;
    println!("nodes {:?}", res);

    Ok(())
}

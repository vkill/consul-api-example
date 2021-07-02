use std::error;

use consul_api::{
    api::catalog::{ListDatacenters, ListNodes},
    Client as _, IsahcClient,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    let mut client = IsahcClient::new().unwrap();
    client.set_base_url("http://127.0.0.1:8500/");

    let endpoint = ListDatacenters::new();
    let res = client.respond_endpoint(&endpoint).await?;
    println!("datacenters {:?}", res);

    let endpoint = ListNodes::new();
    let res = client.respond_endpoint(&endpoint).await?;
    println!("nodes {:?}", res);

    Ok(())
}

use aws_config::meta::region::RegionProviderChain;
use aws_sdk_dynamodb::{Client, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    // defines our region
    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
    // defines our config from env variables
    let config = aws_config::from_env().region(region_provider).load().await;
    // new client instance
    let client = Client::new(&config); // passing config by reference
    // variable representing dynamodb result
    let resp = client.list_tables().send().await?;
    
    println!("Tables:"); 

    let names = resp.table_names().unwrap_or_default(); 

    for name in names {
        println!(" {}", name); 
    }

    println!("Found {} tables", names.len()); 

    // return value of type Result<>
    Ok(())
}

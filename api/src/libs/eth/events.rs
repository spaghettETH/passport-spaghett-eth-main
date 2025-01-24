use ethers::abi::Address;
use ethers::contract::abigen;
use ethers::prelude::*;
use ethers::providers::{Http, Provider};
extern crate dotenv;
use dotenv::dotenv;
use ethers::abi::ParamType;
use ethers::types::Filter;
use rocket::response::Debug;
use std::convert::TryFrom;
use std::env;
use std::str::FromStr;

pub async fn get_passport_set_events() -> Result<Vec<(u64, Address, String, String, String)>, Debug<String>> {
    dotenv().ok();
    // Read environment variables
    let provider_url = &env::var("ETH_PROVIDER_URL").unwrap();
    let contract_address = &env::var("RESOLVER_CONTRACT").unwrap();
    // Create provider instance from provider url
    let provider = Provider::<Http>::try_from(provider_url.as_str());
    if provider.is_err() {
        return Err(Debug::from(provider.unwrap_err().to_string()));
    }
    let provider = provider.unwrap();
    abigen!(SpaghettEthResolver, "./src/libs/eth/abis/resolver.json");
    let contract_address = Address::from_str(contract_address.as_str());
    let event_signature = "PassportSet(address,string,string,string)";
    let topic0 = H256::from(&ethers::utils::keccak256(event_signature.as_bytes()));

    // Read last block from provider
    let last_block = provider.get_block(BlockNumber::Latest).await.unwrap();
    let from_block = BlockNumber::from(last_block.clone().unwrap().number.unwrap() - 1000);
 
    let filter = Filter::new()
        .address(contract_address.unwrap())
        .topic0(topic0)
        .from_block(from_block)
        .to_block(BlockNumber::from(last_block.unwrap().number.unwrap()));

    let logs = provider.get_logs(&filter).await.unwrap();

    let events: Vec<(u64, Address, String, String, String)> = logs
        .into_iter()
        .filter_map(|log| {
            if log.topics.len() == 2 {
                let block_number = log.block_number?.as_u64();
                let address = Address::from_slice(&log.topics[1].as_fixed_bytes()[12..]);
                
                // Decode all three string parameters at once
                let decoded = ethers::abi::decode(
                    &[ParamType::String, ParamType::String, ParamType::String],
                    &log.data
                ).ok()?;

                let name = decoded[0].clone().into_string()?;
                let description = decoded[1].clone().into_string()?;
                let image = decoded[2].clone().into_string()?;

                Some((block_number, address, name, description, image))
            } else {
                None
            }
        })
        .collect();

    Ok(events)
}

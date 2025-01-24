use ethers::abi::Address;
use ethers::contract::abigen;
use ethers::prelude::*;
use ethers::providers::{Http, Middleware, Provider};
use ethers::signers::{LocalWallet, Signer};
use ethers::types::Signature;
extern crate dotenv;
use rocket::response::Debug;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use std::{convert::TryFrom, sync::Arc};

use dotenv::dotenv;
use std::env;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Profile {
    pub name: String,
    pub description: String,
    pub image: String,
}

#[allow(dead_code)]
fn get_token_id(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

#[allow(dead_code)]
pub async fn get_address(key: String) -> Result<String, Debug<String>> {
    // Create wallet instance from private key
    let wallet = LocalWallet::from_str(&key);
    // Return address
    Ok(format!("{:#x}", wallet.unwrap().address()))
}

#[allow(dead_code)]
pub async fn get_last_block() -> Result<u64, Debug<String>> {
    dotenv().ok();
    // Read provider url from environment variable
    let provider_url = &env::var("ETH_PROVIDER_URL").unwrap();
    // Create provider instance from provider url
    let provider = Provider::<Http>::try_from(provider_url.as_str());
    if provider.is_err() {
        return Err(Debug::from(provider.unwrap_err().to_string()));
    }
    // Get last block number
    let last_block = provider.unwrap().get_block_number().await;
    if last_block.is_err() {
        return Err(Debug::from(last_block.unwrap_err().to_string()));
    }
    // Return last block number
    Ok(last_block.unwrap().as_u64())
}

#[allow(dead_code)]
pub async fn get_gas_price(safe: bool) -> Result<U256, Debug<String>> {
    dotenv().ok();
    // Read provider url from environment variable
    let provider_url = &env::var("ETH_PROVIDER_URL").unwrap();
    // Create provider instance from provider url
    let provider = Provider::<Http>::try_from(provider_url.as_str());
    if provider.is_err() {
        return Err(Debug::from(provider.unwrap_err().to_string()));
    }
    // Get gas price
    let gas_price = provider.clone().unwrap().get_gas_price().await;
    if gas_price.is_err() {
        return Err(Debug::from(gas_price.unwrap_err().to_string()));
    }
    // Return gas price
    if safe {
        let boost = gas_price.unwrap().checked_mul(U256::from(2));
        println!("🧨 Gas price boosted: {}", boost.unwrap());
        return Ok(boost.unwrap());
    }
    Ok(gas_price.unwrap())
}

#[allow(dead_code)]
pub async fn get_transaction_count(address: String) -> Result<U256, Debug<String>> {
    dotenv().ok();
    // Read provider url from environment variable
    let provider_url = &env::var("ETH_PROVIDER_URL").unwrap();
    // Create provider instance from provider url
    let provider = Provider::<Http>::try_from(provider_url.as_str());
    if provider.is_err() {
        return Err(Debug::from(provider.unwrap_err().to_string()));
    }
    // Get address's transaction count
    let typed_address = Address::from_str(&address);
    let last_nonce = provider
        .clone()
        .unwrap()
        .get_transaction_count(typed_address.unwrap(), None)
        .await;
    if last_nonce.is_err() {
        return Err(Debug::from(last_nonce.unwrap_err().to_string()));
    }
    Ok(last_nonce.unwrap())
}

#[allow(dead_code)]
pub async fn verify_signature(
    message: &str,
    signature: &str,
    address: &str,
) -> Result<String, Debug<String>> {
    // Recreate signature from signature string
    let sig: Signature = Signature::from_str(signature).unwrap();
    // Create address from address string
    let addr = Address::from_str(address);
    if addr.is_err() {
        return Err(Debug::from(addr.unwrap_err().to_string()));
    }
    // Verify signature
    let verified = sig.verify(message, addr.unwrap());
    if verified.is_err() {
        return Err(Debug::from(verified.unwrap_err().to_string()));
    }
    // Return verified signature
    let formatted = format!("{:#x}", addr.unwrap());
    Ok(formatted)
}

#[allow(dead_code)]
pub async fn get_token_uri(id: String) -> Result<String, Debug<String>> {
    dotenv().ok();
    // Read provider url from environment variable
    let provider_url = &env::var("ETH_PROVIDER_URL").unwrap();
    // Create provider instance from provider url
    let provider = Provider::<Http>::try_from(provider_url.as_str());
    if provider.is_err() {
        return Err(Debug::from(provider.unwrap_err().to_string()));
    }
    abigen!(SpaghettEthPassport, "./src/libs/eth/abis/passport.json");
    // Read private key from environment variable
    let private_key = &env::var("ETH_READER_KEY").unwrap();
    // Create wallet instance from private key
    let wallet = private_key.parse::<LocalWallet>();
    if wallet.is_err() {
        return Err(Debug::from(wallet.unwrap_err().to_string()));
    }
    let signer = Arc::new(SignerMiddleware::new(
        provider.unwrap().clone(),
        wallet.unwrap().to_owned(),
    ));
    // Create contract instance
    let contract_address = Address::from_str(&env::var("PASSPORT_CONTRACT").unwrap());
    let contract = SpaghettEthPassport::new(contract_address.unwrap(), signer);
    // Call contract function
    let token_id = get_token_id(id);
    let token_uri = contract.token_uri(U256::from(token_id)).await;
    if token_uri.is_err() {
        return Err(Debug::from(token_uri.unwrap_err().to_string()));
    }
    // Return token uri
    Ok(token_uri.unwrap())
}

pub async fn balance_of(address: String) -> Result<u32, Debug<String>> {
    dotenv().ok();
    // Read provider url from environment variable
    let provider_url = &env::var("ETH_PROVIDER_URL").unwrap();
    // Create provider instance from provider url
    let provider = Provider::<Http>::try_from(provider_url.as_str());
    if provider.is_err() {
        return Err(Debug::from(provider.unwrap_err().to_string()));
    }

    abigen!(SpaghettEthPassport, "./src/libs/eth/abis/passport.json");
    // Read private key from environment variable
    let private_key = &env::var("ETH_READER_KEY").unwrap();
    // Create wallet instance from private key
    let wallet = private_key.parse::<LocalWallet>();
    if wallet.is_err() {
        return Err(Debug::from(wallet.unwrap_err().to_string()));
    }
    let signer = Arc::new(SignerMiddleware::new(
        provider.unwrap().clone(),
        wallet.unwrap().to_owned(),
    ));
    // Create contract instance
    let contract_address = Address::from_str(&env::var("PASSPORT_CONTRACT").unwrap());
    let contract = SpaghettEthPassport::new(contract_address.unwrap(), signer);
    // Call contract function
    let owner_address = Address::from_str(&address);
    let balance = contract.balance_of(owner_address.unwrap()).await;
    if balance.is_err() {
        return Err(Debug::from(balance.unwrap_err().to_string()));
    }
    // Return balance
    Ok(balance.unwrap().as_u32())
}


pub async fn get_passport_profile(address: String) -> Result<Profile, Debug<String>> {
    dotenv().ok();
    // Read provider url from environment variable
    let provider_url = &env::var("ETH_PROVIDER_URL").unwrap();
    // Create provider instance from provider url
    let provider = Provider::<Http>::try_from(provider_url.as_str());
    if provider.is_err() {
        return Err(Debug::from(provider.unwrap_err().to_string()));
    }

    abigen!(SpaghettEthResolver, "./src/libs/eth/abis/resolver.json");
    // Read private key from environment variable
    let private_key = &env::var("ETH_READER_KEY").unwrap();
    // Create wallet instance from private key
    let wallet = private_key.parse::<LocalWallet>();
    if wallet.is_err() {
        return Err(Debug::from(wallet.unwrap_err().to_string()));
    }
    let signer = Arc::new(SignerMiddleware::new(
        provider.unwrap().clone(),
        wallet.unwrap().to_owned(),
    ));
    // Create contract instance
    let contract_address = Address::from_str(&env::var("RESOLVER_CONTRACT").unwrap());
    let contract = SpaghettEthResolver::new(contract_address.unwrap(), signer);
    // Call contract function
    let owner_address = Address::from_str(&address);
    let name = contract.names(owner_address.unwrap()).await;
    if name.is_err() {
        return Err(Debug::from(name.unwrap_err().to_string()));
    }
    let description = contract.descriptions(owner_address.unwrap()).await;
    if description.is_err() {
        return Err(Debug::from(description.unwrap_err().to_string()));
    }
    let image = contract.images(owner_address.unwrap()).await;
    if image.is_err() {
        return Err(Debug::from(image.unwrap_err().to_string()));
    }
    // Return balance
    Ok(Profile {
        name: name.unwrap(),
        description: description.unwrap(),
        image: image.unwrap(),
    })
}

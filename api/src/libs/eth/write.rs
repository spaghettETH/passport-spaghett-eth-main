use ethers::abi::Address;
use ethers::contract::abigen;
use ethers::prelude::*;
use ethers::providers::{Http, Provider};
use ethers::signers::{LocalWallet, Signer};
extern crate dotenv;
use dotenv::dotenv;
use ethers::types::U256;
use mongodb::Database;
use rocket::response::Debug;
use rocket::State;
use std::env;
use std::ops::Add;
use std::str::FromStr;
use std::{convert::TryFrom, sync::Arc};

#[allow(dead_code)]
pub async fn sign_message(message: &str) -> Result<String, Debug<String>> {
    dotenv().ok();
    // Read private key from environment variable
    let private_key = &env::var("ETH_ADMIN_KEY").unwrap();
    // Create wallet instance from private key
    let wallet = private_key.parse::<LocalWallet>();
    if wallet.is_err() {
        return Err(Debug::from(wallet.unwrap_err().to_string()));
    }
    // Sign message with provided wallet
    let signature = wallet.unwrap().to_owned().sign_message(message).await;
    if signature.is_err() {
        return Err(Debug::from(signature.unwrap_err().to_string()));
    }
    // Return signature
    Ok("0x".to_owned() + &signature.unwrap().to_string())
}

#[allow(dead_code)]
pub async fn sign_checkpoint(
    db: &State<Database>,
    checkpoint_id: U256,
    receiver: String,
) -> Result<String, Debug<String>> {
    dotenv().ok();
    // Read environment variables
    println!("Checkpoint ID: {}", checkpoint_id.clone());
    println!("Receiver: {}", receiver.clone());
    let provider_url = &env::var("ETH_PROVIDER_URL").unwrap();
    let contract_address = &env::var("PASSPORT_CONTRACT").unwrap();
    // Create provider instance from provider url
    let provider = Provider::<Http>::try_from(provider_url.as_str());
    if provider.is_err() {
        return Err(Debug::from(provider.unwrap_err().to_string()));
    }
    abigen!(SpaghettEthPassport, "./src/libs/eth/abis/passport.json");
    // Read private key from environment variable
    let private_key = &env::var("ETH_SIGNER_KEY").unwrap();
    // Create wallet instance from private key
    let wallet = private_key.parse::<LocalWallet>();
    if wallet.is_err() {
        return Err(Debug::from(wallet.unwrap_err().to_string()));
    }
    let signer = Arc::new(SignerMiddleware::new(
        provider.clone().unwrap(),
        wallet.unwrap().to_owned().with_chain_id(
            provider
                .clone()
                .unwrap()
                .get_chainid()
                .await
                .unwrap()
                .as_u64(),
        ),
    ));
    // Create contract instance
    let contract_address = Address::from_str(contract_address.as_str());
    let contract = SpaghettEthPassport::new(contract_address.unwrap(), signer);
    // Call contract function
    let minter_address = crate::libs::eth::read::get_address(private_key.to_string()).await;
    if minter_address.is_err() {
        return Err(Debug::from("😵 Can't get minter address".to_string()));
    }
    let minter_address_string = minter_address.unwrap().to_string();
    // Get boosted gas price
    let gas_price = crate::libs::eth::read::get_gas_price(true).await;
    if !gas_price.is_ok() {
        return Err(Debug::from("😵 Can't get gas price".to_string()));
    }
    // Get last nonce
    let nonce = crate::libs::eth::read::get_transaction_count(minter_address_string.clone()).await;
    if !nonce.is_ok() {
        return Err(Debug::from("😵 Can't get nonce".to_string()));
    }
    println!("🔎 Checking if nonce was used..");
    let mut nonce256 = nonce.unwrap();
    println!("🔄 Checking nonce {}..", nonce256.clone());
    // Check in nonce was used
    let mut check = crate::libs::eth::db::is_nonce_valid(
        db,
        minter_address_string.clone(),
        nonce256.clone().to_string(),
    )
    .await;
    if !check.clone().unwrap() {
        while !check.clone().unwrap() {
            nonce256 = nonce256.add(1);
            check = crate::libs::eth::db::is_nonce_valid(
                db,
                minter_address_string.clone(),
                nonce256.clone().to_string(),
            )
            .await;
            println!("✅ Is nonce valid? {}", check.clone().unwrap());
        }
    }
    println!("✅ Nonce {} confirmed!", nonce256.clone());
    // Create transaction
    let to_address = Address::from_str(&receiver);
    let tx = contract.sign_checkpoint(checkpoint_id, to_address.unwrap());
    // Send transaction
    let to_be_sent = tx.gas_price(gas_price.unwrap()).nonce(nonce256.clone());
    println!("🚀 Sending transaction..");
    let pending_tx = to_be_sent.send().await;
    if pending_tx.is_err() {
        return Err(Debug::from(pending_tx.unwrap_err().to_string()));
    }
    let pending_tx_hash = pending_tx.as_ref().unwrap().tx_hash();
    println!(
        "✅ Transaction pending with hash: {}",
        format!("{:#x}", pending_tx_hash)
    );
    let stored_txid = crate::libs::eth::db::insert_tx(
        db,
        minter_address_string.clone(),
        nonce256.clone().to_string(),
        format!("{:#x}", pending_tx_hash),
        receiver.clone(),
    )
    .await;
    if stored_txid.is_err() {
        println!("😵 Can't store pending transaction");
    }
    println!("⏳ Waiting for transaction..");
    let mined_tx = pending_tx.unwrap().await;
    if mined_tx.is_err() {
        return Err(Debug::from(mined_tx.unwrap_err().to_string()));
    }
    // Return transaction receipt
    Ok(format!("{:#x}", pending_tx_hash))
}

pub async fn create_checkpoint(
    db: &State<Database>,
    name: String,
    description: String,
    image: String,
    slug: String,
    timestamp_start: U256,
    timestamp_end: U256,
) -> Result<String, Debug<String>> {
    dotenv().ok();
    // Read environment variables
    let provider_url = &env::var("ETH_PROVIDER_URL").unwrap();
    let contract_address = &env::var("PASSPORT_CONTRACT").unwrap();
    // Create provider instance from provider url
    let provider = Provider::<Http>::try_from(provider_url.as_str());
    if provider.is_err() {
        return Err(Debug::from(provider.unwrap_err().to_string()));
    }
    abigen!(SpaghettEthPassport, "./src/libs/eth/abis/passport.json");
    // Read private key from environment variable
    let private_key = &env::var("ETH_ADMIN_KEY").unwrap();
    // Create wallet instance from private key
    let wallet = private_key.parse::<LocalWallet>();
    if wallet.is_err() {
        return Err(Debug::from(wallet.unwrap_err().to_string()));
    }
    let signer = Arc::new(SignerMiddleware::new(
        provider.clone().unwrap(),
        wallet.unwrap().to_owned().with_chain_id(
            provider
                .clone()
                .unwrap()
                .get_chainid()
                .await
                .unwrap()
                .as_u64(),
        ),
    ));
    // Create contract instance
    let contract_address = Address::from_str(contract_address.as_str());
    let contract = SpaghettEthPassport::new(contract_address.unwrap(), signer);
    // Call contract function
    let minter_address = crate::libs::eth::read::get_address(private_key.to_string()).await;
    if minter_address.is_err() {
        return Err(Debug::from("😵 Can't get minter address".to_string()));
    }
    let minter_address_string = minter_address.unwrap().to_string();
    let checkpoint_check = contract.checkpoint_ids(slug.clone()).call().await;
    if checkpoint_check.unwrap() != U256::from(0) {
        return Err(Debug::from(format!(
            "😵 Checkpoint already exists in contract",
        )));
    }
    // Get boosted gas price
    let gas_price = crate::libs::eth::read::get_gas_price(true).await;
    if !gas_price.is_ok() {
        return Err(Debug::from("😵 Can't get gas price".to_string()));
    }
    // Get last nonce
    let nonce = crate::libs::eth::read::get_transaction_count(minter_address_string.clone()).await;
    if !nonce.is_ok() {
        return Err(Debug::from("😵 Can't get nonce".to_string()));
    }
    println!("🔎 Checking if nonce was used..");
    let mut nonce256 = nonce.unwrap();
    println!("🔄 Checking nonce {}..", nonce256.clone());
    // Check in nonce was used
    let mut check = crate::libs::eth::db::is_nonce_valid(
        db,
        minter_address_string.clone(),
        nonce256.clone().to_string(),
    )
    .await;
    if !check.clone().unwrap() {
        while !check.clone().unwrap() {
            nonce256 = nonce256.add(1);
            check = crate::libs::eth::db::is_nonce_valid(
                db,
                minter_address_string.clone(),
                nonce256.clone().to_string(),
            )
            .await;
            println!("✅ Is nonce valid? {}", check.clone().unwrap());
        }
    }
    println!("✅ Nonce {} confirmed!", nonce256.clone());
    // Create transaction
    println!("🔍 Name: {}", name);
    println!("🔍 Description: {}", description);
    println!("🔍 Image: {}", image);
    println!("🔍 Slug: {}", slug);
    println!("🔍 Timestamp start: {}", timestamp_start);
    println!("🔍 Timestamp end: {}", timestamp_end);
    let tx = contract.add_checkpoint(
        name,
        description,
        timestamp_start,
        timestamp_end,
        slug.clone(),
        image,
    );
    // Send transaction
    let to_be_sent = tx
        .gas_price(gas_price.unwrap())
        .gas(1000000)
        .nonce(nonce256.clone());
    println!("🚀 Sending transaction..");
    let pending_tx = to_be_sent.send().await;
    if pending_tx.is_err() {
        return Err(Debug::from(pending_tx.unwrap_err().to_string()));
    }
    let pending_tx_hash = pending_tx.as_ref().unwrap().tx_hash();
    println!(
        "✅ Transaction pending with hash: {}",
        format!("{:#x}", pending_tx_hash)
    );
    let stored_txid = crate::libs::eth::db::insert_tx(
        db,
        minter_address_string.clone(),
        nonce256.clone().to_string(),
        format!("{:#x}", pending_tx_hash),
        "0x0".to_string(),
    )
    .await;
    if stored_txid.is_err() {
        println!("😵 Can't store pending transaction");
    }
    println!("⏳ Waiting for transaction..");
    let mined_tx = pending_tx.unwrap().await;
    if mined_tx.is_err() {
        return Err(Debug::from(mined_tx.unwrap_err().to_string()));
    }
    println!("✅ Transaction mined!");
    println!("🔍 Transaction hash: {:#?}", pending_tx_hash);
    let checkpoints_counter = contract.checkpoints_counter().call().await;
    if checkpoints_counter.is_err() {
        return Err(Debug::from(checkpoints_counter.unwrap_err().to_string()));
    }
    let checkpoint_id = contract.checkpoint_ids(slug.clone()).call().await;
    // Return transaction receipt
    Ok(checkpoint_id.unwrap().to_string())
}

pub async fn update_checkpoint(
    db: &State<Database>,
    name: String,
    description: String,
    image: String,
    slug: String,
    timestamp_start: U256,
    timestamp_end: U256,
) -> Result<String, Debug<String>> {
    dotenv().ok();
    // Read environment variables
    let provider_url = &env::var("ETH_PROVIDER_URL").unwrap();
    let contract_address = &env::var("PASSPORT_CONTRACT").unwrap();
    // Create provider instance from provider url
    let provider = Provider::<Http>::try_from(provider_url.as_str());
    if provider.is_err() {
        return Err(Debug::from(provider.unwrap_err().to_string()));
    }
    abigen!(SpaghettEthPassport, "./src/libs/eth/abis/passport.json");
    // Read private key from environment variable
    let private_key = &env::var("ETH_ADMIN_KEY").unwrap();
    // Create wallet instance from private key
    let wallet = private_key.parse::<LocalWallet>();
    if wallet.is_err() {
        return Err(Debug::from(wallet.unwrap_err().to_string()));
    }
    let signer = Arc::new(SignerMiddleware::new(
        provider.clone().unwrap(),
        wallet.unwrap().to_owned().with_chain_id(
            provider
                .clone()
                .unwrap()
                .get_chainid()
                .await
                .unwrap()
                .as_u64(),
        ),
    ));
    // Create contract instance
    let contract_address = Address::from_str(contract_address.as_str());
    let contract = SpaghettEthPassport::new(contract_address.unwrap(), signer);
    // Call contract function
    let minter_address = crate::libs::eth::read::get_address(private_key.to_string()).await;
    if minter_address.is_err() {
        return Err(Debug::from("😵 Can't get minter address".to_string()));
    }
    let minter_address_string = minter_address.unwrap().to_string();
    let checkpoint_check = contract.checkpoint_ids(slug.clone()).call().await;
    if checkpoint_check.unwrap() == U256::from(0) {
        return Err(Debug::from(format!("😵 Checkpoint not found in contract",)));
    }
    // Get boosted gas price
    let gas_price = crate::libs::eth::read::get_gas_price(true).await;
    if !gas_price.is_ok() {
        return Err(Debug::from("😵 Can't get gas price".to_string()));
    }
    // Get last nonce
    let nonce = crate::libs::eth::read::get_transaction_count(minter_address_string.clone()).await;
    if !nonce.is_ok() {
        return Err(Debug::from("😵 Can't get nonce".to_string()));
    }
    println!("🔎 Checking if nonce was used..");
    let mut nonce256 = nonce.unwrap();
    println!("🔄 Checking nonce {}..", nonce256.clone());
    // Check in nonce was used
    let mut check = crate::libs::eth::db::is_nonce_valid(
        db,
        minter_address_string.clone(),
        nonce256.clone().to_string(),
    )
    .await;
    if !check.clone().unwrap() {
        while !check.clone().unwrap() {
            nonce256 = nonce256.add(1);
            check = crate::libs::eth::db::is_nonce_valid(
                db,
                minter_address_string.clone(),
                nonce256.clone().to_string(),
            )
            .await;
            println!("✅ Is nonce valid? {}", check.clone().unwrap());
        }
    }
    println!("✅ Nonce {} confirmed!", nonce256.clone());
    // Create transaction
    println!("🔍 Name: {}", name);
    println!("🔍 Description: {}", description);
    println!("🔍 Image: {}", image);
    println!("🔍 Slug: {}", slug);
    println!("🔍 Timestamp start: {}", timestamp_start);
    println!("🔍 Timestamp end: {}", timestamp_end);
    let tx = contract.edit_checkpoint(
        slug.clone(),
        name,
        description,
        timestamp_start,
        timestamp_end,
        image,
    );
    // Send transaction
    let to_be_sent = tx
        .gas_price(gas_price.unwrap())
        .gas(1000000)
        .nonce(nonce256.clone());
    println!("🚀 Sending transaction..");
    let pending_tx = to_be_sent.send().await;
    if pending_tx.is_err() {
        return Err(Debug::from(pending_tx.unwrap_err().to_string()));
    }
    let pending_tx_hash = pending_tx.as_ref().unwrap().tx_hash();
    println!(
        "✅ Transaction pending with hash: {}",
        format!("{:#x}", pending_tx_hash)
    );
    let stored_txid = crate::libs::eth::db::insert_tx(
        db,
        minter_address_string.clone(),
        nonce256.clone().to_string(),
        format!("{:#x}", pending_tx_hash),
        "0x0".to_string(),
    )
    .await;
    if stored_txid.is_err() {
        println!("😵 Can't store pending transaction");
    }
    println!("⏳ Waiting for transaction..");
    let mined_tx = pending_tx.unwrap().await;
    if mined_tx.is_err() {
        return Err(Debug::from(mined_tx.unwrap_err().to_string()));
    }
    println!("✅ Transaction mined!");
    println!("🔍 Transaction hash: {:#?}", pending_tx_hash);
    let checkpoints_counter = contract.checkpoints_counter().call().await;
    if checkpoints_counter.is_err() {
        return Err(Debug::from(checkpoints_counter.unwrap_err().to_string()));
    }
    let checkpoint_id = contract.checkpoint_ids(slug.clone()).call().await;
    // Return transaction receipt
    Ok(checkpoint_id.unwrap().to_string())
}

pub async fn mint_passport(
    db: &State<Database>,
    receiver: String,
) -> Result<String, Debug<String>> {
    dotenv().ok();
    // Read environment variables
    let provider_url = &env::var("ETH_PROVIDER_URL").unwrap();
    let contract_address = &env::var("PASSPORT_CONTRACT").unwrap();
    // Create provider instance from provider url
    let provider = Provider::<Http>::try_from(provider_url.as_str());
    if provider.is_err() {
        return Err(Debug::from(provider.unwrap_err().to_string()));
    }
    abigen!(SpaghettEthPassport, "./src/libs/eth/abis/passport.json");
    // Read private key from environment variable
    let private_key = &env::var("ETH_MINTER_KEY").unwrap();
    // Create wallet instance from private key
    let wallet = private_key.parse::<LocalWallet>();
    if wallet.is_err() {
        return Err(Debug::from(wallet.unwrap_err().to_string()));
    }
    let signer = Arc::new(SignerMiddleware::new(
        provider.clone().unwrap(),
        wallet.unwrap().to_owned().with_chain_id(
            provider
                .clone()
                .unwrap()
                .get_chainid()
                .await
                .unwrap()
                .as_u64(),
        ),
    ));
    // Create contract instance
    let contract_address = Address::from_str(contract_address.as_str());
    let contract = SpaghettEthPassport::new(contract_address.unwrap(), signer);
    // Call contract function
    let minter_address = crate::libs::eth::read::get_address(private_key.to_string()).await;
    if minter_address.is_err() {
        return Err(Debug::from("😵 Can't get minter address".to_string()));
    }
    let minter_address_string = minter_address.unwrap().to_string();
    // Get boosted gas price
    let gas_price = crate::libs::eth::read::get_gas_price(true).await;
    if !gas_price.is_ok() {
        return Err(Debug::from("😵 Can't get gas price".to_string()));
    }
    // Get last nonce
    let nonce = crate::libs::eth::read::get_transaction_count(minter_address_string.clone()).await;
    if !nonce.is_ok() {
        return Err(Debug::from("😵 Can't get nonce".to_string()));
    }
    println!("🔎 Checking if nonce was used..");
    let mut nonce256 = nonce.unwrap();
    println!("🔄 Checking nonce {}..", nonce256.clone());
    // Check in nonce was used
    let mut check = crate::libs::eth::db::is_nonce_valid(
        db,
        minter_address_string.clone(),
        nonce256.clone().to_string(),
    )
    .await;
    if !check.clone().unwrap() {
        while !check.clone().unwrap() {
            nonce256 = nonce256.add(1);
            check = crate::libs::eth::db::is_nonce_valid(
                db,
                minter_address_string.clone(),
                nonce256.clone().to_string(),
            )
            .await;
            println!("✅ Is nonce valid? {}", check.clone().unwrap());
        }
    }
    println!("✅ Nonce {} confirmed!", nonce256.clone());
    // Create transaction
    let to_address = Address::from_str(&receiver);
    let tx = contract.mint_passport(to_address.unwrap());
    // Send transaction
    let to_be_sent = tx.gas_price(gas_price.unwrap()).nonce(nonce256.clone());
    println!("🚀 Sending transaction..");
    let pending_tx = to_be_sent.send().await;
    if pending_tx.is_err() {
        return Err(Debug::from(pending_tx.unwrap_err().to_string()));
    }
    let pending_tx_hash = pending_tx.as_ref().unwrap().tx_hash();
    println!(
        "✅ Transaction pending with hash: {}",
        format!("{:#x}", pending_tx_hash)
    );
    let stored_txid = crate::libs::eth::db::insert_tx(
        db,
        minter_address_string.clone(),
        nonce256.clone().to_string(),
        format!("{:#x}", pending_tx_hash),
        "0x0".to_string(),
    )
    .await;
    if stored_txid.is_err() {
        println!("😵 Can't store pending transaction");
    }
    println!("⏳ Waiting for transaction..");
    let mined_tx = pending_tx.unwrap().await;
    if mined_tx.is_err() {
        return Err(Debug::from(mined_tx.unwrap_err().to_string()));
    }
    println!("✅ Transaction mined!");
    println!("🔍 Transaction hash: {:#?}", pending_tx_hash);
    // Return transaction receipt
    Ok(format!("{:#x}", pending_tx_hash))
}

pub async fn set_profile(
    db: &State<Database>,
    name: &String,
    image: &String,
    description: &String,
    address: &String,
) -> Result<String, Debug<String>> {
    dotenv().ok();
    // Read environment variables
    let provider_url = &env::var("ETH_PROVIDER_URL").unwrap();
    let contract_address = &env::var("RESOLVER_CONTRACT").unwrap();
    // Create provider instance from provider url
    let provider = Provider::<Http>::try_from(provider_url.as_str());
    if provider.is_err() {
        return Err(Debug::from(provider.unwrap_err().to_string()));
    }
    abigen!(SpaghettEthResolver, "./src/libs/eth/abis/resolver.json");
    // Read private key from environment variable
    let private_key = &env::var("ETH_SETTER_KEY").unwrap();
    // Create wallet instance from private key
    let wallet = private_key.parse::<LocalWallet>();
    if wallet.is_err() {
        return Err(Debug::from(wallet.unwrap_err().to_string()));
    }
    let signer = Arc::new(SignerMiddleware::new(
        provider.clone().unwrap(),
        wallet.unwrap().to_owned().with_chain_id(
            provider
                .clone()
                .unwrap()
                .get_chainid()
                .await
                .unwrap()
                .as_u64(),
        ),
    ));
    // Create contract instance
    let contract_address = Address::from_str(contract_address.as_str());
    let contract = SpaghettEthResolver::new(contract_address.unwrap(), signer);
    // Call contract function
    let minter_address = crate::libs::eth::read::get_address(private_key.to_string()).await;
    if minter_address.is_err() {
        return Err(Debug::from("😵 Can't get minter address".to_string()));
    }
    let minter_address_string = minter_address.unwrap().to_string();
    // Get boosted gas price
    let gas_price = crate::libs::eth::read::get_gas_price(true).await;
    if !gas_price.is_ok() {
        return Err(Debug::from("😵 Can't get gas price".to_string()));
    }
    // Get last nonce
    let nonce = crate::libs::eth::read::get_transaction_count(minter_address_string.clone()).await;
    if !nonce.is_ok() {
        return Err(Debug::from("😵 Can't get nonce".to_string()));
    }
    println!("🔎 Checking if nonce was used..");
    let mut nonce256 = nonce.unwrap();
    println!("🔄 Checking nonce {}..", nonce256.clone());
    // Check in nonce was used
    let mut check = crate::libs::eth::db::is_nonce_valid(
        db,
        minter_address_string.clone(),
        nonce256.clone().to_string(),
    )
    .await;
    if !check.clone().unwrap() {
        while !check.clone().unwrap() {
            nonce256 = nonce256.add(1);
            check = crate::libs::eth::db::is_nonce_valid(
                db,
                minter_address_string.clone(),
                nonce256.clone().to_string(),
            )
            .await;
            println!("✅ Is nonce valid? {}", check.clone().unwrap());
        }
    }
    println!("✅ Nonce {} confirmed!", nonce256.clone());
    // Create transaction
    let to_address = Address::from_str(&address);
    let tx = contract.set_profile(
        name.to_owned(),
        description.to_owned(),
        image.to_owned(),
        to_address.unwrap(),
    );
    // Send transaction
    let to_be_sent = tx.gas_price(gas_price.unwrap()).nonce(nonce256.clone());
    println!("🚀 Sending transaction..");
    let pending_tx = to_be_sent.send().await;
    if pending_tx.is_err() {
        return Err(Debug::from(pending_tx.unwrap_err().to_string()));
    }
    let pending_tx_hash = pending_tx.as_ref().unwrap().tx_hash();
    println!(
        "✅ Transaction pending with hash: {}",
        format!("{:#x}", pending_tx_hash)
    );
    let stored_txid = crate::libs::eth::db::insert_tx(
        db,
        minter_address_string.clone(),
        nonce256.clone().to_string(),
        format!("{:#x}", pending_tx_hash),
        "0x0".to_string(),
    )
    .await;
    if stored_txid.is_err() {
        println!("😵 Can't store pending transaction");
    }
    println!("⏳ Waiting for transaction..");
    let mined_tx = pending_tx.unwrap().await;
    if mined_tx.is_err() {
        return Err(Debug::from(mined_tx.unwrap_err().to_string()));
    }
    println!("✅ Transaction mined!");
    println!("🔍 Transaction hash: {:#?}", pending_tx_hash);
    // Return transaction receipt
    Ok(format!("{:#x}", pending_tx_hash))
}

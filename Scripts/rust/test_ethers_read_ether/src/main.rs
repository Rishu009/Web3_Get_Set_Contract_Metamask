//Credit: https://coinsbench.com/ethereum-with-rust-tutorial-part-1-create-simple-transactions-with-rust-26d365a7ea93
use std::convert::TryFrom;
use std::env;

use ethers_providers::{Middleware, Provider, Http};
use ethers_core::types::{Address};

use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {

    let rpc_goerli_infura_https = env::var("goerliHTTPS_InfuraAPIKey").expect("$goerliHTTPS_InfuraAPIKey is not set");

    let provider = Provider::<Http>::try_from(rpc_goerli_infura_https).expect("could not instantiate HTTP Provider");

    // let rpc_shardeum_https = "https://liberty20.shardeum.org/";

    // let provider = Provider::<Http>::try_from(rpc_shardeum_https).expect("could not instantiate HTTP Provider");

    let chainid = provider.get_chainid().await?;
    println!("Got chainid: {}", chainid);

    let current_block_number = provider.get_block_number().await?;
    println!("current_block_number: {:?}", current_block_number);
    
    let current_block_parameters = provider.get_block(current_block_number).await?;
    println!(" current_block_parameters: {:?}", current_block_parameters);

    let block_timestamp = current_block_parameters.clone().unwrap().timestamp;
    println!("Got block_timestamp: {:?}", block_timestamp);

    let block_number = current_block_parameters.clone().unwrap().number.unwrap();
    println!("Got block_number: {:?}", block_number);

    let cycle_number = current_block_parameters.clone().unwrap().number.unwrap()/10;
    println!("Cycle number (Shardeum general rule: roundDown(blockNumber/10) ): {:?}", cycle_number);

    let block_gas = current_block_parameters.clone().unwrap().gas_used;
    println!("Got block_gas: {:?}", block_gas);
    
    let block_gas_limit = current_block_parameters.clone().unwrap().gas_limit;
    println!("Got block_gas_limit: {:?}", block_gas_limit);

    let block_hash = current_block_parameters.clone().unwrap().hash.unwrap();
    println!("Got block_hash: {:?}", block_hash);

    //Shardeum will not show transactions in bundles (blocks). 
    //Use the Shardeum Explorer JSON URL view with filters to get transactions from cycle ranges: 
    //https://docs.shardeum.org/smartContracts/events/pollEvents#reading-events-with-shardeum-cycles
    let block_transactions = current_block_parameters.clone().unwrap().transactions;
    println!("Got block_transactions: {:?}", block_transactions);

    let other_address_hex = "0x0000000000000000000000000000000000000000";
    let other_address = "0x0000000000000000000000000000000000000000".parse::<Address>()?;
    let other_balance = provider.get_balance(other_address, None).await?;
    println!("Balance for address {}: {}",other_address_hex, other_balance);

    // let ens_balance = provider.get_balance("car.eth", None).await?;
    // println!("ENS address balance: {}", ens_balance);

    Ok(())

}


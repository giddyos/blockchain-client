use blockchain_client::{BlockchainClient, Chain, JsonRpcClient, Network, RpcConfig};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // the rpc configuration
    let config = RpcConfig::new(
        "http://localhost:9332".to_string(),
        "yourusername".to_string(),
        "yourpassword".to_string(),
    )
    .with_timeout(30);

    // the client used to interact with the blockchain
    let client = JsonRpcClient::new(config, Chain::Litecoin, Network::Mainnet)?;

    println!("Connecting to {:?}...", client.chain());

    // 1. Get blockchain info
    println!("\n--- Blockchain Info ---");
    match client.get_blockchain_info().await {
        Ok(info) => {
            println!("Chain: {}", info.chain);
            println!("Blocks: {}", info.blocks);
            println!("Best Block Hash: {}", info.bestblockhash);
            println!("Difficulty: {}", info.difficulty);
            println!("Pruned: {:?}", info.pruned);
        }
        Err(e) => println!("Error getting blockchain info: {}", e),
    }

    // 2. Get block count
    println!("\n--- Block Count ---");
    match client.get_block_count().await {
        Ok(count) => println!("Current block count: {}", count),
        Err(e) => println!("Error getting block count: {}", e),
    }

    // 3. Get best block hash
    println!("\n--- Best Block Hash ---");
    match client.get_best_block_hash().await {
        Ok(hash) => println!("Best block hash: {}", hash),
        Err(e) => println!("Error getting best block hash: {}", e),
    }

    // 4. List recent transactions (Simple)
    println!("\n--- Recent Transactions (Top 5) ---");
    match client.list_transactions(None, Some(5), Some(0), true).await {
        Ok(txs) => {
            println!("Found {} transactions:", txs.len());
            for tx in txs.iter() {
                println!("- TXID: {}", tx.txid);
                println!("  Amount: {}", tx.amount);
                println!("  Confirmations: {}", tx.confirmations);
                println!("  Category: {}", tx.category);
            }

            // if we found a transaction, let's get its details
            if let Some(first_tx) = txs.first() {
                println!("\n--- Transaction Details for {} ---", first_tx.txid);
                match client.get_raw_transaction(&first_tx.txid, true).await {
                    Ok(raw_tx) => {
                        println!("Hash: {:?}", raw_tx.hash);
                        println!("Size: {:?} bytes", raw_tx.size);
                        println!("Inputs: {}", raw_tx.vin.len());
                        println!("Outputs: {}", raw_tx.vout.len());
                    }
                    Err(e) => println!("Error getting raw transaction: {}", e),
                }
            }
        }
        Err(e) => println!("Error listing transactions: {}", e),
    }

    // // Example of fetching ALL transactions using pagination
    // // Warning: This can take a long time for wallets with many transactions
    // println!("\n--- Fetching ALL Transactions (Paginated) ---");
    // let pagination = PaginationParams {
    //     skip: 0,
    //     count: 100, // Batch size
    // };
    // match client.list_transactions_paginated(pagination, true).await {
    //     Ok(all_txs) => println!("Total transactions fetched: {}", all_txs.len()),
    //     Err(e) => println!("Error fetching all transactions: {}", e),
    // }

    // 5. Check if an address is watched (Example address)
    let address = "MS8g7KDz3atp82qrQidtqBXtpHimNr2Uwp";
    println!("\n--- Address Watch Check ---");
    match client.is_address_watched(address).await {
        Ok(is_watched) => println!("Address {} is watched: {}", address, is_watched),
        Err(e) => println!("Error checking if address is watched: {}", e),
    }

    // 6. List Unspent (UTXOs) for the address
    println!("\n--- List Unspent (UTXOs) ---");
    match client
        .list_unspent(Some(1), None, Some(&[address.to_string()]))
        .await
    {
        Ok(utxos) => {
            println!("Found {} UTXOs for address {}", utxos.len(), address);
            for utxo in utxos.iter().take(5) {
                println!(
                    "- {} ({} confirmations): {} amount",
                    utxo.txid, utxo.confirmations, utxo.amount
                );
            }
        }
        Err(e) => println!("Error listing unspent: {}", e),
    }

    Ok(())
}

# blockchain-client

a rust client for talking to bitcoin/litecoin nodes. this is my first project on crates.io.

## usage

add this to your `Cargo.toml`:

```toml
[dependencies]
blockchain-client = "1.0.0"
```

then you can use it in your code:

```rust
use blockchain_client::{BlockchainClient, Chain, JsonRpcClient, Network, RpcConfig};

#[tokio::main]
async fn main() {
    let config = RpcConfig::new(
        "http://localhost:9332".to_string(),
        "yourusername".to_string(),
        "yourpassword".to_string(),
    );

    let client = JsonRpcClient::new(config, Chain::Litecoin, Network::Mainnet).unwrap();

    let info = client.get_blockchain_info().await.unwrap();
    println!("{:?}", info);
}
```

## testing with the example

this project comes with a demo in the `examples` folder. to run it:

1.  you need a local litecoin node running.
2.  edit the `examples/demo.rs` file with your node's rpc username and password.
3.  run the example:

```sh
cargo run --example demo
```

this will connect to your node and print out some blockchain info. It's also a great reference on how to use this package.

use ethers::{
    contract::{abigen, ContractFactory},
    core::utils::Anvil,
    middleware::SignerMiddleware,
    providers::{Http, Provider},
    signers::{LocalWallet, Signer},
    solc::{Artifact, Project, ProjectPathsConfig},
    types::U256,
};
use eyre::Result;
use std::{path::PathBuf, sync::Arc, time::Duration};

// Generate the type-safe contract bindings by providing the ABI
// definition in human readable format
abigen!(
    SimpleContract,
    r#"[
        function get() public returns(uint)
        function set(uint n) public
        function increment(uint n) public
        function decrement(uint n) public
    ]"#,
    event_derives(serde::Deserialize, serde::Serialize)
);

#[tokio::main]
async fn main() -> Result<()> {
    // the directory we use is root-dir/examples
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples");
    // we use `root` for both the project root and for where to search for contracts since
    // everything is in the same directory
    let paths = ProjectPathsConfig::builder()
        .root(&root)
        .sources(&root)
        .build()
        .unwrap();

    // get the solc project instance using the paths above
    let project = Project::builder()
        .paths(paths)
        .ephemeral()
        .no_artifacts()
        .build()
        .unwrap();
    // compile the project and get the artifacts
    let output = project.compile().unwrap();
    let contract = output
        .find_first("SimpleStorage")
        .expect("could not find contract")
        .clone();
    let (abi, bytecode, _) = contract.into_parts();

    // 2. instantiate our wallet & anvil
    let anvil = Anvil::new().path("/home/faraz/.foundry/bin/anvil").spawn();
    let wallet: LocalWallet = anvil.keys()[0].clone().into();

    // 3. connect to the network
    let provider =
        Provider::<Http>::try_from(anvil.endpoint())?.interval(Duration::from_millis(10u64));

    let provider =
        Provider::<Http>::try_from("http://localhost:8545")?.interval(Duration::from_millis(10u64));

    // 4. instantiate the client with the wallet
    let client = SignerMiddleware::new(provider, wallet.with_chain_id(anvil.chain_id()));
    let client = Arc::new(client);

    // 5. create a factory which will be used to deploy instances of the contract
    let factory = ContractFactory::new(abi.unwrap(), bytecode.unwrap(), client.clone());

    // 6. deploy it with the constructor arguments
    let contract = factory.deploy("initial value".to_string())?.send().await?;

    // 7. get the contract's address
    let addr = contract.address();

    // 8. instantiate the contract
    let contract = SimpleContract::new(addr, client.clone());

    // 9. call the `setValue` method
    // (first `await` returns a PendingTransaction, second one waits for it to be mined)
    let _receipt = contract.set(U256::one()).send().await?.await?;

    // 10. get all events
    //let logs = contract
    //    .value_changed_filter()
    //    .from_block(0u64)
    //    .query()
    //    .await?;

    // 11. get the new value
    let value = contract.get().call().await?;

    println!("Value: {value}");

    Ok(())
}

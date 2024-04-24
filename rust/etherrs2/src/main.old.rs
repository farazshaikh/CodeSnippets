use ethers::{
    contract::{abigen, ContractFactory},
    core::utils::Anvil,
    middleware::SignerMiddleware,
    prelude::{
        gas_escalator::{Frequency, GeometricGasPrice},
        gas_oracle::{GasCategory, GasNow, GasOracleMiddleware},
        GasEscalatorMiddleware,
    },
    providers::{Http, Provider},
    signers::{LocalWallet, Signer},
    solc::{Artifact, Project, ProjectPathsConfig},
};
use eyre::Result;
use std::{path::PathBuf, sync::Arc, time::Duration};

abigen!(NordSCMoc, "./contracts/out/NordSCMock.sol/NordSCMock.json");

#[tokio::main]
async fn main() -> Result<()> {
    // the directory we use is root-dir/examples
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("contracts");
    // we use `root` for both the project root and for where to search for contracts since
    // everything is in the same directory
    let paths = ProjectPathsConfig::builder()
        .root(&root)
        .sources(&root)
        .build()
        .unwrap();

    // get the solc project instance using the paths above
    let project = Project::builder().paths(paths).ephemeral().build().unwrap();
    // compile the project and get the artifacts
    let output = project.compile().unwrap();
    let contract = output
        .find_first("NordSCMock")
        .expect("could not find contract")
        .clone();
    let (abi, bytecode, _) = contract.into_parts();

    // 2. instantiate our wallet & anvil
    //let anvil = Anvil::default().spawn();

    let anvil = Anvil::new().args(["--hardfork", "london"]).spawn();

    println!("Anvil running at `{}`", anvil.endpoint());
    let wallet: LocalWallet = anvil.keys()[0].clone().into();
    //let key = hex::decode("ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80")
    //    .expect("failed to decode");
    //let wallet: LocalWallet = LocalWallet::from_bytes(&key).unwrap();

    // 3. connect to the network
    let provider =
        Provider::<Http>::try_from(anvil.endpoint())?.interval(Duration::from_millis(50000u64));

    // 4. instantiate the client with the wallet
    let client = SignerMiddleware::new(provider, wallet.with_chain_id(anvil.chain_id()));
    let client = Arc::new(client);

    // 5. create a factory which will be used to deploy instances of the contract
    let factory = ContractFactory::new(abi.unwrap(), bytecode.unwrap(), client.clone());

    // 6. deploy it with the constructor arguments
    let contract = factory.deploy("Somestring".to_string())?.send().await;
    println!("{contract:?}");
    loop {}
    let contract = contract.unwrap();

    // 7. get the contract's address
    let addr = contract.address();

    // 8. instantiate the contract
    let contract = NordSCMoc::new(addr, client.clone());
    loop {}
    /*
        // 9. call the `setValue` method
        // (first `await` returns a PendingTransaction, second one waits for it to be mined)
        let _receipt = contract.set_value("hi".to_owned()).send().await?.await?;

        // 10. get all events
        let logs = contract
            .value_changed_filter()
            .from_block(0u64)
            .query()
            .await?;

        // 11. get the new value
        let value = contract.get_value().call().await?;

        println!("Value: {value}. Logs: {}", serde_json::to_string(&logs)?);
    */
    Ok(())
}

/*
use ethers::{
    contract::abigen,
    core::utils::Anvil,
    middleware::SignerMiddleware,
    providers::{Http, Provider},
    signers::{LocalWallet, Signer},
};
use ethers::{prelude::Abigen, solc::Solc};
use eyre::Result;
use std::sync::Arc;
use std::time::Duration;

fn main() -> Result<()> {
    let mut args = std::env::args();
    args.next().unwrap(); // skip program name


    println!("Generating bindings for {contract}\n");

    let contracts = Solc::default().compile_source(&contract)?;
    contracts.contracts_iter().find_fir

    // compile it
    let abi = if contract.ends_with(".sol") {
        let contracts = Solc::default().compile_source(&contract)?;
        let abi = contracts
            .get(&contract, &contract_name)
            .unwrap()
            .abi
            .unwrap();
        serde_json::to_string(abi).unwrap()
    } else {
        contract
    };

    let bindings = Abigen::new(&contract_name, abi)?.generate()?;

    // print to stdout if no output arg is given
    if let Some(output_path) = args.next() {
        bindings.write_to_file(output_path)?;
    } else {
        bindings.write(&mut std::io::stdout())?;
    }

    let anvil = Anvil::new().spawn();
    let wallet: LocalWallet = anvil.keys()[0].clone().into();
    let provider =
        Provider::<Http>::try_from(anvil.endpoint())?.interval(Duration::from_millis(10u64));

    let client = Arc::new(SignerMiddleware::new(
        provider,
        wallet.with_chain_id(anvil.chain_id()),
    ));

    // 5. deploy contract
    let greeter_contract = Greeter::deploy(client, "Hello World!".to_string())
        .unwrap()
        .send()
        .await
        .unwrap();

    Ok(())
}
*/

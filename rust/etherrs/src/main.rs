use ethers::{
    abi::Address,
    contract::{abigen, ContractFactory},
    core::utils::Anvil,
    middleware::SignerMiddleware,
    providers::{Http, Provider},
    signers::{LocalWallet, Signer},
    solc::{Artifact, Project, ProjectPathsConfig},
};
use eyre::Result;
use paste::paste;
use std::{path::PathBuf, sync::Arc, time::Duration};
//abigen!(NordSCMoc, "./contracts/out/NordSCMock.sol/NordSCMock.json");
abigen!(
    NordGetters,
    "$CARGO_MANIFEST_DIR/settlement_abi/NordGettersFacet.sol/NordGettersFacet.json"
);

use abigenmacros::my_fn_like_proc_macro(NordGettersFacet);

//
//abigen!(NordGetters,);
/*
macro_rules! gencode {
    () => {
        const p: u32 = 0;
        abigen!(
            NordGettersFacet,
            paste! {[<std::env!(OUTDIR), "out/contracts_bindings/NordGettersFacet.sol/NordGettersFacet.json">]}
        );
    };
}

macro_rules! gencode {
    { $($i: ident),* } => {
                  $(abigen!(
                           $i,
                           concat!("$CARGO_MANIFEST_DIR/settlement_abi/", $i)
                           );
                      )*
              }
}
*/
//gencode!(NordGettersFacet);

/*
mod build_test {
    use ethers::solc::{Project, ProjectPathsConfig};
    use std::path::PathBuf;

    pub(crate) fn localcontracts(path: &str) {
        // the directory we use is root-dir/examples
        let root = PathBuf::from(path).join("contracts");
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
        println!("{output:?}")
    }
}
*/
mod foundry_build_test {
    use std::str::FromStr;

    use foundry_compilers::{
        remappings::Remapping, resolver::print, utils, Project, ProjectPathsConfig,
    };

    use crate::{nord_getters, NordGetters};

    pub fn foundry_build2(path: impl ToString) {
        let root = utils::canonicalize("/wrk/lyn/nucleus/contracts/").unwrap();
        let mut remaps = [
            "@openzeppelin-contracts/=lib/openzeppelin-contracts",
            "@diamond/=lib/diamond-1-hardhat",
            "base64/=lib/base64",
            "ds-test/=lib/forge-std/lib/ds-test/src",
            "erc4626-tests/=lib/openzeppelin-contracts/lib/erc4626-tests",
            "forge-std/=lib/forge-std/src",
            "openzeppelin-contracts/=lib/openzeppelin-contracts",
        ];

        let remap = remaps
            .into_iter()
            .map(|m| Remapping::from_str(m).unwrap())
            .collect::<Vec<_>>();
        let paths = ProjectPathsConfig::builder()
            .root(&root)
            .sources(root.join("src"))
            .lib(root.join("lib"))
            .remappings(remap)
            .build()
            .unwrap();
        let project = Project::builder().paths(paths).build().unwrap();
        //let contracts = project.compile().unwrap().succeeded().output().contracts;

        println!("{project:?}");
        let output = project.compile().unwrap().output();
        println!("{output:?}");

        output
            .errors
            .iter()
            .for_each(|e| println!("{:?}", e.message));
        for ele in output.contracts.into_iter() {
            println!("Contracts {ele:?}");
        }

        // Tell Cargo that if a source file changes, to rerun this build script.
        project.rerun_if_sources_changed();
    }

    pub fn foundry_build(path: &str) {
        // configure the project with all its paths, solc, cache etc.
        let project = Project::builder()
            .paths(ProjectPathsConfig::dapptools(path).unwrap())
            .build()
            .unwrap();
        println!("{project:?}");

        let output = project.compile().unwrap();

        let output = output.output();
        output
            .errors
            .iter()
            .for_each(|e| println!("{:?}", e.message));

        for ele in output.contracts.into_iter() {
            println!("Contracts {ele:?}");
        }

        // Tell Cargo that if a source file changes, to rerun this build script.
        project.rerun_if_sources_changed();
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    //build_test::localcontracts("/wrk/lyn/nucleus/");
    //foundry_build_test::foundry_build2("/wrk/lyn/nucleus/contracts");
    //return Ok(());

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

    let anvil = Anvil::new().spawn();

    println!("Anvil running at `{}`", anvil.endpoint());
    let wallet: LocalWallet = anvil.keys()[0].clone().into();
    let key = hex::decode("ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80")
        .expect("failed to decode");
    let wallet: LocalWallet = LocalWallet::from_bytes(&key).unwrap();

    // 3. connect to the network
    let provider =
        Provider::<Http>::try_from(anvil.endpoint())?.interval(Duration::from_millis(50000u64));
    //let provider = Provider::<Http>::try_from("http://localhost:8545")?
    //    .interval(Duration::from_millis(50000u64));
    let res = provider
        .request::<_, serde_json::Value>("anvil_mine", &["100"])
        .await
        .unwrap();

    // 4. instantiate the client with the wallet
    //    let client = SignerMiddleware::new(provider, wallet.with_chain_id(anvil.chain_id()));
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
    //let contract = NordSCMoc::new(addr, client.clone());

    let getter = NordGetters::new(
        "0x610178dA211FEF7D417bC0e6FeD39F05609AD788"
            .parse::<Address>()
            .unwrap(),
        client.clone(),
    );
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

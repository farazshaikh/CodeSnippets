use ethers::solc::{remappings::Remapping, resolver::print, utils, Project, ProjectPathsConfig};
use eyre::Result;
use std::str::FromStr;
use std::{path::PathBuf, sync::Arc, time::Duration};
fn main() {
    //localcontracts(env!("CARGO_MANIFEST_DIR"));
    build_eth_bindings("/wrk/lyn/nucleus/contracts/").expect("eth contracts");
    //localcontracts("/wrk/lyn/nucleus/");
}

fn build_eth_bindings(contracts_path: &str) -> Result<()> {
    let root = utils::canonicalize(contracts_path)?;
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
        .artifacts(
            // Cannout reliably use OUT_DIR OR TARGET_DIR here
            // https://github.com/rust-lang/cargo/issues/9661
            PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap()).join("settlement_abi"),
        )
        .remappings(remap)
        .build()
        .unwrap();
    let project = Project::builder().paths(paths).build().unwrap();
    println!("{:?}", project);
    let output = project.compile().unwrap().output();

    output
        .errors
        .iter()
        .for_each(|e| println!("{:?}", e.message));
    for ele in output.contracts.into_iter() {
        println!("Contracts {ele:?}");
    }

    // Tell Cargo that if a source file changes, to rerun this build script.
    project.rerun_if_sources_changed();
    Ok(())
}

fn localcontracts(path: &str) {
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

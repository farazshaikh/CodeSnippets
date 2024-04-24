use borsh::{BorshDeserialize, BorshSerialize};
#[derive(BorshDeserialize, BorshSerialize)]
struct Foo {
    tab: std::collections::BTreeMap<u32, u32>,
    #[borsh(skip)]
    name: String,
    #[borsh(skip)]
    name2: Mang,
}

#[derive(Default)]
struct Mang {
    n: String,
}

#[derive(BorshDeserialize, BorshSerialize)]
struct DAStruct {
    start: u64,
    end: u64,
    paylaod: Vec<Vec<u8>>,
}

fn main() {
    let mut da = DAStruct {
        start: u64::MAX,
        end: u64::MAX,
        paylaod: vec![vec![0]],
    };
    let mut payload: Vec<u8> = Vec::new();
    da.serialize(&mut payload).unwrap();
    println!("Payload {}", payload.len());
}

use std::sync::Arc;

use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
struct Man {
    time: time::OffsetDateTime,
    data: Arc<u8>,
}

impl BorshSerialize for Man {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        let x = self.time.unix_timestamp();
        writer.write_all(&x.to_le_bytes())?;
        BorshSerialize::serialize(&self.data, writer)
    }
}

impl BorshDeserialize for Man {
    fn deserialize_reader<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        reader.
    }
}

/*
impl BorshSerialize for Man {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        self.time
    }
}*/
fn main() {
    println!("Hello, world!");
}

pub mod morestina;
pub mod tableleakbug;
use redb::TableDefinition;
use std::sync::Arc;

#[derive(Debug)]
pub struct LoggedAction {
    pub time: time::OffsetDateTime,
    pub payload: Arc<[u8]>,
}

const TABLE: TableDefinition<u64, LoggedAction> = TableDefinition::new("actions");
type ActionId = u64;

impl LoggedAction {
    fn new() -> Self {
        let time = time::OffsetDateTime::now_utc();
        let data = [0, 1, 2, 3];
        let payload = Arc::from(data);
        Self { time, payload }
    }
}

impl redb::RedbValue for LoggedAction {
    type SelfType<'a> = LoggedAction;
    type AsBytes<'a> = Vec<u8>;

    fn fixed_width() -> Option<usize> {
        None
    }

    fn type_name() -> redb::TypeName {
        redb::TypeName::new("nord::jnl::LoggedAction")
    }

    fn as_bytes<'a, 'b: 'a>(x: &'a Self::SelfType<'b>) -> Self::AsBytes<'a>
    where
        Self: 'a,
        Self: 'b,
    {
        let mut v = Vec::with_capacity(8 + x.payload.len());
        v.extend_from_slice(&x.time.unix_timestamp().to_le_bytes()[..]);
        v.extend_from_slice(&x.payload[..]);
        v
    }

    fn from_bytes<'a>(data: &'a [u8]) -> Self::SelfType<'a>
    where
        Self: 'a,
    {
        let (time, payload_bytes) = data.split_at(8);
        let time = i64::from_le_bytes(time.try_into().unwrap());
        let time = time::OffsetDateTime::from_unix_timestamp(time).unwrap();
        let payload = payload_bytes.into();
        Self { time, payload }
    }
}

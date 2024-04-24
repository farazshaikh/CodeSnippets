use crate::{ActionId, LoggedAction, TABLE};
use anyhow::{anyhow, Result};
use redb::*;
use std::sync::Arc;

#[derive(Clone)]
pub struct Journal {
    db: Arc<redb::Database>,
}

impl Journal {
    pub fn open(path: impl AsRef<std::path::Path>) -> Result<Self, redb::Error> {
        let db = redb::Database::builder().create(path)?;
        let db = std::sync::Arc::new(db);
        Ok(Self { db })
    }
    pub fn insertdata(&self) -> Result<()> {
        let txn = self.db.begin_write()?;
        {
            let mut table = txn.open_table(TABLE)?;
            table.insert(1, LoggedAction::new())?;
            table.insert(2, LoggedAction::new())?;
            table.insert(3, LoggedAction::new())?;
        }
        txn.commit()?;
        Ok(())
    }

    pub fn iter<'a>(
        &'a self,
        from: ActionId,
        until: ActionId,
    ) -> Result<Box<dyn std::iter::Iterator<Item = Result<(u64, LoggedAction), anyhow::Error>> + 'a>>
    {
        let tx = self.db.begin_read()?;
        let table: ReadOnlyTable<'a, u64, LoggedAction> =
            unsafe { std::mem::transmute(tx.open_table(TABLE).unwrap()) };
        let ret: Box<dyn std::iter::Iterator<Item = Result<(u64, LoggedAction), anyhow::Error>>> =
            Box::new(table.range(from..=until).unwrap().map(|x| {
                x.map(|(k, v)| (k.value(), v.value()))
                    .map_err(|_| anyhow!("foo"))
            }));
        Ok(unsafe { std::mem::transmute(ret) })
    }
}

#[cfg(test)]
mod test {
    use super::Journal;
    #[test]
    fn iterator_test() {
        let journal = Journal::open("/tmp/jnltableleak.db").unwrap();
        journal.insertdata().unwrap();
        let iter = journal.iter(0, 3).unwrap();
        for item in iter {
            println!("{item:?}");
        }
    }
}

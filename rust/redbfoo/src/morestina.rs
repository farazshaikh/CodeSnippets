use crate::{ActionId, LoggedAction, TABLE};
use aliasable::prelude::AliasableBox;
use redb::*;
use std::sync::Arc;

#[derive(Clone)]
pub struct Journal {
    db: Arc<redb::Database>,
}

type JournalIter<'a> =
    Box<dyn std::iter::Iterator<Item = Result<(u64, LoggedAction), redb::Error>> + 'a>;

pub struct JournalIterator<'a> {
    iter: JournalIter<'a>,
    _table: ReadOnlyTable<'a, ActionId, LoggedAction>,
    _tx: AliasableBox<redb::ReadTransaction<'a>>,
}

impl<'a> JournalIterator<'a> {
    pub fn new(
        db: &'a Database,
        since: ActionId,
        until_inclusive: ActionId,
    ) -> Result<Self, redb::Error> {
        let _tx = AliasableBox::from_unique(Box::new(db.begin_read()?));
        let _table: ReadOnlyTable<'a, _, _> =
            unsafe { std::mem::transmute(_tx.open_table(TABLE).unwrap()) };
        let iter: JournalIter<'_> =
            Box::new(_table.range(since..=until_inclusive).unwrap().map(|x| {
                x.map(|(k, v)| (k.value(), v.value()))
                    .map_err(redb::Error::from)
            }));
        let iter = unsafe { std::mem::transmute(iter) };
        Ok(Self { iter, _table, _tx })
    }
}

impl<'a> Iterator for JournalIterator<'a> {
    type Item = Result<(u64, LoggedAction), redb::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl Journal {
    pub fn open(path: impl AsRef<std::path::Path>) -> Result<Self, redb::Error> {
        let db = redb::Database::builder().create(path)?;
        let db = std::sync::Arc::new(db);
        Ok(Self { db })
    }
    pub fn insertdata(&self) -> Result<(), redb::Error> {
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

    pub fn range_iter(
        &self,
        since: ActionId,
        until_inclusive: ActionId,
    ) -> Result<JournalIterator, redb::Error> {
        JournalIterator::new(&self.db, since, until_inclusive)
    }
}

#[cfg(test)]
mod test {
    use super::Journal;
    #[test]
    fn iterator_test() {
        let journal = Journal::open("/tmp/jnl.db").unwrap();
        journal.insertdata().unwrap();
        for item in journal.range_iter(0, 3).unwrap() {
            println!("{item:?}");
        }
    }
}

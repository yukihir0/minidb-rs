use anyhow::Result;

use minidb_rs::btree::{BTree, SearchMode};
use minidb_rs::buffer::{BufferPool, BufferPoolManager};
use minidb_rs::disk::{DiskManager, PageId};
use minidb_rs::tuple;

fn main() -> Result<()> {
    let disk = DiskManager::open("simple.rly")?;
    let pool = BufferPool::new(10);
    let mut bufmgr = BufferPoolManager::new(disk, pool);

    let btree = BTree::new(PageId(0));
    let mut iter = btree.search(&mut bufmgr, SearchMode::Start)?;

    while let Some((key, value)) = iter.next(&mut bufmgr)? {
        let mut record = vec![];
        tuple::decode(&key, &mut record);
        tuple::decode(&value, &mut record);
        if record[2] == b"Smith" {
            println!("{:?}", tuple::Pretty(&record));
        }
    }
    Ok(())
}

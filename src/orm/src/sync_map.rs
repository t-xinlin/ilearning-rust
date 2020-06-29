mod sync_map;

use tokio::sync::Mutex;
#[derive(Debug)]
pub struct SyncMap<T> {
    pub cell: Mutex<RefCell<HashMap<String, T>>>
}

impl<T> SyncMap<T> {
    pub fn new() -> SyncMap<T> {
        SyncMap {
            cell: Mutex::new(RefCell::new(HashMap::new()))
        }
    }

    /// put an value,this value will move lifetime into SyncMap
    pub async fn put(&self, key: &str, value: T) {
        let lock = self.cell.lock().await;
        let mut b = lock.borrow_mut();
        b.insert(key.to_string(), value);
        //函数结尾 lock锁即可释放，因此不管是put还是pop，锁定的时间都是比较小的。而且锁定是依赖tokio运行时调度，而不是线程阻塞
    }

    /// pop value,lifetime will move to caller
    pub async fn pop(&self, key: &str) -> Option<T> {
        let lock = self.cell.lock().await;
        let mut b = lock.borrow_mut();
        return b.remove(key);
    }
}

// 使用
// pub struct Rbatis<'r> { context_tx: SyncMap<Transaction<PoolConnection<MySqlConnection>>> }
//
// impl Rbatis {
//
// //这里我们可以看到，使用SyncMap既可以修改context上下文，又不必吧&self改为&mut self。即保证了并发安全和性能
//
//     pub async fn begin(&self, tx_id: &str) -> Result<u64, rbatis_core::Error> {
//         if tx_id.is_empty() { return Err(rbatis_core::Error::from("[rbatis] tx_id can not be empty")); }
//         let conn = self.get_pool()?.begin().await?;
//         self.context_tx.put(tx_id, conn).await;
//         return Ok(1);
//     }
// }
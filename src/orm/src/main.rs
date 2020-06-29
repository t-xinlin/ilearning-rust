mod decode_json;
mod sync_map;

fn main() {
    println!("Hello, sync_map!");

    let rb = Rbatis::new(MYSQL_URL).await.unwrap();
    let py = r#"
SELECT * FROM biz_activity
WHERE delete_flag = #{delete_flag}
if name != null:
  AND name like #{name+'%'}
if ids != null:
  AND id in (
  trim ',':
     for item in ids:
       #{item},
  )"#;
    let data: serde_json::Value = rb.py_fetch("", py, &json!({   "delete_flag": 1 })).await.unwrap();
    println!("{}", data);


}

pub struct Rbatis<'r> { context_tx: SyncMap<Transaction<PoolConnection<MySqlConnection>>> }

impl Rbatis {

//这里我们可以看到，使用SyncMap既可以修改context上下文，又不必吧&self改为&mut self。即保证了并发安全和性能

    pub async fn begin(&self, tx_id: &str) -> Result<u64, rbatis_core::Error> {
        if tx_id.is_empty() { return Err(rbatis_core::Error::from("[rbatis] tx_id can not be empty")); }
        let conn = self.get_pool()?.begin().await?;
        self.context_tx.put(tx_id, conn).await;
        return Ok(1);
    }
}
fn decode_json<T>(&mut self) -> BoxFuture<Result<T, crate::Error>>
    where T: DeserializeOwned {
    Box::pin(async move {
        let mut arr = vec![];
        while let Some(row) = self.next().await? as Option<MySqlRow<'_>> {
            let mut m = serde_json::Map::new();
            let keys = row.names.keys();
            for x in keys {
                let key = x.to_string();
                let key_str=key.as_str();
                let v:serde_json::Value = row.json_decode_impl(key_str)?;
                m.insert(key, v);
            }
            arr.push(serde_json::Value::Object(m));
        }
        let r = json_decode(arr)?;
        return Ok(r);
    })
}
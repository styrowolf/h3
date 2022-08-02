use redis::{FromRedisValue, ToRedisArgs};

use crate::H3Cell;

impl ToRedisArgs for H3Cell {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + redis::RedisWrite,
    {
        out.write_arg(hex::encode(&self.0.to_be_bytes()).as_bytes())
    }
}

impl FromRedisValue for H3Cell {
    fn from_redis_value(v: &redis::Value) -> redis::RedisResult<Self> {
        match v {
            redis::Value::Data(data) => {
                let n = hex::decode(data).map_err(|_| {
                    <(redis::ErrorKind, &str) as Into<redis::RedisError>>::into((
                        redis::ErrorKind::TypeError,
                        "type mismatch: H3Cell",
                    ))
                })?;

                let mut bytes = [0u8; 8];
                for (i, b) in bytes.iter_mut().enumerate() {
                    *b = n[i];
                }
                Ok(H3Cell(u64::from_be_bytes(bytes)))
            }
            _ => Err((redis::ErrorKind::TypeError, "type mismatch: H3Cell").into()),
        }
    }
}

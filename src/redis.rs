use redis::{ToRedisArgs, FromRedisValue};

use crate::H3Cell;

impl ToRedisArgs for H3Cell {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + redis::RedisWrite {
        out.write_arg(&self.0.to_be_bytes())
    }
}

impl FromRedisValue for H3Cell {
    fn from_redis_value(v: &redis::Value) -> redis::RedisResult<Self> {
        match v {
            redis::Value::Data(data) => {
                let mut bytes: [u8; 8] = [0; 8];
                for (i, byte)in bytes.iter_mut().enumerate() {
                    *byte = data[i];
                }
                Ok(H3Cell(u64::from_be_bytes(bytes)))
            }
            _ => Err((redis::ErrorKind::TypeError, "type mismatch: H3Cell").into())
        }
    }
}
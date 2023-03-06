use time::{OffsetDateTime, PrimitiveDateTime};
use time::util::local_offset;

use crate::error::AppResult;

/// 获取当前时间
pub fn now() -> AppResult<PrimitiveDateTime> {
    let local_offset = OffsetDateTime::now_local()?;
    Ok(PrimitiveDateTime::new(local_offset.date(), local_offset.time()))
}
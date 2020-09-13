use chrono::{Utc, Duration};

/// 有効期間
pub const VALIDITY_DAYS: i64 = 7;

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    uid: i32,
    exp: i64
}

impl Claims {
    pub fn new(id: i32) -> Claims {
        Claims {
            uid: id,
            exp: (Utc::now() + Duration::days(VALIDITY_DAYS)).timestamp()
        }
    }
}

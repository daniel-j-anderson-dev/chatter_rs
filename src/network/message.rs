use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Message {
    user_name: String,
    message: String,
    time: DateTime<Utc>,
}

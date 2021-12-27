use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    id: String,
    username: String,
    name: String,
    description: String,
    public_metrics: Metrics,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Metrics {
    tweet_count: u32,
    following_count: u32,
    followers_count: u32,
}

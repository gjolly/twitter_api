use std::collections::HashMap;
use std::process::exit;
use serde::{Deserialize, Serialize};
use reqwest;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    id: String,
    username: String,
    name: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserQueryResp {
    data: Vec<User>
}

pub struct Client {
    api_key: String,
    base_url: String,
}

impl Client {
    pub fn new(key: String) -> Client {
        Client{
            api_key: String::from(key),
            base_url: String::from("https://api.twitter.com/2"),
        }
    }

    #[tokio::main]
    pub async fn get(&self, path: String, body: HashMap<String, String>)
        -> Result<Vec<User>, Box<dyn std::error::Error>> {
        let url = format!("{}{}", self.base_url.clone(), &path);

        let client = reqwest::Client::new();

        let request = client
            .get(&url)
            .bearer_auth(&self.api_key)
            .query(&body);

        let raw_resp = match request.send().await {
            Ok(resp) => resp,
            Err(error) => panic!("failed to query URL[{:?}]: {:?}", url, error),
        };

        if ! raw_resp.status().is_success() {
            println!("http status: {:?}",raw_resp.status());
            exit(1);
        }

        let txt_resp = match raw_resp.text()
            .await {
                Ok(txt) => txt,
                Err(error) => panic!("failed to decode json: {:?}", error),
            };

        let resp: UserQueryResp = match serde_json::from_str(&txt_resp) {
            Ok(users) => users,
            Err(error) => panic!("failed to decode json: {:?}", error),
        };

        Ok(resp.data)
    }
}

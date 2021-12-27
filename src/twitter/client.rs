use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use reqwest::Method;
use reqwest;

use crate::twitter::user::User;

#[derive(Serialize, Deserialize)]
pub struct UserQueryResp {
    data: Vec<User>
}

pub struct Client {
    api_key: String,
    base_url: String,
    http_client: reqwest::Client,
}

impl Client {
    pub fn new(key: String) -> Client {
        Client{
            api_key: String::from(key),
            base_url: String::from("https://api.twitter.com/2"),
            http_client: reqwest::Client::new(),
        }
    }

    async fn do_request(&self, method: Method, path: String, body: HashMap<String, String>)
        -> Result<String, Box<dyn std::error::Error>> {
        let url = format!("{}{}", self.base_url.clone(), &path);

        let request = self.http_client
            .request(method, &url)
            .bearer_auth(&self.api_key)
            .query(&body);

        let raw_resp = match request.send().await {
            Ok(resp) => match resp.error_for_status() {
                Ok(resp) => resp,
                Err(error) => return Err(error.into())
            },
            Err(error) => return Err(error.into()),
        };

        match raw_resp.text().await {
            Ok(resp) => Ok(String::from(resp.as_str())),
            Err(error) => Err(error.into())
        }
    }

    pub async fn find_users_by_username(&self, usernames: Vec<String>)
        -> Result<Vec<User>, Box<dyn std::error::Error>> {
        let path = String::from("/users/by");

        let mut body = HashMap::new();
        body.insert(String::from("usernames"), usernames.join(","));
        body.insert(String::from("user.fields"), String::from("description,public_metrics"));

        let txt_resp = match self.do_request(Method::GET, path, body).await {
            Ok(resp) => resp,
            Err(error) => return Err(error)
        };

        let resp: UserQueryResp = match serde_json::from_str(&txt_resp) {
            Ok(users) => users,
            Err(error) => panic!("failed to decode json: {:?}", error),
        };

        Ok(resp.data)
    }
}

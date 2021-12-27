use std::env;
use std::collections::HashMap;

mod twitter;

pub use crate::twitter::client::Client;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let twitter_token = match env::var("TWITTER_API_TOKEN") {
        Err(error) => panic!("TWITTER_API_TOKEN env var not set: {:?}", error),
        Ok(token) => token,
    };
    let client = Client::new(twitter_token);

    let mut body = HashMap::new();
    body.insert(String::from("usernames"), String::from("gauthier_jolly,gjolly,thibmeu"));

    match client.get(String::from("/users/by"), body) {
        Err(error) => panic!("{:?}", error),
        Ok(resp) => println!("{:?}", resp),
    };

    Ok(())
}

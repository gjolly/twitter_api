use std::env;

mod twitter;

pub use crate::twitter::client::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let twitter_token = match env::var("TWITTER_API_TOKEN") {
        Ok(token) => token,
        Err(error) => {
            println!("TWITTER_API_TOKEN not set!");
            return Err(error.into())
        }
    };
    let client = Client::new(twitter_token);

    let usernames = Vec::from(&args[1..]);
    let users = client.find_users_by_username(usernames).await?;

    for user in users {
        println!("{:?}", user);
    }

    Ok(())
}

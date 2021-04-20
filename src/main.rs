use serenity::async_trait;
use serenity::client::{Client, EventHandler};
use serenity::framework::standard::{macros::group, StandardFramework};

use std::fs::File;
use std::io::{BufRead, BufReader};

mod commands;

use commands::math::*;
//const token_file: &File = &File::open("token.txt").unwrap();
//const TOKEN: &str = "Test";
//BufReader::new(file).lines().next()

#[group]
#[commands(multiply, add)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    let token_file = File::open("token.txt").unwrap();
    println!("{:?}", BufReader::new(token_file).lines().next());
    const TOKEN: &str = "test";

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("~")) // set the bot's prefix to "~"
        .group(&GENERAL_GROUP);

    // Login with a bot token from the environment
    let mut client = Client::builder(TOKEN)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

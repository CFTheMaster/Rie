#![allow(non_snake_case)]

#[macro_use]
extern crate diesel;

extern crate sys_info;

mod command_handler;
pub mod commands;
mod DatabaseWrapper;
mod utils;
mod ReadImage;
extern crate regex;
extern crate eventual;

use tracing_subscriber::{
    FmtSubscriber,
    EnvFilter,
};

use commands::{
    help::*,
};

use serenity::{
    
    client::bridge::gateway::ShardManager,
    framework::{
        StandardFramework,
        standard::{
            CommandResult,
            macros::hook,
        },
    },
    http::Http,
    model::channel::Message,
    prelude::*,
};

use std::{
    collections::HashSet,
    sync::Arc,
    time::Duration
};

use tokio::time::delay_for;

use dotenv::dotenv;
use std::env;

pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

#[hook]
async fn after(_ctx: &Context, _msg: &Message, command_name: &str, command_result: CommandResult){
    let width = 4;
    let discrim = format!("{:0width$}", _msg.author.discriminator, width = width);
    if _msg.is_private() {
        match command_result {
            Ok(()) => println!("Log: Command \x1b[31m'{}'\x1b[0m executed by \x1b[35m'{}#{}' ({})\x1b[0m in DMs\x1b[0m", command_name, _msg.author.name, discrim, _msg.author.id),
            Err(why) => println!("ERROR: An error ocurred on the command '{}'.\n{:?}", command_name, why)
        }
    } 
    else {
        match command_result {
            Ok(()) => println!("Log: Command \x1b[31m'{}'\x1b[0m executed by \x1b[35m'{}#{}' ({})\x1b[0m in guild: \x1b[32m'{}' ({}).\x1b[0m", command_name, _msg.author.name, discrim, _msg.author.id, _msg.guild(&_ctx.cache).await.unwrap().name,  _msg.guild(&_ctx.cache).await.unwrap().id),
            Err(why) => println!("ERROR: An error ocurred on the command '{}'.\n{:?}", command_name, why)
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let _token = DatabaseWrapper::Database::getToken();

    let prefix: String = env::var("PREFIX")
                .to_owned()
                .expect("no prefix found in .env");

    let strPrefix: &str = &prefix;

    // println!("the current token: {}", _token);

    let subscriber = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("Failed to start the logger");

    let http = Http::new_with_token(&_token);

    // We will fetch your bot's owners and id
    let (owners, _bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            (owners, info.id)
        },
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    let framework = StandardFramework::new()
        .configure(|c| c
            .with_whitespace(true)
            .on_mention(Some(_bot_id))
            .owners(owners)
            .prefix(strPrefix)
            .case_insensitivity(true))
        .group(&command_handler::GENERAL_GROUP)
        .group(&command_handler::IMAGES_GROUP)
        .help(&HELP)
        .after(after);

    let mut client = Client::builder(_token)
        .event_handler(command_handler::Handler)
        .framework(framework)
        .await
        .expect("Error creating client");
    
    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
    }

    let shard_manager = client.shard_manager.clone();

    tokio::spawn(async move {
        loop {
            delay_for(Duration::from_secs(30)).await;

            let lock = shard_manager.lock().await;
            let shard_runners = lock.runners.lock().await;

            for (id, runner) in shard_runners.iter() {
                let _ = runner.stage;
                let _ = runner.latency;
                let _ = id;
                /*println!(
                    "Shard ID {} is {} with a latency of {:?}",
                    id,
                    runner.stage,
                    runner.latency,
                );*/
            }
        }
    });
    
    

    if let Err(why) = client.start_autosharded().await {
        println!("Client error: {:?}", why);
    }
}

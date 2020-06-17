mod command_handler;
pub mod commands;

use commands::{
    help::*,
};

use serenity::{
    client::bridge::gateway::ShardManager,
    prelude::*,
    framework::{
        StandardFramework,
        standard::macros::group,
    },
    model::id::UserId,
};
use typemap::Key;



use std::{
    thread,
    time::Duration,
    sync::{ Arc },
    io::{ Read },
    collections::{ HashSet },
};

use serde::{Deserialize};

#[derive(Default, Deserialize, Clone)]
pub struct Settings { 
    pub discord_token: String
}

impl Key for Settings { 
    type Value = Arc<Mutex<Settings>>;
}

fn init_settings() -> Settings {
    let mut f = std::fs::File::open("./tokens/config.toml").expect("Could not find the config.toml file. Please copy config.toml.example to config.toml and edit the resulting file");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("Could not read configuration file");
    toml::from_str(&contents).expect("Could not deserialize configuration")
}

struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}


fn main() {
    let prefix: &'static str = "rie.";

    let settings = init_settings();

    let mut client = Client::new(&settings.discord_token, command_handler::Handler).expect("Err creating client");

    
    {
        let mut data = client.data.write();
        data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
    }

    
    let manager = client.shard_manager.clone();

    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_secs(30));

            let lock = manager.lock();
            let shard_runners = lock.runners.lock();

            for (id, runner) in shard_runners.iter() {
                println!(
                    "Shard ID {} is {} with a latency of {:?}",
                    id,
                    runner.stage,
                    runner.latency,
                );
            }
        }
    });
    let (owners, bot_id) = match client.cache_and_http.http.get_current_application_info() {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            println!("Owners: {}, Bot ID: {}", info.owner.id, info.id);
            (owners, info.id)
            
        },
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    //TO-DO
    client.with_framework(StandardFramework::new()
        .configure(|c| c
            .with_whitespace(true)
            .on_mention(Some(bot_id))
            .owners(owners)
            .prefix(prefix))
        .group(&command_handler::GENERAL_GROUP)
        .group(&command_handler::MODERATION_GROUP)
        .help(&HELP));
    

    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }
}

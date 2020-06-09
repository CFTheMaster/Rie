mod CommandHandler;
mod command;

use serenity::{
    prelude::*,
    framework::StandardFramework
};
use typemap::Key;

use std::{
    sync::{ Arc, Mutex },
    io::{ Error, ErrorKind, Read },
    collections::{ HashSet, HashMap },
    iter::FromIterator, str::FromStr, process::Command,
};

use serde::{Serialize, Deserialize};

use crate::command::{GENERAL_GROUP}

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

fn main() {
    let settings = init_settings();

    let mut client = Client::new(&settings.discord_token, CommandHandler::Handler).expect("Err creating client");

    client.with_framework(
        StandardFramework::new()
            .configure(|c| c.prefix("rie."))
            .group(&GENERAL_GROUP),
    );

    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }
}

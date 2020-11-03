use rand::{self, Rng}; 
use dotenv::dotenv;
use std::env;

use serenity::{
    model::{gateway::Ready},
    prelude::*,
    framework::standard::{
        macros::{
            group
        }
    }
};

use crate::commands::general::{
    me::*,
    ping::*,
    owner::*,
    invite::*,
};

use crate::commands::moderation::{
    ban::*,
    unban::*,
};

use crate::commands::images::{
    anime::*,
    hentai::*,
    dva::*,
    neko::*,
    nsfwneko::*,
    trap::*,
    yuri::*,
};

#[group]
#[commands(me, ping, quit, invite)]
pub struct General;

#[group]
#[commands(ban, unban)]
pub struct Moderation;

#[group]
#[commands(anime, hentai, dva, neko, nsfwneko, trap, yuri)]
pub struct Images;

pub struct Handler;
impl EventHandler for Handler {
    fn ready(&self, ctx: Context, data: Ready){
        dotenv().ok();

        let pressence = env::var("PRESENCE_SETTER")
            .expect("no pressence found in .env");

        let split: Vec<&str> = pressence.split("__").collect();
        if let Some(shard) = data.shard {
            use serenity::model::gateway::Activity;
            use serenity::model::user::OnlineStatus;

           let rng = rand::thread_rng().gen_range(0, 2);

            let status_dnd = OnlineStatus::DoNotDisturb;
            let status = OnlineStatus::Online;
            let status_afk = OnlineStatus::Idle;

            let randomText = split[rand::thread_rng().gen_range(0, split.len())];


            // Make this into an array (WIP)
            match rng{
                0 => ctx.set_presence(Some(Activity::playing(randomText)), status_dnd),
                1 => ctx.set_presence(Some(Activity::playing(randomText)), status),
                2 => ctx.set_presence(Some(Activity::playing(randomText)), status_afk),
                _ => ctx.set_presence(Some(Activity::playing("If you see this presence the bot has errored")), status_dnd),
            };
            
            // Note that array index 0 is 0-indexed, while index 1 is 1-indexed.
            //
            // This may seem unintuitive, but it models Discord's behaviour.
            println!(
                "{}#{} is connected on shard {}/{} and {} guild(s)",
                data.user.name,
                data.user.discriminator,
                shard[0],
                shard[1],
                data.guilds.capacity()
            );

            
        }
    }
    
}
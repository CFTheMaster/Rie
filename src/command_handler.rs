use rand::{self, Rng}; 
use dotenv::dotenv;
use std::env;

use tokio::time::{timeout, Duration};

use serenity::{
    async_trait,
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
#[commands(anime, hentai, dva, neko, nsfwneko, trap, yuri)]
pub struct Images;

async fn statusChanger(ctx: Context){
    dotenv().ok();

    use serenity::model::gateway::Activity;
    use serenity::model::user::OnlineStatus;

    let status_dnd = OnlineStatus::DoNotDisturb;
    let status = OnlineStatus::Online;
    let status_afk = OnlineStatus::Idle;
    let rng = rand::thread_rng().gen_range(0, 2);

    

    let pressence = env::var("PRESENCE_SETTER")
        .expect("no pressence found in .env");

    let split: Vec<&str> = pressence.split("__").collect();

    let randomText = split[rand::thread_rng().gen_range(0, split.len())];

    // Make this into an array (WIP)
    match rng{
        0 => ctx.set_presence(Some(Activity::playing(randomText)), status_dnd).await,
        1 => ctx.set_presence(Some(Activity::playing(randomText)), status).await,
        2 => ctx.set_presence(Some(Activity::playing(randomText)), status_afk).await,
        _ => ctx.set_presence(Some(Activity::playing("If you see this presence the bot has errored")), status_dnd).await,
    };
            
}

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, data: Ready){
        
        if let Some(shard) = data.shard {
            
            let _res = timeout(Duration::from_secs(600), statusChanger(ctx));
            
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
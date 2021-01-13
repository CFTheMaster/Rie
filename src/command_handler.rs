use rand::{self, Rng}; 
use dotenv::dotenv;
use std::env;

use tracing::{info};

use serenity::{
    async_trait,
    model::{gateway::Ready, event::ResumedEvent},
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
    system::*,
};

use crate::commands::images::{
    anime::*,
    baguette::*,
    hentai::*,
    dva::*,
    neko::*,
    nsfwneko::*,
    trap::*,
    yuri::*,
};

use crate::DatabaseWrapper::Database;
use eventual::Timer;


#[group]
#[commands(me, ping, quit, invite, system)]
pub struct General;

#[group]
#[commands(anime, baguette, hentai, dva, neko, nsfwneko, trap, yuri)]
pub struct Images;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        if let Some(shard) = ready.shard {
            dotenv().ok();

            // Note that array index 0 is 0-indexed, while index 1 is 1-indexed.
            //
            // This may seem unintuitive, but it models Discord's behaviour.

            println!("{}#{} is connected on shard {}/{} and {} guild(s)\n",
                ready.user.name,
                ready.user.discriminator,
                shard[0],
                shard[1],
                ready.guilds.capacity()
            );

            Database::basicChecker();

            use serenity::model::gateway::Activity;
            use serenity::model::user::OnlineStatus;

            let status_dnd = OnlineStatus::DoNotDisturb;
            let status = OnlineStatus::Online;
            let status_afk = OnlineStatus::Idle;

            let pressence = env::var("PRESENCE_SETTER")
                        .expect("no pressence found in .env");

            let split: Vec<&str> = pressence.split("__").collect();

            let randomText = split[rand::thread_rng().gen_range(0..=split.len())];

            

            let rng = rand::thread_rng().gen_range(0..=2);

            match rng{
                0 => ctx.set_presence(Some(Activity::playing(randomText)), status_dnd).await,
                1 => ctx.set_presence(Some(Activity::playing(randomText)), status).await,
                2 => ctx.set_presence(Some(Activity::playing(randomText)), status_afk).await,
                _ => ctx.set_presence(Some(Activity::playing("If you see this presence the bot has errored")), status_dnd).await,
            };

            changeActivity(ctx).await;
        }
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {
        info!("Resumed");
    }
    
}

async fn changeActivity(ctx: Context){
    let timer = Timer::new();
    let ticks = timer.interval_ms(3600000).iter();

    for _ in ticks {        
        use serenity::model::gateway::Activity;
        use serenity::model::user::OnlineStatus;

        let status_dnd = OnlineStatus::DoNotDisturb;
        let status = OnlineStatus::Online;
        let status_afk = OnlineStatus::Idle;

        let pressence = env::var("PRESENCE_SETTER")
                    .expect("no pressence found in .env");

        let split: Vec<&str> = pressence.split("__").collect();

        let randomText = split[rand::thread_rng().gen_range(0..=split.len())];

        

        let rng = rand::thread_rng().gen_range(0..=2);

        match rng{
            0 => ctx.set_presence(Some(Activity::playing(randomText)), status_dnd).await,
            1 => ctx.set_presence(Some(Activity::playing(randomText)), status).await,
            2 => ctx.set_presence(Some(Activity::playing(randomText)), status_afk).await,
            _ => ctx.set_presence(Some(Activity::playing("If you see this presence the bot has errored")), status_dnd).await,
        };
    }
}
use rand::{Rng};

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
        if let Some(shard) = data.shard {
            use serenity::model::gateway::Activity;
            use serenity::model::user::OnlineStatus;

            let rng = rand::thread_rng().gen_range(0, 11);

            let status_dnd = OnlineStatus::DoNotDisturb;
            let status = OnlineStatus::Online;
            let status_afk = OnlineStatus::Idle;

            // Make this into an array (WIP)
            match rng{
                0 => ctx.set_presence(Some(Activity::playing("I'm an airplane!")), status_dnd),
                1 => ctx.set_presence(Some(Activity::playing("Stop the memes!")), status),
                2 => ctx.set_presence(Some(Activity::playing("Why am I doing this?")), status_afk),
                3 => ctx.set_presence(Some(Activity::playing("Help me!")), status),
                4 => ctx.set_presence(Some(Activity::playing("Help me!")), status_dnd),
                5 => ctx.set_presence(Some(Activity::playing("Why am I doing this?")), status),
                6 => ctx.set_presence(Some(Activity::playing("Why am I doing this?")), status_dnd),
                7 => ctx.set_presence(Some(Activity::playing("Help me!")), status_afk),
                8 => ctx.set_presence(Some(Activity::playing("Stop the memes!")), status_dnd),
                9 => ctx.set_presence(Some(Activity::playing("Stop the memes!")), status_afk),
                10 => ctx.set_presence(Some(Activity::playing("I'm an airplane!")), status),
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

    //TODO

    /*fn message(&self, ctx: Context, message: Message){;

        let prefix: &'static str = "rie.";

        let mut ping_command = String::from(prefix);
        ping_command.push_str("ping");

        let mut simple_reply = String::from(prefix);
        simple_reply.push_str("me");

        let mut help_command = String::from(prefix);
        help_command.push_str("help");

        if !message.author.bot{

            /*if message.content == help_command {
                let response = MessageBuilder::new()
                    .push(
                        "```\nCommand List:\nhelp - Shows this message;\nme;\nping - Replies to the user, no ping measure yet.```"
                    )
                    .build();
                
                if let Err(why) = message.channel_id.say(&ctx.http, &response) {
                    println!("Error sending message: {:?}", why);
                }
            }

            if message.content == ping_command {
                let channel = match message.channel_id.to_channel(&ctx) {
                    Ok(channel) => channel,
                    Err(why) => {
                        println!("Error getting channel: {:?}", why);
                        return;
                    },
                };

                let response = MessageBuilder::new()
                    .push("User ")
                    .push_bold_safe(&message.author.name)
                    .push(" used the 'ping' command in the ")
                    .mention(&channel)
                    .push(" channel")
                    .build();

                if let Err(why) = message.channel_id.say(&ctx.http, &response) {
                    println!("Error sending message: {:?}", why);
                }
            }

            if message.content == simple_reply {
                if let Err(why) = message.reply(ctx.http, " welcome to hell") {
                    println!("Error sending message: {:?}", why);
                }
            }*/
        
        }
        
    }*/
}
use std::string::String;
use rand::{Rng};

use serenity::{
    model::{channel::Message, gateway::Ready},
    prelude::*,
    utils::MessageBuilder,
    framework::standard::{
        StandardFramework,
        CommandResult,
        macros::{
            command,
            group
        }
    }
};


pub struct Handler;
impl EventHandler for Handler {
    fn ready(&self, ctx: Context, data: Ready){
        if let Some(shard) = data.shard {
            use serenity::model::gateway::Activity;
            use serenity::model::user::OnlineStatus;

            let mut rng = rand::thread_rng().gen_range(0, 10);

            let status = OnlineStatus::DoNotDisturb;

            match rng{
                0 => ctx.set_presence(Some(Activity::playing("I'm an airplane!")), status),
                1 => ctx.set_presence(Some(Activity::playing("Stop the memes!")), status),
                2 => ctx.set_presence(Some(Activity::playing("Why am I doing this?")), status),
                3..=10 => ctx.set_presence(Some(Activity::playing("Help me!")), status),
                _ => ctx.set_presence(Some(Activity::playing("If you see this presence the bot has errored")), status),
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

    fn message(&self, ctx: Context, message: Message){
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
        
    }
}
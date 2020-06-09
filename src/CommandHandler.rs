use std::string::String;

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
        println!("Connected using : {}#{}", data.user.name, data.user.discriminator);
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
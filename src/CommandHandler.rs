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


        if message.content == ping_command && !message.author.bot {
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
        
    }
}
use serenity::framework::standard::{macros::*, Args, CommandResult};
use serenity::model::channel::Message;
use serenity::prelude::*;


#[command]
#[description = "get the commands from the bot"]
#[usage = "help"]
#[example = "help"]
fn help(ctx: &mut Context, message: &Message) -> CommandResult {
    let channel = match message.channel_id.to_channel(&ctx) {
        Ok(channel) => channel,
        Err(why) => {
            println!("Error getting channel: {:?}", why);
            return;
        },
    };

    let response = MessageBuilder::new()
        .push("```\nCommand List:\nhelp - Shows this message;\nme;\nping - Replies to the user, no ping measure yet.```")
        .build();

    if let Err(why) = message.channel_id.say(&ctx.http, &response) {
        println!("Error sending message: {:?}", why);
    }
}
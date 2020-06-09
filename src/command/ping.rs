use serenity::framework::standard::{macros::*, Args, CommandResult};
use serenity::model::channel::Message;
use serenity::prelude::*;


#[command]
#[description = "Replies to the user, no ping measure yet."]
#[usage = "ping"]
#[example = "ping"]
fn ping(ctx: &mut Context, message: &Message) -> CommandResult {
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
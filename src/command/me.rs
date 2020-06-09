use serenity::framework::standard::{macros::*, Args, CommandResult};
use serenity::model::channel::Message;
use serenity::prelude::*;


#[command]
#[description = "get a basic reply from the bot"]
#[usage = "me"]
#[example = "me"]
fn me(ctx: &mut Context, message: &Message) -> CommandResult {
    if let Err(why) = message.reply(ctx.http, " welcome to hell") {
        println!("Error sending message: {:?}", why);
    }
}
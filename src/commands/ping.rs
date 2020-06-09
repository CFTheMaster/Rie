use chrono::{offset::Utc, Duration};

use serenity::{
    client::{bridge::gateway::ShardId, Context},
    framework::standard::{macros::command, CommandResult},
    model::prelude::Message
};

#[command]
#[description = "Checks Discord's API / message latency."]
fn ping(ctx: &mut Context, message: &Message) -> CommandResult {
    let _ = message.channel_id.say(&ctx.http, "Pong!");
    println!("Processed command 'ping'");


    Ok(())



}
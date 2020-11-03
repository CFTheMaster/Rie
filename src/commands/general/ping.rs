use serenity::{
    client::{Context},
    framework::standard::{macros::command, CommandResult},
    model::prelude::Message,
    client::bridge::gateway::ShardId
};
use crate::ShardManagerContainer;

use std::process::Command;


#[command]
#[description = "Checks Discord's API / message latency."]
#[usage = "ping"]
#[example = "ping"]
async fn ping(ctx: &Context, message: &Message) -> CommandResult {
    // The shard manager is an interface for mutating, stopping, restarting, and
    // retrieving information about shards.
    let data = &ctx.data.read().await;

    let shard_manager = match data.get::<ShardManagerContainer>() {
        Some(v) => v,
        None => {
            message.reply(ctx, "There was a problem getting the shard manager").await?;

            return Ok(());
        },
    };

    let manager = shard_manager.lock().await;
    let runners = manager.runners.lock().await;

    let runner = match runners.get(&ShardId(ctx.shard_id)) {
        Some(runner) => runner,
        None => {
            message.reply(ctx,  "No shard found").await?;

            return Ok(());
        },
    };


    // Shards are backed by a "shard runner" responsible for processing events
    // over the shard, so we'll get the information about the shard runner for
    // the shard this command was sent over.

    let pinger = if cfg!(target_os = "windows") {
        Command::new("cmd")
        .args(&["/C","curl --output NUL --write-out %{time_connect} https://discord.com/api/v8/"])
        .output()
        .expect("failed to execute process")
    } else {
        Command::new("sh")
        .arg("-c")
        .arg("curl --output /dev/null --write-out %{time_connect} https://discord.com/api/v8/")
        .output()
        .expect("failed to execute process")
    };

    let mut pingponger = String::new();

    pingponger.push_str(&String::from_utf8(pinger.stdout).unwrap());
    let _trimmed = pingponger.trim_end_matches("000").trim_start_matches("0,").to_owned();

    message.reply(&ctx, &format!("The shard latency is {:?}, Websocket latency is {}ms", runner.latency, _trimmed)).await?;

    Ok(())

}
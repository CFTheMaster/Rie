use serenity::{
    client::{Context},
    framework::standard::{macros::command, CommandResult},
    model::prelude::Message,
    client::bridge::gateway::ShardId
};
use crate::ShardManagerContainer;

use tokio::process::Command;

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

    message.reply(&ctx, &format!("The shard latency is {:?}", runner.latency)).await?;

    Ok(())

}
use serenity::{
    client::{Context},
    framework::standard::{macros::command, CommandResult},
    model::prelude::Message,
    client::bridge::gateway::ShardId
};
use crate::ShardManagerContainer;


#[command]
#[description = "Checks Discord's API / message latency."]
#[usage = "ping"]
#[example = "ping"]
fn ping(ctx: &mut Context, message: &Message) -> CommandResult {
    // The shard manager is an interface for mutating, stopping, restarting, and
    // retrieving information about shards.
    let data = &ctx.data.read();

    let shard_manager = match data.get::<ShardManagerContainer>() {
        Some(v) => v,
        None => {
            let _ = message.reply(&ctx.http, "There was a problem getting the shard manager");

            return Ok(());
        },
    };

    let manager = shard_manager.lock();
    let runners = manager.runners.lock();

    // Shards are backed by a "shard runner" responsible for processing events
    // over the shard, so we'll get the information about the shard runner for
    // the shard this command was sent over.
    let runner = match runners.get(&ShardId(ctx.shard_id)) {
        Some(runner) => runner,
        None => {
            let _ = message.reply(&ctx,  "No shard found");

            return Ok(());
        },
    };


    let _ = message.reply(&ctx, &format!("The shard latency is {:?}", runner.latency));

    let g = message.guild(&ctx.cache).unwrap();
    let width = 4;
    let discrim = format!("{:0width$}", message.author.discriminator, width = width);

    println!("Processed command 'ping' by user '{}#{}' ({}) in guild '{}' ({}) ", message.author.name, discrim, message.author.id, &g.read().name, &g.read().id);

    Ok(())



}
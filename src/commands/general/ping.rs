use serenity::{
    client::{Context},
    framework::standard::{macros::command, CommandResult},
    model::prelude::Message
};

#[command]
#[description = "Checks Discord's API / message latency."]
#[usage = "ping"]
#[example = "ping"]
fn ping(ctx: &mut Context, message: &Message) -> CommandResult {
    let _ = message.channel_id.say(&ctx.http, "Pong!");

    let g = message.guild(&ctx.cache).unwrap();
    let width = 4;
    let discrim = format!("{:0width$}", message.author.discriminator, width = width);

    println!("Processed command 'ping' by user '{}#{}' ({}) in guild '{}' ({}) ", message.author.name, discrim, message.author.id, &g.read().name, &g.read().id);

    Ok(())



}
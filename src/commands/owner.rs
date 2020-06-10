use crate::ShardManagerContainer;
use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandResult,
    macros::command,
};
use std::string::String;

#[command]
#[owners_only]
fn quit(ctx: &mut Context, message: &Message) -> CommandResult {
    let data = ctx.data.read();

    if let Some(manager) = data.get::<ShardManagerContainer>() {
        message.channel_id.say(&ctx.http, "Shutting down!")?;

        let g = message.guild(&ctx.cache).unwrap();

        let width = 4;
        let discrim = format!("{:0width$}", message.author.discriminator, width = width);

        println!("Processed command 'owner' by user '{}#{}' ({}) in guild '{}' ({}) ", message.author.name, discrim, message.author.id, &g.read().name, &g.read().id);

        ctx.shard.shutdown_clean();
        
    } else {
        println!("There was a problem getting the shard manager.");
    }

    Ok(())
}
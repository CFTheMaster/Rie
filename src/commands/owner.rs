use crate::ShardManagerContainer;
use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandResult,
    macros::command,
};

#[command]
#[owners_only]
fn quit(ctx: &mut Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read();

    if let Some(manager) = data.get::<ShardManagerContainer>() {
        msg.channel_id.say(&ctx.http, "Shutting down!")?;

        ctx.shard.shutdown_clean();
    } else {
        println!("There was a problem getting the shard manager.");
    }

    Ok(())
}
use crate::ShardManagerContainer;
use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandResult,
    macros::command,
};


#[command]
#[description = "shutdown the current shard."]
#[usage = "quit"]
#[example = "quit"]
async fn quit(ctx: &Context, message: &Message) -> CommandResult {
    if message.author.id == 138302166619258880 || message.author.id == 214393232342122506 {
        let data = ctx.data.read().await;

        if let Some(manager) = data.get::<ShardManagerContainer>() {
            message.reply(&ctx, "Shutting down!").await?;
            manager.lock().await.shutdown_all().await;
        } else {
            message.reply(&ctx, "There was a problem getting the shard manager").await?;

            return Ok(());
        }
    } else {
        message.reply(&ctx, "Don't even try!").await?;
    }
    

    Ok(())
}
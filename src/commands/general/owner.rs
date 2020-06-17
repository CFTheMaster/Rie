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
fn quit(ctx: &mut Context, message: &Message) -> CommandResult {
    if message.author.id == 138302166619258880 || message.author.id == 214393232342122506 {
        let data = ctx.data.read();
        let g = message.guild(&ctx.cache).unwrap();
        let width = 4;
        let discrim = format!("{:0width$}", message.author.discriminator, width = width);


        if let Some(manager) = data.get::<ShardManagerContainer>() {
            let _ = message.reply(&ctx, "Shutting down!");
            println!("Processed command 'shutdown' by user '{}#{}' ({}) in guild '{}' ({}) ", message.author.name, discrim, message.author.id, &g.read().name, &g.read().id);
            manager.lock().shutdown_all();
        } else {
            let _ = message.reply(&ctx, "There was a problem getting the shard manager");

            return Ok(());
        }
    } else {
        let _ = message.reply(&ctx, "Don't even try!");
    }
    

    Ok(())
}
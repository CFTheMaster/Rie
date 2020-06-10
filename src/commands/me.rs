use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::{prelude::Message}
};


#[command]
fn me(ctx: &mut Context, message: &Message) -> CommandResult{
    let _ = message.reply(&ctx.http, " Welcome to hell");
    let g = message.guild(&ctx.cache).unwrap();
    println!("Processed command 'me' by user '{}#{}' ({}) in guild {} ({}) ", message.author.name, message.author.discriminator, message.author.id, &g.read().name, &g.read().id);

    Ok(()) 
}

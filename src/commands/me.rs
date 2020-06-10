use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::{prelude::Message}
};
use std::string::String;

#[command]
fn me(ctx: &mut Context, message: &Message) -> CommandResult{
    let _ = message.reply(&ctx.http, " Welcome to hell");

    let g = message.guild(&ctx.cache).unwrap();
    let width = 4;
    let discrim = format!("{:0width$}", message.author.discriminator, width = width);

    println!("Processed command 'me' by user '{}#{}' ({}) in guild '{}' ({}) ", message.author.name, discrim, message.author.id, &g.read().name, &g.read().id);

    Ok(()) 
}

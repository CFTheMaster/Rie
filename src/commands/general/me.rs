use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::{prelude::Message}
};

#[command]
#[description = "just get a basic response"]
#[usage = "me"]
#[example = "me"]
fn me(ctx: &mut Context, message: &Message) -> CommandResult{
    let _ = message.reply(&ctx.http, " Welcome to hell");

    Ok(()) 
}

use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::{prelude::Message}
};

#[command]
#[description = "just get a basic response"]
#[usage = "me"]
#[example = "me"]
async fn me(ctx: &Context, message: &Message) -> CommandResult{
    message.reply(&ctx.http, " Welcome to hell").await?;

    Ok(()) 
}

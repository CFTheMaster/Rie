use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::{prelude::Message}
};


#[command]
fn me(ctx: &mut Context, message: &Message) -> CommandResult{
    let _ = message.reply(&ctx.http, " Welcome to hell");
    println!("Processed command 'me'");

    Ok(()) 
}

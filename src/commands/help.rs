use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::{prelude::Message}
};


#[command]
fn help(ctx: &mut Context, message: &Message) -> CommandResult{
    let _ = message.channel_id.say(&ctx.http, "```\nCommand List:\nhelp - Shows this message;\nme;\nping - Replies to the user, no ping measure yet.```");
    println!("Processed command 'help'");

    Ok(())
}
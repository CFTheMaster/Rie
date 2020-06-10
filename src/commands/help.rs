use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::{prelude::Message}
};
use std::string::String;


#[command]
fn help(ctx: &mut Context, message: &Message) -> CommandResult{

    use serenity::model::guild::Guild;

    let _ = message.channel_id.say(&ctx.http, "```\nCommand List:\nhelp - Shows this message;\nme;\nping - Replies to the user, no ping measure yet.```");
    let g = message.guild(&ctx.cache).unwrap();
    
    let width = 4;
    let discrim = format!("{:0width$}", message.author.discriminator, width = width);

    println!("Processed command 'help' by user '{}#{}' ({}) in guild '{}' ({}) ", message.author.name, discrim, message.author.id, &g.read().name, &g.read().id);

    Ok(())
}